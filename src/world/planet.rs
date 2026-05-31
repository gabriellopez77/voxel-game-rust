use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use crate::math::{Vec3, Vec3i, self};

use crate::render::chunk_renderer::RendererType;
use crate::render::{ChunkVertices, Shader, Texture, render_utils};
use crate::resources::{BlockItemModel, ResourceManager};
use crate::utils::ObjectPool;
use crate::world::blocks::BlocksManager;
use crate::world::chunk::{ChunkGetter, ChunkMeshResult, NeighborChunks, neighbor_chunks};
use crate::world::{Chunk, WorldGen, player::Camera};


pub struct Planet {
    chunks: HashMap<Vec3i, Arc<RefCell<Chunk>>>,
    world_gen: WorldGen,

    pub render_distance: i32,

    last_player_chunk: Vec3i,
    change_chunk_logic: bool,

    remove_chunks_list: Vec<Arc<RefCell<Chunk>>>,
    ordered_chunks: Vec<Arc<RefCell<Chunk>>>,
    visible_chunks: Vec<Arc<RefCell<Chunk>>>,

    shader: Option<Rc<RefCell<Shader>>>,
    texture: Option<Rc<Texture>>,

    chunk_pool: ObjectPool<Arc<RefCell<Chunk>>>,
    chunk_mesh_vertices_pool: ObjectPool<Vec<ChunkVertices>>,
    chunk_mesh_indices_pool: ObjectPool<Vec<u32>>,
}

impl Planet {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            world_gen: WorldGen::new(),

            render_distance: 4,

            last_player_chunk: Vec3i::ZERO,
            change_chunk_logic: true,

            remove_chunks_list: Vec::new(),
            ordered_chunks: Vec::new(),
            visible_chunks: Vec::new(),

            shader: None,
            texture: None,

            chunk_pool: ObjectPool::new(),
            chunk_mesh_vertices_pool: ObjectPool::new(),
            chunk_mesh_indices_pool: ObjectPool::new(),
        }
    }

    pub fn start(&mut self, resource_manager: Rc<RefCell<ResourceManager>>) {
        self.shader = Some(resource_manager.borrow().get_shader("chunk").expect("shader not exists"));
        self.texture = Some(resource_manager.borrow().get_texture("blocks").expect("texture not exists"));
    }

    pub fn update(&mut self, player_pos: Vec3, blocks_manager: &BlocksManager) {
        let player_chunk = math::get_chunk_pos(player_pos);

        if self.last_player_chunk != player_chunk || self.change_chunk_logic {
        //if self.change_chunk_logic {
            self.ordered_chunks.clear();
            self.remove_chunks_list.clear();

            self.change_chunk_logic = false;

            // remove chunks so far
            for (pos, ch) in &self.chunks {
                let distance = math::get_chunk_distance(player_chunk, *pos);

                if distance >= self.render_distance {
                    self.remove_chunks_list.push(ch.clone());
                    continue;
                }

                self.ordered_chunks.push(ch.clone());
            }

            for ch in &self.remove_chunks_list {
                ch.borrow_mut().erase();
                self.chunks.remove(&ch.borrow_mut().position);
                self.chunk_pool.insert(ch.clone());
            }

            self.remove_chunks_list.clear();


            let start = player_chunk - self.render_distance;
            let end = player_chunk + self.render_distance;

            for x in start.x..=end.x {
                for z in start.z..=end.z {
                    let new_chunk_pos = Vec3i::new(x, 0, z);

                    let distance = math::get_chunk_distance(new_chunk_pos, player_chunk);

                    if distance >= self.render_distance || self.chunks.contains_key(&new_chunk_pos) { continue; }

                    let neighbor_chunks = NeighborChunks::new_set(self, new_chunk_pos, false);
                    self.regen_neighbor_chunks(&neighbor_chunks);


                    let new_chunk = match self.chunk_pool.get() {
                        Some(ch) => {
                            ch.borrow_mut().recreate(
                                new_chunk_pos,
                                self.shader.as_ref().unwrap().clone(),
                                self.texture.as_ref().unwrap().clone()
                            );

                            ch
                        }
                        None => {
                            Arc::new(RefCell::new(Chunk::new(
                                new_chunk_pos,
                                self.shader.as_ref().unwrap().clone(),
                                self.texture.as_ref().unwrap().clone()
                            )))
                        }

                    };

                    new_chunk.borrow_mut().start(&mut self.world_gen, blocks_manager);

                    self.chunks.insert(new_chunk_pos, new_chunk.clone());
                    self.ordered_chunks.push(new_chunk.clone());
                }
            }

            // sort chunks
            self.ordered_chunks.sort_by(|ch1, ch2| {
                let ch1_distance = math::get_chunk_distance(ch1.borrow().position, player_chunk);
                let ch2_distance = math::get_chunk_distance(ch2.borrow().position, player_chunk);

                return ch1_distance.cmp(&ch2_distance);
            });

        }

        self.last_player_chunk = player_chunk;
    }

    pub fn draw(&mut self, camera: &Camera, blocks_manager: &BlocksManager) {
        self.visible_chunks.clear();

        for chunk in &self.ordered_chunks {
            let mut ch = chunk.borrow_mut();

            if camera.view_changed {
                ch.inside_frustum = camera.chunk_inside_frustum(ch.position)
            }

            if !ch.inside_frustum { continue }

            if !ch.mesh_generated {
                ch.chunk_data.regen_mesh = true;
                ch.mesh_generated = true;
            }

            if ch.chunk_data.regen_mesh {
                let neighbor_chunks = NeighborChunks::new_set(self, ch.position, true);

                let mut mesh_result = ChunkMeshResult::new(
                    &mut self.chunk_mesh_vertices_pool,
                    &mut self.chunk_mesh_indices_pool
                );

                //let now = std::time::Instant::now();

                Chunk::gen_mesh(&*ch, &neighbor_chunks, blocks_manager, &mut mesh_result);
                //println!("{}", now.elapsed().as_micros());

                ch.renderer.update_mesh(&mesh_result);

                ch.chunk_data.regen_mesh = false;

                mesh_result.restore(&mut self.chunk_mesh_vertices_pool, &mut self.chunk_mesh_indices_pool);
            }

            self.visible_chunks.push(chunk.clone());
        }


        render_utils::disable(render_utils::RenderCap::Blend);
        for ch in &self.visible_chunks {
            ch.borrow().draw(RendererType::Opaque);
        }


        render_utils::enable(render_utils::RenderCap::Blend);
        for ch in &self.visible_chunks {
            ch.borrow().draw(RendererType::Alpha);
        }

        render_utils::disable(render_utils::RenderCap::Blend);


    }

    pub fn get_chunk(&self, pos: Vec3i) -> Option<Arc<RefCell<Chunk>>> {
        if let Some(chunk) = self.chunks.get(&pos) {
            return Some(chunk.clone());
        }

        return None;
    }

    pub fn get_chunk_int(&self, x: i32, y: i32, z: i32) -> Option<Arc<RefCell<Chunk>>> { self.get_chunk(Vec3i::new(x, y, z))}

    fn regen_neighbor_chunks(&self, neighbor_chunks: &NeighborChunks) {
        if let Some(ref north) = neighbor_chunks.north { north.borrow_mut().chunk_data.regen_mesh = true }
        if let Some(ref south) = neighbor_chunks.south { south.borrow_mut().chunk_data.regen_mesh = true }
        if let Some(ref west) = neighbor_chunks.west { west.borrow_mut().chunk_data.regen_mesh = true }
        if let Some(ref east) = neighbor_chunks.east { east.borrow_mut().chunk_data.regen_mesh = true }
    }
}
