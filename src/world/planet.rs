use std::sync::{Mutex, RwLock};
use std::{cell::RefCell, collections::HashMap, sync::Arc};

use crate::math::{Vec3, Vec3i, self};

use crate::render::{ChunkRenderer, ChunkVertices, GlobalRenderer};
use crate::resources::Worker;
use crate::utils::{NullSafePtr, ObjectPool, SafePtr};
use crate::world::Aabb;
use crate::world::blocks::{BlockProperties, BlocksManager};
use crate::world::chunk::{ChunkData, ChunkGetter, ChunkMeshResult, NeighborChunks};
use crate::world::world_gen::WorldGen;
use crate::world::{Chunk, player::Camera};


pub struct BlockIteraterInfo {
    pub global_block: Vec3i,
    pub chunk_block: Vec3i,
    pub chunk: Arc<RefCell<Chunk>>,
    pub block_properties: SafePtr<BlockProperties>,
    pub blocks_manager: SafePtr<BlocksManager>,
}

pub struct Planet {
    chunks: HashMap<Vec3i, Option<Arc<RefCell<Chunk>>>>,
    world_gen: Arc<Mutex<WorldGen>>,

    pub blocks_manager: NullSafePtr<BlocksManager>,

    pub render_distance: i32,

    pub pendings_chunks_count: i32,

    last_player_chunk: Vec3i,
    change_chunk_logic: bool,
    need_ordering_chunks: bool,

    remove_chunks_list: Vec<Arc<RefCell<Chunk>>>,
    ordered_chunks: Vec<Arc<RefCell<Chunk>>>,
    visible_chunks: Vec<Arc<RefCell<Chunk>>>,

    pub chunk_mesh_vertices_pool: ObjectPool<Vec<ChunkVertices>>,
    pub chunk_mesh_indices_pool: ObjectPool<Vec<u32>>,
    pub chunk_data_pool: ObjectPool<Arc<RwLock<ChunkData>>>,

    blocks_aabb_list: Vec<Aabb>,

    chunks_mesh_worker: Worker<Box<RefCell<ChunkMeshResult>>>,
    chunks_gen_worker: Worker<Box<RefCell<Chunk>>>,
}

impl Planet {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            world_gen: Arc::new(Mutex::new(WorldGen::new())),

            blocks_manager: NullSafePtr::null(),

            render_distance: 14,

            pendings_chunks_count: 0,

            last_player_chunk: Vec3i::ZERO,
            change_chunk_logic: true,
            need_ordering_chunks: false,

            remove_chunks_list: Vec::new(),
            ordered_chunks: Vec::new(),
            visible_chunks: Vec::new(),

            chunk_mesh_vertices_pool: ObjectPool::new(),
            chunk_mesh_indices_pool: ObjectPool::new(),
            chunk_data_pool: ObjectPool::new(),

            blocks_aabb_list: Vec::new(),

