use std::{cell::RefCell, rc::Rc};
use std::sync::atomic::AtomicI32;
use crate::math;
use crate::math::{Vec2, Vec3, Vec3i};
use crate::render::{BlockModelMesh, ChunkRenderer, ChunkVertices, Shader, Texture};
use crate::resources::BlockItemModel;
use crate::utils::ObjectPool;
use crate::world::blocks::{BlocksManager, blocks_manager};
use crate::world::{Planet, WorldGen};
use crate::world::chunk::{ChunkData, NeighborChunks};
use crate::world::player::Camera;


#[repr(i32)]
#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    North,
    South,
    West,
    East
}

pub struct Chunk {
    pub position: Vec3i,

    pub chunk_data: ChunkData,

    mesh_generated: bool,
    renderer: ChunkRenderer,
    inside_frustum: bool,

    using_count: AtomicI32,
}

impl Chunk {
    pub const CHUNK_SIZE: Vec3i = Vec3i { x: 16, y: 128, z: 16 };
    pub const CHUNK_SIZE_MINUS_ONE: Vec3i = Vec3i { x: 15, y: 127, z: 15 };
    pub const CHUNK_SIZEF: Vec3 = Vec3 { x: 16.0, y: 128.0, z: 16.0 };
    pub const CHUNK_DATA_SIZE: usize = (Self::CHUNK_SIZE.x * Self::CHUNK_SIZE.y * Self::CHUNK_SIZE.z) as usize;
    pub const REGION_SIZE: i32 = 16;

    pub fn new(position: Vec3i, shader: Rc<RefCell<Shader>>, texture: Rc<Texture>) -> Self {
        Self {
            position,

            chunk_data: ChunkData::new(),

            mesh_generated: false,
            renderer: ChunkRenderer::new(position, shader, texture),
            inside_frustum: false,

            using_count: AtomicI32::new(0)
        }
    }

    pub fn recreate(&mut self, position: Vec3i, shader: Rc<RefCell<Shader>>, texture: Rc<Texture>) {
        self.position = position;

        self.chunk_data.get_data_mut().fill(0);

        self.mesh_generated = false;
        self.renderer.recreate(position, shader, texture);
        self.inside_frustum = false;

        self.using_count.store(0, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn start(&mut self, world_gen: &WorldGen, blocks_manager: &BlocksManager) {
        world_gen.gen_data(self.position, &mut self.chunk_data, blocks_manager);
    }

    pub fn draw(&mut self, camera: &Camera, planet: &Planet, blocks_manager: &BlocksManager,
                vertices_pool: &mut ObjectPool<Vec<ChunkVertices>>, indices_pool: &mut ObjectPool<Vec<u32>>) {
        if camera.view_changed {
            self.inside_frustum = camera.chunk_inside_frustum(self.position)
        }

        if !self.inside_frustum { return }

        if !self.mesh_generated {
            self.chunk_data.regen_mesh = true;
            self.mesh_generated = true;
        }

        if self.chunk_data.regen_mesh {
            let neighbor_chunks = NeighborChunks::new_set(planet, self.position);

            let (mut vertices, mut indices) = self.gen_mesh(
                &neighbor_chunks,
                blocks_manager,
                vertices_pool,
                indices_pool
            );

            self.renderer.update_mesh(&vertices, &indices);

            vertices.clear();
            indices.clear();

            vertices_pool.insert(vertices);
            indices_pool.insert(indices);

            self.chunk_data.regen_mesh = false;
        }
        self.renderer.draw();
    }

    pub fn erase(&mut self) {
        self.renderer.erase();
    }

    pub fn lock(&self) { self.using_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst); }
    pub fn unlock(&self) { self.using_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst); }

