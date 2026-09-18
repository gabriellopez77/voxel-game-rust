use std::{collections::HashMap, sync::{Arc, Mutex}};
use parking_lot::RwLock;

use crate::{
    math::{self, Vec3i},
    render::ChunksRenderer,
    utils::{NullSafePtr, ObjectPool, SafePtr},
    resources::{ThreadWorker, ThreadWorkerValue},
    world::{
        chunk::{
            Chunk,
            NeighborsChunksData,
            chunk_data::{ChunkData, ChunkDataFlags},
        },
        world_gen::WorldGen,
        light_engine,
        blocks::BlocksManager,
        player::Camera,
    }
};


pub struct ChunksManager {
    pub chunks: Arc<RwLock<HashMap<Vec3i, Option<Arc<Chunk>>>>>,
    world_gen: Arc<Mutex<WorldGen>>,
    blocks_manager: NullSafePtr<BlocksManager>,

    render_distance: i32,
    pendings_chunks_count: i32,

    dispose_chunks_renderers_list: Vec<Arc<Chunk>>,
    remove_chunks_list: Vec<Arc<Chunk>>,
    ordered_chunks: Vec<Arc<Chunk>>,

    last_player_chunk: Vec3i,
    update_change_chunk_logic: bool,
    need_ordering_chunks: bool,

    pub chunks_gen_worker: ThreadWorkerValue<Box<Chunk>, 4>,
    pub chunks_background_worker: ThreadWorker<1>,

    pub chunk_data_pool: ObjectPool<Arc<ChunkData>>,
}

impl ChunksManager {
    pub fn new() -> Self {
        Self {
            chunks: Arc::new(RwLock::new(HashMap::new())),
            world_gen: Arc::new(Mutex::new(WorldGen::new())),
            blocks_manager: NullSafePtr::null(),

            render_distance: 0,
            pendings_chunks_count: 0,

            dispose_chunks_renderers_list: Vec::new(),
            remove_chunks_list: Vec::new(),
            ordered_chunks: Vec::new(),

            last_player_chunk: Vec3i::ZERO,
            update_change_chunk_logic: true,
            need_ordering_chunks: true,

            chunks_gen_worker: ThreadWorkerValue::new(),
            chunks_background_worker: ThreadWorker::new(),

            chunk_data_pool: ObjectPool::new(),
        }
    }

    pub fn get_pendings_chunks_count(&self) -> i32 { self.pendings_chunks_count }

    pub fn get_chunki(&self, x: i32, y: i32, z: i32) -> Option<Arc<Chunk>> {
        self.get_chunk(Vec3i::new(x, y, z))
    }

    pub fn get_chunk(&self, pos: Vec3i) -> Option<Arc<Chunk>> {
        if let Some(chunk) = self.chunks.read().get(&pos) {
            return chunk.clone();
        }

        return None;
    }

    pub fn start(&mut self, blocks_manager: &BlocksManager) {
        self.blocks_manager = NullSafePtr::new(blocks_manager);

        self.world_gen.lock().unwrap().start(blocks_manager);

        self.chunks_gen_worker.start();
        self.chunks_background_worker.start();
    }

    pub fn cleanup(&mut self, chunks_renderer: &mut ChunksRenderer) {
        for (_, chunk) in &mut *self.chunks.write() {
            if let Some(chunk) = chunk {
                chunk.content.borrow_mut().renderer.dispose(chunks_renderer);
            }
        }

        self.chunks.write().clear();

        self.ordered_chunks.clear();
        self.remove_chunks_list.clear();

        self.chunk_data_pool.clear();

        self.chunks_gen_worker.clear();
        self.chunks_background_worker.clear();
    }

    pub fn stop(&mut self) {
        self.chunks_gen_worker.stop();
        self.chunks_background_worker.stop();
    }

    pub fn set_render_distance(&mut self, value: i32) {
        self.render_distance = value;
        self.update_change_chunk_logic = true;
    }

    pub fn update(&mut self, player_chunk_pos: Vec3i) {
        if self.last_player_chunk != player_chunk_pos || self.update_change_chunk_logic {
        //if self.update_change_chunk_logic {
            self.change_chunk_logic(player_chunk_pos);
        }

        self.process_chunks_gen();


        // sort chunks
        if self.need_ordering_chunks {
            self.need_ordering_chunks = false;

            self.ordered_chunks.sort_by(|ch1, ch2| {
                let ch1_distance = math::get_chunk_distance(ch1.position, player_chunk_pos);
                let ch2_distance = math::get_chunk_distance(ch2.position, player_chunk_pos);

                return ch1_distance.cmp(&ch2_distance);
            });
        }
    }

    pub fn draw_chunks(&self, dt: f32, camera: &Camera, chunks_renderer: &mut ChunksRenderer) {
        for ch in &self.ordered_chunks {
            ch.draw(self.chunks.clone(), camera, chunks_renderer, dt);
        }
    }

