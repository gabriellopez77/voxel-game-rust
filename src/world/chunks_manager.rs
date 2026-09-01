use std::{cell::RefCell, collections::HashMap, sync::{Arc, Mutex, atomic::Ordering}};
use std::sync::RwLock;
use crate::{math::{self, Vec3i}, resources::{ThreadWorker, ThreadWorkerValue}, world::{Chunk, chunk::NeighborsChunksData, player::Camera}};
use crate::render::ChunksRenderer;
use crate::utils::{NullSafePtr, ObjectPool, SafePtr};
use crate::world::blocks::BlocksManager;
use crate::world::chunk::ChunkData;
use crate::world::light_engine;
use crate::world::world_gen::WorldGen;


pub struct ChunksManager {
    pub chunks: Arc<RwLock<HashMap<Vec3i, Option<Arc<RwLock<Chunk>>>>>>,
    world_gen: Arc<Mutex<WorldGen>>,
    blocks_manager: NullSafePtr<BlocksManager>,

    render_distance: i32,
    pendings_chunks_count: i32,

    dispose_chunks_renderers_list: Vec<Arc<RwLock<Chunk>>>,
    remove_chunks_list: Vec<Arc<RwLock<Chunk>>>,
    ordered_chunks: Vec<Arc<RwLock<Chunk>>>,

    last_player_chunk: Vec3i,
    update_change_chunk_logic: bool,
    need_ordering_chunks: bool,

    pub chunks_gen_worker: ThreadWorkerValue<Box<RwLock<Chunk>>>,
    pub chunks_background_worker: ThreadWorker,

    pub chunk_data_pool: ObjectPool<Arc<RwLock<ChunkData>>>,
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

    pub fn get_chunki(&self, x: i32, y: i32, z: i32) -> Option<Arc<RwLock<Chunk>>> {
        self.get_chunk(Vec3i::new(x, y, z))
    }

    pub fn get_chunk(&self, pos: Vec3i) -> Option<Arc<RwLock<Chunk>>> {
        if let Some(chunk) = self.chunks.read().unwrap().get(&pos) {
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
        for (_, chunk) in &mut *self.chunks.write().unwrap() {
            if let Some(chunk) = chunk {
                chunk.write().unwrap().renderer.dispose(chunks_renderer);
            }
        }

        self.chunks.write().unwrap().clear();

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

    pub fn draw_chunks(&self, dt: f32, camera: &Camera, chunks_renderer: &mut ChunksRenderer) {
        for ch in &self.ordered_chunks {
            ch.write().unwrap().draw(dt, self, camera, chunks_renderer);
        }
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
                let ch1_distance = math::get_chunk_distance(ch1.read().unwrap().position, player_chunk_pos);
                let ch2_distance = math::get_chunk_distance(ch2.read().unwrap().position, player_chunk_pos);

                return ch1_distance.cmp(&ch2_distance);
            });
        }
    }

    pub fn dispose_chunks_renderers(&mut self, chunks_renderer: &mut ChunksRenderer) {
        for ch in &self.dispose_chunks_renderers_list {
            ch.write().unwrap().renderer.dispose(chunks_renderer);
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

        // add distant chunks to remove list
        for (pos, ch) in &*self.chunks.read().unwrap() {
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
            let ch_borrow = ch.read().unwrap();
            self.chunks.write().unwrap().remove(&ch_borrow.position);
            self.chunk_data_pool.restore(ch_borrow.data.clone());

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

            if distance > self.render_distance || self.chunks.read().unwrap().contains_key(&new_chunk_pos) {
                continue
            }

            // SAFETY: blocks_manager reference is valid for all game time
            let blocks_manager = self.blocks_manager.clone();

            let world_gen = self.world_gen.clone();
            let new_chunk_data = self.chunk_data_pool.get();


            // create chunk async
            self.chunks_gen_worker.add_task(move || {
                // resets chunk data to avoid corrupted values
                if let Some(ref chunk_data) = new_chunk_data {
                    chunk_data.write().unwrap().clear(new_chunk_pos);
                }

                let mut new_chunk = Chunk::new(new_chunk_pos, new_chunk_data, SafePtr::from_ptr(blocks_manager.get_raw()));
                new_chunk.start(&mut world_gen.lock().unwrap(), &blocks_manager);

                //let now = std::time::Instant::now();
                light_engine::init_chunk_light(new_chunk.data.clone());
                //println!("{}", now.elapsed().as_micros());

                return Box::new(RwLock::new(new_chunk));
            });

            self.pendings_chunks_count += 1;
            self.chunks.write().unwrap().insert(new_chunk_pos, None);
        }
        }
    }

    fn process_chunks_gen(&mut self,) {
        self.chunks_gen_worker.process_tasks();

        while let Some(chunk_result) = self.chunks_gen_worker.get_finalized_task() {
            let chunk_pos = chunk_result.read().unwrap().position;
            let chunk_arc: Arc<RwLock<Chunk>> = Arc::from(chunk_result);

            self.need_ordering_chunks = true;
            self.ordered_chunks.push(chunk_arc.clone());
            self.pendings_chunks_count -= 1;

            // fix visual glitch
            let neighbors_data = NeighborsChunksData::new(self, chunk_pos, false);
            self.regen_neighbor_chunks(&neighbors_data);

            *self.chunks.write().unwrap().get_mut(&chunk_pos).unwrap() = Some(chunk_arc.clone());

            //let now = std::time::Instant::now();
            //light_engine::update_light_in_border_neighbors(self.chunks.clone(), chunk_arc.read().unwrap().data.clone(), neighbors_data);
            //println!("{}", now.elapsed().as_micros());


            // self is valid for all game life
            //let chunks_manager_ptr = SafePtr::new(self);

            let chunk_data = chunk_arc.read().unwrap().data.clone();

            let chunks_map = self.chunks.clone();
            self.chunks_background_worker.add_task(move || {
                light_engine::update_light_in_border_neighbors(chunks_map, chunk_data, neighbors_data);
            });
        }
    }

    fn regen_neighbor_chunks(&self, neighbors_data: &NeighborsChunksData) {
        if let Some(ref north) = neighbors_data.north { north.read().unwrap().regen_mesh.store(true, Ordering::Relaxed); }
        if let Some(ref south) = neighbors_data.south { south.read().unwrap().regen_mesh.store(true, Ordering::Relaxed); }
        if let Some(ref west) = neighbors_data.west { west.read().unwrap().regen_mesh.store(true, Ordering::Relaxed); }
        if let Some(ref east) = neighbors_data.east { east.read().unwrap().regen_mesh.store(true, Ordering::Relaxed); }
    }
}