            chunks_mesh_worker: Worker::new(),
            chunks_gen_worker: Worker::new(),
        }
    }

    pub fn start(&mut self, blocks_manager: &BlocksManager) {
        self.blocks_manager = NullSafePtr::new(blocks_manager);

        self.chunks_mesh_worker.start();
        self.chunks_gen_worker.start();

        self.world_gen.lock().unwrap().start(blocks_manager);
    }

    pub fn stop(&mut self) {
        self.chunks_mesh_worker.stop();
        self.chunks_gen_worker.stop();
    }

    pub fn cleanup(&mut self) {
        for (_, chunk) in &mut self.chunks {
            if let Some(chunk) = chunk {
                chunk.borrow_mut().erase();
            }
        }

        self.chunks.clear();

        self.chunk_mesh_indices_pool.clear();
        self.chunk_mesh_vertices_pool.clear();
        self.chunk_data_pool.clear();
        self.ordered_chunks.clear();
        self.remove_chunks_list.clear();
        self.visible_chunks.clear();

        self.chunks_mesh_worker.clear();
        self.chunks_gen_worker.clear();
    }

    pub fn update(&mut self, player_pos: Vec3) {
        let player_chunk = math::get_chunk_pos(player_pos);

        if self.last_player_chunk != player_chunk || self.change_chunk_logic {
        //if self.change_chunk_logic {
            self.change_chunk_logic(player_chunk);
        }

        // sort chunks
        if self.need_ordering_chunks {
            self.need_ordering_chunks = false;

            self.ordered_chunks.sort_by(|ch1, ch2| {
                let ch1_distance = math::get_chunk_distance(ch1.borrow().position, self.last_player_chunk);
                let ch2_distance = math::get_chunk_distance(ch2.borrow().position, self.last_player_chunk);

                return ch1_distance.cmp(&ch2_distance);
            });
        }

        self.process_chunks_gen();
        self.process_chunks_mesh();
    }

    pub fn draw(&mut self, dt: f32, camera: &Camera, global_renderer: &mut GlobalRenderer) {
        self.visible_chunks.clear();

        for i in 0..self.ordered_chunks.len() {
            let chunk_arc = self.ordered_chunks[i].clone();
            let mut ch = chunk_arc.borrow_mut();

            if camera.view_changed {
                ch.inside_frustum = camera.chunk_inside_frustum(ch.visual_position);
            }

            if !ch.inside_frustum {
                continue;
            }

            self.visible_chunks.push(chunk_arc.clone());

            if ch.renderer.is_none() {
                ch.renderer = Some(ChunkRenderer::new(global_renderer));
                ch.chunk_data.write().unwrap().regen_mesh = true;
            }

            if ch.chunk_data.read().unwrap().regen_mesh {
                ch.chunk_data.write().unwrap().regen_mesh = false;

                // SAFETY: blocks_manager reference is valid for all game time
                let blocks_manager_ptr = self.blocks_manager.clone();

                let mesh_result = Box::new(RefCell::new(ChunkMeshResult::new(self, &ch)));

                // create chunk mesh async
                self.chunks_mesh_worker.add_task(move || {
                    let blocks_manager = blocks_manager_ptr.clone();

                    Chunk::gen_mesh(&mut mesh_result.borrow_mut(), &*blocks_manager);
                    mesh_result.borrow_mut().gen_indices();

                    return mesh_result;
                });
            }
        }

        for ch in &self.visible_chunks {
            ch.borrow_mut().draw(dt, global_renderer);
        }
    }

    pub fn get_blocks_hitboxes(&mut self, aabb: &Aabb) -> &Vec<Aabb> {
        self.blocks_aabb_list.clear();

        self.iterate_over_blocks_cube(aabb, |_, planet, _, x, y, z, properties|
            if let Some(ref collision_box) = properties.collision_box {
                planet.blocks_aabb_list.push(collision_box.clone_move(x as f32, y as f32, z as f32));
            }
        );

        return &self.blocks_aabb_list;
    }

    pub fn iterate_over_blocks_cube(&mut self,
        aabb: &Aabb,
        mut func: impl FnMut(&mut bool, &mut Planet, SafePtr<BlocksManager>, i32, i32, i32, SafePtr<BlockProperties>)
    ) {
        let x0 = aabb.x0.floor() as i32;
        let y0 = aabb.y0.floor() as i32;
        let z0 = aabb.z0.floor() as i32;
        let x1 = (aabb.x1 + 1.0).floor() as i32;
        let y1 = (aabb.y1 + 1.0).floor() as i32;
        let z1 = (aabb.z1 + 1.0).floor() as i32;

        let mut chunk_getter = ChunkGetter::new();
        let blocks_manager = SafePtr::from_ptr(self.blocks_manager.get_raw());
        
        for x in x0..x1 {
        for y in y0..y1 {
        for z in z0..z1 {
            let global_coords = Vec3i::new(x, y, z).as_vec3();
            let chunk_pos = math::get_chunk_pos(global_coords);

            if let Some(ch) = chunk_getter.change(chunk_pos, self) {
                let chunk_block = math::get_chunk_block(chunk_pos, global_coords);

                let block_info = ch.borrow().chunk_data.read().unwrap().get_block_info(chunk_block);
                let block_properties = self.blocks_manager.get_properties_from_block_info(block_info);

                let mut stop = false;
                func(&mut stop, self, blocks_manager.clone(), x, y, z, block_properties.clone());
                if stop { return }

            }
        }
        }
        }
    }

    // gemini code with some adjustments
    pub fn iterate_over_blocks_raycast(&self,
        ray_origin: Vec3,
        ray_dir: Vec3,
        ray_length: f32,
        mut func: impl FnMut(&mut bool, &BlockIteraterInfo)
    ) {
        let mut chunk_getter = ChunkGetter::new();
        let blocks_manager = SafePtr::from_ptr(self.blocks_manager.get_raw());
        
        // 1. Initialize current voxel coordinate
        let mut block_pos = Vec3::new(
            ray_origin.x.floor(),
            ray_origin.y.floor(),
            ray_origin.z.floor(),
        );

        // 2. Determine step direction per axis (+1 or -1)
        let step_x = if ray_dir.x >= 0.0 { 1.0 } else { -1.0 };
        let step_y = if ray_dir.y >= 0.0 { 1.0 } else { -1.0 };
        let step_z = if ray_dir.z >= 0.0 { 1.0 } else { -1.0 };

        // 3. Compute how far along the ray (t) we must travel to cross one full voxel width
        // Use f32::INFINITY to safety-guard against division by zero
        let t_delta_x = if ray_dir.x != 0.0 { (1.0 / ray_dir.x).abs() } else { f32::INFINITY };
        let t_delta_y = if ray_dir.y != 0.0 { (1.0 / ray_dir.y).abs() } else { f32::INFINITY };
        let t_delta_z = if ray_dir.z != 0.0 { (1.0 / ray_dir.z).abs() } else { f32::INFINITY };

        // 4. Compute starting t values to reach the first voxel boundaries
        let mut t_max_x = if ray_dir.x > 0.0 {
            (block_pos.x + 1.0 - ray_origin.x) * t_delta_x
        } else if ray_dir.x < 0.0 {
            (ray_origin.x - block_pos.x) * t_delta_x
        } else { f32::INFINITY };

        let mut t_max_y = if ray_dir.y > 0.0 {
            (block_pos.y + 1.0 - ray_origin.y) * t_delta_y
        } else if ray_dir.y < 0.0 {
            (ray_origin.y - block_pos.y) * t_delta_y
        } else { f32::INFINITY };

        let mut t_max_z = if ray_dir.z > 0.0 {
            (block_pos.z + 1.0 - ray_origin.z) * t_delta_z
        } else if ray_dir.z < 0.0 {
            (ray_origin.z - block_pos.z) * t_delta_z
        } else { f32::INFINITY };

        // 5. Walk the grid
        loop {
            let chunk_pos = math::get_chunk_pos(block_pos);

            if let Some(chunk) = chunk_getter.change(chunk_pos, self) {
                let chunk_block = math::get_chunk_block(chunk_pos, block_pos);

                let block_info = chunk.borrow().chunk_data.write().unwrap().get_block_info(chunk_block);
                let block_properties = self.blocks_manager.get_properties_from_block_info(block_info);

                let iterater_info = BlockIteraterInfo {
                    global_block: block_pos.as_vec3i(),
                    chunk_block,
                    chunk: chunk.clone(),
                    block_properties,
                    blocks_manager: blocks_manager.clone(),
                };

                let mut stop = false;
                func(&mut stop, &iterater_info);
                if stop { return }
            }

            // Advance to next boundary by picking the smallest t_max
            if t_max_x < t_max_y {
                if t_max_x < t_max_z {
                    if t_max_x > ray_length { break }
                    block_pos.x += step_x;
                    t_max_x += t_delta_x;
                }
                else {
                    if t_max_z > ray_length { break }
                    block_pos.z += step_z;
                    t_max_z += t_delta_z;
                }
            }
            else {
                if t_max_y < t_max_z {
                    if t_max_y > ray_length { break }
                    block_pos.y += step_y;
                    t_max_y += t_delta_y;
                }
                else {
                    if t_max_z > ray_length { break }
                    block_pos.z += step_z;
                    t_max_z += t_delta_z;
                }
            }
        }
    }

    pub fn load_chunks(&mut self, player_pos: Vec3) {
        let chunk_pos = math::get_chunk_pos(player_pos);

        self.change_chunk_logic(chunk_pos);
    }

    pub fn get_chunk(&self, pos: Vec3i) -> Option<Arc<RefCell<Chunk>>> {
        if let Some(chunk) = self.chunks.get(&pos) {
            return chunk.clone();
        }

        return None;
    }

    pub fn get_chunk_int(&self, x: i32, y: i32, z: i32) -> Option<Arc<RefCell<Chunk>>> {
        self.get_chunk(Vec3i::new(x, y, z))
    }

    pub fn process_chunks_gen(&mut self) {
        self.chunks_gen_worker.process_tasks();

        while let Some(chunk_result) = self.chunks_gen_worker.get_finalized_task() {
            let chunk_pos = chunk_result.borrow().position;
            let chunk_arc: Arc<RefCell<Chunk>> = Arc::from(chunk_result);

            self.need_ordering_chunks = true;
            self.ordered_chunks.push(chunk_arc.clone());
            self.pendings_chunks_count -= 1;

            *self.chunks.get_mut(&chunk_pos).unwrap() = Some(chunk_arc);

            // fix visual glitch
            let neighbor_chunks = NeighborChunks::new_set(self, chunk_pos, false);
            self.regen_neighbor_chunks(&neighbor_chunks);
        }
    }

    fn process_chunks_mesh(&mut self) {
        self.chunks_mesh_worker.process_tasks();

        while let Some(mesh_result) = self.chunks_mesh_worker.get_finalized_task() {
            if let Some(ch) = self.get_chunk(mesh_result.borrow().chunk_pos) &&
                let Some(ref mut ch_renderer) = ch.borrow_mut().renderer {
                ch_renderer.update_mesh(&mesh_result.borrow());
            }

            mesh_result.into_inner().restore(self);
        }
    }

    fn change_chunk_logic(&mut self, player_chunk_pos: Vec3i) {
        self.ordered_chunks.clear();
        self.remove_chunks_list.clear();

        self.change_chunk_logic = false;
        self.need_ordering_chunks = true;

        // remove chunks so far
        for (pos, ch) in &self.chunks {
            if let Some(ch) = ch {
                let distance = math::get_chunk_distance(player_chunk_pos, *pos);

                if distance > self.render_distance {
                    self.remove_chunks_list.push(ch.clone());
                    continue;
                }

                self.ordered_chunks.push(ch.clone());
            }
        }

        for ch in &self.remove_chunks_list {
            let mut ch_borrow = ch.borrow_mut();

            ch_borrow.erase();
            self.chunk_data_pool.restore(ch_borrow.chunk_data.clone());
            self.chunks.remove(&ch_borrow.position);
        }

        self.remove_chunks_list.clear();


        let start = player_chunk_pos - self.render_distance;
        let end = player_chunk_pos + self.render_distance;

        // create new chunks
        for x in start.x..=end.x {
        for z in start.z..=end.z {
            let new_chunk_pos = Vec3i::new(x, 0, z);

            let distance = math::get_chunk_distance(new_chunk_pos, player_chunk_pos);

            if distance > self.render_distance || self.chunks.contains_key(&new_chunk_pos) {
                continue
            }

            // SAFETY: blocks_manager reference is valid for all game time
            let blocks_manager_ptr = self.blocks_manager.clone();

            let world_gen = self.world_gen.clone();
            let new_chunk_data = self.chunk_data_pool.get();


            // create chunk async
            self.chunks_gen_worker.add_task(move || {
                // resets chunk data to avoid corrupted values
                if let Some(ref chunk_data) = new_chunk_data {
                    chunk_data.write().unwrap().clear(new_chunk_pos);
                }

                let new_chunk = Chunk::new(new_chunk_pos, new_chunk_data, SafePtr::from_ptr(blocks_manager_ptr.get_raw()));
                let new_chunk = Box::new(RefCell::new(new_chunk));
                new_chunk.borrow_mut().start(&mut world_gen.lock().unwrap(), &*blocks_manager_ptr);

                return new_chunk;
            });

            self.pendings_chunks_count += 1;
            self.chunks.insert(new_chunk_pos, None);
        }
        }

        self.last_player_chunk = player_chunk_pos;
    }

    fn regen_neighbor_chunks(&self, neighbors: &NeighborChunks) {
        if let Some(ref north) = neighbors.north { north.borrow_mut().chunk_data.write().unwrap().regen_mesh = true }
        if let Some(ref south) = neighbors.south { south.borrow_mut().chunk_data.write().unwrap().regen_mesh = true }
        if let Some(ref west) = neighbors.west { west.borrow_mut().chunk_data.write().unwrap().regen_mesh = true }
        if let Some(ref east) = neighbors.east { east.borrow_mut().chunk_data.write().unwrap().regen_mesh = true }
    }
}