    pub fn gen_mesh(&self, neighbor_chunks: &NeighborChunks, blocks_manager: &BlocksManager,
                    vertices_pool: &mut ObjectPool<Vec<ChunkVertices>>,
                    indices_pool: &mut ObjectPool<Vec<u32>>) -> (Vec<ChunkVertices>, Vec<u32>) {

        fn add_face(vertices: &mut Vec<ChunkVertices>, chunk_block: Vec3, model_vertices: &Vec<BlockModelMesh>) {
            for i in (0..model_vertices.len()).step_by(4) {
                let vert1 = &model_vertices[i + 0];
                let vert2 = &model_vertices[i + 1];
                let vert3 = &model_vertices[i + 2];
                let vert4 = &model_vertices[i + 3];

                vertices.push(ChunkVertices { vertices: vert1.vertices + chunk_block, normal: vert1.normal, uv: vert1.uv });
                vertices.push(ChunkVertices { vertices: vert2.vertices + chunk_block, normal: vert2.normal, uv: vert2.uv });
                vertices.push(ChunkVertices { vertices: vert3.vertices + chunk_block, normal: vert3.normal, uv: vert3.uv });
                vertices.push(ChunkVertices { vertices: vert4.vertices + chunk_block, normal: vert4.normal, uv: vert4.uv });
            }
        }

        let mut vertices = match vertices_pool.get() {
            Some(x) => x,
            None => Vec::new()
        };


        for x in 0..Chunk::CHUNK_SIZE.x {
        for y in 0..Chunk::CHUNK_SIZE.y {
        for z in 0..Chunk::CHUNK_SIZE.z {
            let block_id = self.chunk_data.get_blocki(x, y, z);

            // air does not have model
            if block_id == 0 { continue; }

            let chunk_block = Vec3{x: x as f32, y: y as f32, z: z as f32};
            let mut draw = false;

            let model = blocks_manager.get(block_id).get_base().get_model();

            // add nothing faces
            add_face(&mut vertices, chunk_block, &model.nothing_vertices);

            if y < Chunk::CHUNK_SIZE_MINUS_ONE.y && self.chunk_data.get_blocki(x, y + 1, z) == 0 { draw = true }
            else if y == Chunk::CHUNK_SIZE_MINUS_ONE.y { draw = true }

            if draw { add_face(&mut vertices, chunk_block, &model.up_vertices); }
            draw = false;


            if y > 0  && self.chunk_data.get_blocki(x, y - 1, z) == 0 { draw = true }

            if draw { add_face(&mut vertices, chunk_block, &model.down_vertices); }
            draw = false;


            if z < Chunk::CHUNK_SIZE_MINUS_ONE.z {
                if self.chunk_data.get_blocki(x, y, z + 1) == 0 { draw = true }
            }
            else if neighbor_chunks.south.exists() {
                if neighbor_chunks.south.get().borrow().chunk_data.get_blocki(x, y, 0) == 0 { draw = true }
            }

            if draw { add_face(&mut vertices, chunk_block, &model.south_vertices); }
            draw = false;


            if z > 0 {
                if self.chunk_data.get_blocki(x, y, z - 1) == 0 { draw = true }
            }
            else if neighbor_chunks.north.exists() {
                if neighbor_chunks.north.get().borrow().chunk_data.get_blocki(x, y, Self::CHUNK_SIZE_MINUS_ONE.z) == 0 {draw = true}
            }

            if draw { add_face(&mut vertices, chunk_block, &model.north_vertices); }
            draw = false;


            // east
            if x < Chunk::CHUNK_SIZE_MINUS_ONE.x {
                if self.chunk_data.get_blocki(x + 1, y, z) == 0 { draw = true }
            }
            else if neighbor_chunks.east.exists() {
                if neighbor_chunks.east.get().borrow().chunk_data.get_blocki(0, y, z) == 0 { draw = true }
            }

            if draw { add_face(&mut vertices, chunk_block, &model.east_vertices); }
            draw = false;


            // west
            if x > 0 {
                if self.chunk_data.get_blocki(x - 1, y, z) == 0 { draw = true }
            }
            else if neighbor_chunks.west.exists() {
                if neighbor_chunks.west.get().borrow().chunk_data.get_blocki(Self::CHUNK_SIZE_MINUS_ONE.x, y, z) == 0 { draw = true }
            }

            if draw { add_face(&mut vertices, chunk_block, &model.west_vertices); }
        }
        }
        }

        let indices_count = vertices.len() / 4;
        let mut indices = match indices_pool.get() {
            Some(x) => x,
            None => Vec::with_capacity(indices_count * 6)
        };

        let mut current_index: u32 = 0;

        for _ in 0..indices_count {
            indices.push(current_index + 0);
            indices.push(current_index + 1);
            indices.push(current_index + 3);

            indices.push(current_index + 1);
            indices.push(current_index + 2);
            indices.push(current_index + 3);

            current_index += 4;
        }

        return (vertices, indices);
    }
}
