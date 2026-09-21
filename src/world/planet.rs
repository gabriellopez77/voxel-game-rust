use std::sync::Arc;

use crate::{
    math::{self, Vec3, Vec3i},
    render::ChunksRenderer,
    world::{
        Aabb,
        ChunksManager,
        blocks::{BlockIdState, BlockProperties, block_registry},
        chunk::{
            Chunk, ChunkGetter, NeighborsChunks,
            chunk_data::{ChunkDataFlags, ChunkDataReadBehavior},
        },
        light_engine,
        particles::{ParticlesManager, ParticlesSpawnArgs},
        player::Camera,
    }
};


pub struct BlockIteraterInfo<'a> {
    pub global_block: Vec3,
    pub chunk_block: Vec3i,
    pub chunk: Arc<Chunk>,
    pub block_properties: &'a BlockProperties,
}

pub struct Planet {
    pub chunks_manager: ChunksManager,

    pub render_distance: i32,

    blocks_aabb_list: Vec<Aabb>,
}

impl Planet {
    pub fn new() -> Self {
        Self {
            chunks_manager: ChunksManager::new(),

            render_distance: 4,

            blocks_aabb_list: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.chunks_manager.start();
        self.chunks_manager.set_render_distance(self.render_distance);
    }

    pub fn stop(&mut self) {
        self.chunks_manager.stop();
    }

    pub fn cleanup(&mut self, chunks_renderer: &mut ChunksRenderer) {
        self.chunks_manager.cleanup(chunks_renderer);
    }

    pub fn update(&mut self, player_pos: Vec3) {
        let player_chunk_pos = math::get_chunk_pos(player_pos);

        self.chunks_manager.update(player_chunk_pos);
    }

    pub fn draw(&mut self, dt: f32, camera: &Camera, chunks_renderer: &mut ChunksRenderer) {
        self.chunks_manager.dispose_chunks_renderers(chunks_renderer);

        while let Some(mesh_result) = chunks_renderer.get_generated_mesh() {
            if let Some(ch) = self.chunks_manager.get_chunk(mesh_result.chunk_pos) {
                ch.content.borrow_mut().renderer.update_mesh(&mesh_result, chunks_renderer);
            }

            chunks_renderer.restore_mesh_result(mesh_result);
        }

        self.chunks_manager.draw_chunks(dt, camera,chunks_renderer);
    }

    pub fn place_block(&self, chunk: &Chunk, chunk_block: Vec3i, id_state: BlockIdState) {
        let old_block = chunk.data.change_block(chunk_block, id_state);

        self.change_block_logic(chunk, chunk_block, &old_block, block_registry::get().get_properties(id_state));
    }

    pub fn destroy_block(&self, chunk: &Chunk, chunk_block: Vec3i, particles_manager: &mut ParticlesManager) {
        let old_block = chunk.data.change_block(chunk_block, BlockIdState::AIR);

        self.change_block_logic(chunk, chunk_block, &old_block, block_registry::get().get_properties(BlockIdState::AIR));

        particles_manager.spawn(ParticlesSpawnArgs::BlockDestroy(
            old_block.base_properties.get_id_state(),
            (chunk.position * Chunk::CHUNK_SIZE + chunk_block).as_vec3()
        ));
    }

    pub fn get_blocks_hitboxes(&mut self, aabb: &Aabb) -> &Vec<Aabb> {
        let mut aabb_list = std::mem::take(&mut self.blocks_aabb_list);

        aabb_list.clear();

        self.iterate_over_blocks_cube(aabb, |_, x, y, z, properties|
            if let Some(ref collision_box) = properties.collision_box {
                aabb_list.push(collision_box.clone_move(x as f32, y as f32, z as f32));
            }
        );

        self.blocks_aabb_list = aabb_list;

        return &self.blocks_aabb_list;
    }

    pub fn iterate_over_blocks_cube(&mut self,
        aabb: &Aabb,
        mut func: impl FnMut(
            &mut bool,
            i32, i32, i32,
            &BlockProperties
        )
    ) {
        let x0 = aabb.x0.floor() as i32;
        let y0 = aabb.y0.floor() as i32;
        let z0 = aabb.z0.floor() as i32;
        let x1 = (aabb.x1 + 1.0).floor() as i32;
        let y1 = (aabb.y1 + 1.0).floor() as i32;
        let z1 = (aabb.z1 + 1.0).floor() as i32;

        let mut chunk_getter = ChunkGetter::new();

        for x in x0..x1 {
        for y in y0..y1 {
        for z in z0..z1 {
            let global_coords = Vec3i::new(x, y, z).as_vec3();
            let chunk_pos = math::get_chunk_pos(global_coords);

            chunk_getter.change(chunk_pos, &self.chunks_manager);

            if let Some(ref ch) = chunk_getter.chunk {
                let chunk_block = math::get_chunk_block(chunk_pos, global_coords);

                let block_properties = ch.data.get_block_properties(chunk_block);

                let mut stop = false;
                func(&mut stop, x, y, z, block_properties);
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

            if let Some(chunk) = chunk_getter.change(chunk_pos, &self.chunks_manager) {
                let chunk_block = math::get_chunk_block(chunk_pos, block_pos);

                let block_properties = chunk.data.get_block_properties(chunk_block);

                let iterater_info = BlockIteraterInfo {
                    global_block: block_pos,
                    chunk_block,
                    chunk: chunk.clone(),
                    block_properties,
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

        self.chunks_manager.load_chunks(chunk_pos);
    }

    fn change_block_logic(&self,
        chunk: &Chunk,
        chunk_block: Vec3i,
        old_block: &BlockProperties,
        new_block: &BlockProperties
    ) {
        light_engine::update_light(self.chunks_manager.chunks.clone(), chunk.data.clone(), chunk_block, old_block, new_block);

        let neighbors = NeighborsChunks::new(&self.chunks_manager, chunk.position, false);

        // update around chunks to avoids visual glitchs
        if let Some(south) = neighbors.south && chunk_block.z == Chunk::CHUNK_SIZE_MINUS_ONE.z {
            south.data.turn_on_flag(ChunkDataFlags::REGEN_MESH_FLAG);
        }

        if let Some(north) = neighbors.north && chunk_block.z == 0 {
            north.data.turn_on_flag(ChunkDataFlags::REGEN_MESH_FLAG);
        }

        if let Some(west) = neighbors.west && chunk_block.x == 0 {
            west.data.turn_on_flag(ChunkDataFlags::REGEN_MESH_FLAG);
        }

        if let Some(east) = neighbors.east && chunk_block.x == Chunk::CHUNK_SIZE_MINUS_ONE.x {
            east.data.turn_on_flag(ChunkDataFlags::REGEN_MESH_FLAG);
        }
    }
}