    pub fn dispose_chunks_renderers(&mut self, chunks_renderer: &mut ChunksRenderer) {
        for ch in &self.dispose_chunks_renderers_list {
            ch.content.borrow_mut().renderer.dispose(chunks_renderer);
        }

        self.dispose_chunks_renderers_list.clear();
    }

    pub fn load_chunks(&mut self, player_chunk_pos: Vec3i) {
        self.change_chunk_logic(player_chunk_pos);
    }

    pub fn process_load_chunks(&mut self) {
        self.process_chunks_gen();
    }

    fn change_chunk_logic(&mut self, player_chunk_pos: Vec3i) {
        self.ordered_chunks.clear();

        self.last_player_chunk = player_chunk_pos;
        self.update_change_chunk_logic = false;
        self.need_ordering_chunks = true;

        let mut chunks = self.chunks.write();

        // add distant chunks to remove list
        for (pos, ch) in &*chunks {
            if let Some(ch) = ch {
                let distance = math::get_chunk_distance(player_chunk_pos, *pos);

                if distance > self.render_distance {
                    self.remove_chunks_list.push(ch.clone());
                    continue;
                }

                self.ordered_chunks.push(ch.clone());
            }
        }

        // remove chunk from chunks and add to dispose_chunks_render_list
        for ch in &self.remove_chunks_list {
            chunks.remove(&ch.position);
            self.chunk_data_pool.restore(ch.data.clone());

            self.dispose_chunks_renderers_list.push(ch.clone());
        }

        self.remove_chunks_list.clear();


        let start = player_chunk_pos - self.render_distance;
        let end = player_chunk_pos + self.render_distance;

        // create new chunks
        for x in start.x..=end.x {
        for z in start.z..=end.z {
            let new_chunk_pos = Vec3i::new(x, 0, z);

            let distance = math::get_chunk_distance(new_chunk_pos, player_chunk_pos);

            if distance > self.render_distance || chunks.contains_key(&new_chunk_pos) {
                continue
            }

            // SAFETY: blocks_manager reference is valid for all game time
            let blocks_manager = self.blocks_manager.clone();

            let world_gen = self.world_gen.clone();
            let new_chunk_data = self.chunk_data_pool.get_from_fn(|value| {
                if let Some(data) = Arc::get_mut(value) {
                    *data = ChunkData::new(new_chunk_pos, SafePtr::from_ptr(NullSafePtr::get_raw(&blocks_manager)));

                    return true;
                }

                return false;
            });


            // create chunk async
            self.chunks_gen_worker.add_task(move || {
                // resets chunk data to avoid corrupted values
                //if let Some(ref chunk_data) = new_chunk_data {
                //    chunk_data.clear(new_chunk_pos);
                //}

                let mut new_chunk = Chunk::new(
                    new_chunk_pos,
                    new_chunk_data,
                    SafePtr::from_ptr(NullSafePtr::get_raw(&blocks_manager))
                );
                new_chunk.start(&mut world_gen.lock().unwrap(), &blocks_manager);

                //let now = std::time::Instant::now();
                light_engine::compute_light_value(new_chunk.data.clone());
                //println!("{}", now.elapsed().as_micros());

                return Box::new(new_chunk);
            });

            self.pendings_chunks_count += 1;
            chunks.insert(new_chunk_pos, None);
        }
        }
    }

    fn process_chunks_gen(&mut self) {
        self.chunks_gen_worker.process_tasks();

        while let Some(chunk_result) = self.chunks_gen_worker.get_finalized_task() {
            let chunk_pos = chunk_result.position;
            let chunk_arc: Arc<Chunk> = Arc::from(chunk_result);

            self.need_ordering_chunks = true;
            self.ordered_chunks.push(chunk_arc.clone());
            self.pendings_chunks_count -= 1;

            // fix visual glitch
            let neighbors_data = NeighborsChunksData::new(self, chunk_pos, false);
            self.regen_neighbor_chunks(&neighbors_data);

            *self.chunks.write().get_mut(&chunk_pos).unwrap() = Some(chunk_arc.clone());

            let chunk_data = chunk_arc.data.clone();
            let chunks_map = self.chunks.clone();

            self.chunks_background_worker.add_task(move || {
                //let now = std::time::Instant::now();
                light_engine::update_light_in_border_neighbors(chunks_map, chunk_data, neighbors_data);
                //println!("{}", now.elapsed().as_micros());
            });
        }
    }

    fn regen_neighbor_chunks(&self, neighbors_data: &NeighborsChunksData) {
        if let Some(ref north) = neighbors_data.north { north.turn_on_flag(ChunkDataFlags::REGEN_MESH_FLAG); }
        if let Some(ref south) = neighbors_data.south { south.turn_on_flag(ChunkDataFlags::REGEN_MESH_FLAG); }
        if let Some(ref west) = neighbors_data.west { west.turn_on_flag(ChunkDataFlags::REGEN_MESH_FLAG); }
        if let Some(ref east) = neighbors_data.east { east.turn_on_flag(ChunkDataFlags::REGEN_MESH_FLAG); }
    }
}
