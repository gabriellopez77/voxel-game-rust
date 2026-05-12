use std::cell::RefCell;
use std::rc::Rc;
use crate::math;
use crate::math::{Vec2, Vec3, Vec3i};
use crate::render::{ChunkRenderer, ChunkVertices, Shader, Texture};

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

    pub chunk_data: [u16; Chunk::CHUNK_DATA_SIZE],
    pub renderer: ChunkRenderer
}

impl Chunk {
    pub const CHUNK_SIZE: Vec3i = Vec3i { x: 16, y: 128, z: 16 };
    pub const CHUNK_SIZE_MINUS_ONE: Vec3i = Vec3i { x: 15, y: 127, z: 15 };
    pub const CHUNK_SIZEF: Vec3 = Vec3 { x: 16.0, y: 128.0, z: 16.0 };
    pub const CHUNK_DATA_SIZE: usize = (Chunk::CHUNK_SIZE.x * Chunk::CHUNK_SIZE.y * Chunk::CHUNK_SIZE.z) as usize;

    pub fn new(position: Vec3i, shader: Rc<RefCell<Shader>>) -> Self {
        Self {
            position,
            chunk_data: [0; Chunk::CHUNK_DATA_SIZE],
            renderer: ChunkRenderer::new(position, shader.clone())
        }
    }

    pub fn gen_mesh(&self) -> (Vec<ChunkVertices>, Vec<u32>) {
        let mut vertices: Vec<ChunkVertices> = vec!();

        for x in 0..Chunk::CHUNK_SIZE.x {
        for y in 0..Chunk::CHUNK_SIZE.y {
        for z in 0..Chunk::CHUNK_SIZE.z {
            let current_block =  math::get_index(x, y, z);

            if self.chunk_data[current_block] == 0 { continue; }
            let chunk_block = Vec3{x: x as f32, y: y as f32, z: z as f32};
            let mut draw = false;

            if y < Chunk::CHUNK_SIZE_MINUS_ONE.y && self.get_block(x, y + 1, z) == 0 { draw = true }
            else if y == Chunk::CHUNK_SIZE_MINUS_ONE.y { draw = true }
            
            if draw { Self::add_face(Direction::Up, chunk_block, &mut vertices) }
            draw = false;
            
            
            if y > 0  && self.get_block(x, y - 1, z) == 0 { draw = true }
            else if y == 0 { draw = true }

            if draw { Self::add_face(Direction::Down, chunk_block, &mut vertices) }
            draw = false;


            if z < Chunk::CHUNK_SIZE_MINUS_ONE.z && self.get_block(x, y, z + 1) == 0 { draw = true }
            else if z == Self::CHUNK_SIZE_MINUS_ONE.z { draw = true }
            
            if draw { Self::add_face(Direction::South, chunk_block, &mut vertices) }
            draw = false;
            
            
            if z > 0  && self.get_block(x, y, z - 1) == 0 { draw = true }
            else if z == 0 { draw = true }

            if draw { Self::add_face(Direction::North, chunk_block, &mut vertices) }
            draw = false;

            if x < Chunk::CHUNK_SIZE_MINUS_ONE.x  && self.get_block(x + 1, y, z) == 0 { draw = true }
            else if x == Chunk::CHUNK_SIZE_MINUS_ONE.x { draw = true }
            
            if draw { Self::add_face(Direction::East, chunk_block, &mut vertices) }
            draw = false;
            
            
            if x > 0  && self.get_block(x - 1, y, z) == 0 { draw = true }
            else if x == 0 { draw = true }
            
            if draw { Self::add_face(Direction::West, chunk_block, &mut vertices) }
        }
        }
        }

        let indices_count = vertices.len() / 4;
        let mut indices: Vec<u32> = Vec::with_capacity(indices_count * 6);
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

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> u16 {
        return self.chunk_data[math::get_index(x, y, z)];
    }
    fn add_face(dir: Direction, block: Vec3, vertices: &mut Vec<ChunkVertices>) {
        match dir {
            Direction::Up => {
                vertices.push(ChunkVertices{vertices: Vec3::new(0.0, 1.0, 1.0) + block, normal: Vec3::new(0.0, 1.0, 0.0), uv: Vec2::new(0.0, 0.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(1.0, 1.0, 1.0) + block, normal: Vec3::new(0.0, 1.0, 0.0), uv: Vec2::new(0.0, 1.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(1.0, 1.0, 0.0) + block, normal: Vec3::new(0.0, 1.0, 0.0), uv: Vec2::new(1.0, 1.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(0.0, 1.0, 0.0) + block, normal: Vec3::new(0.0, 1.0, 0.0), uv: Vec2::new(1.0, 0.0)});
            }

            Direction::Down => {
                vertices.push(ChunkVertices{vertices: Vec3::new(1.0, 0.0, 1.0) + block, normal: Vec3::new(0.0, -1.0, 0.0), uv: Vec2::new(0.0, 0.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(0.0, 0.0, 1.0) + block, normal: Vec3::new(0.0, -1.0, 0.0), uv: Vec2::new(0.0, 1.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(0.0, 0.0, 0.0) + block, normal: Vec3::new(0.0, -1.0, 0.0), uv: Vec2::new(1.0, 1.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(1.0, 0.0, 0.0) + block, normal: Vec3::new(0.0, -1.0, 0.0), uv: Vec2::new(1.0, 0.0)});
            }

            Direction::North => {
                vertices.push(ChunkVertices{vertices: Vec3::new(1.0, 1.0, 0.0) + block, normal: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(0.0, 0.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(1.0, 0.0, 0.0) + block, normal: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(0.0, 1.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(0.0, 0.0, 0.0) + block, normal: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(1.0, 1.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(0.0, 1.0, 0.0) + block, normal: Vec3::new(0.0, 0.0, 1.0), uv: Vec2::new(1.0, 0.0)});
            }

            Direction::South => {
                vertices.push(ChunkVertices{vertices: Vec3::new(0.0, 1.0, 1.0) + block, normal: Vec3::new(0.0, 0.0, -1.0), uv: Vec2::new(0.0, 0.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(0.0, 0.0, 1.0) + block, normal: Vec3::new(0.0, 0.0, -1.0), uv: Vec2::new(0.0, 1.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(1.0, 0.0, 1.0) + block, normal: Vec3::new(0.0, 0.0, -1.0), uv: Vec2::new(1.0, 1.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(1.0, 1.0, 1.0) + block, normal: Vec3::new(0.0, 0.0, -1.0), uv: Vec2::new(1.0, 0.0)});
            }

            Direction::West => {
                vertices.push(ChunkVertices{vertices: Vec3::new(0.0, 1.0, 0.0) + block, normal: Vec3::new(-1.0, 0.0, 0.0), uv: Vec2::new(0.0, 0.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(0.0, 0.0, 0.0) + block, normal: Vec3::new(-1.0, 0.0, 0.0), uv: Vec2::new(0.0, 1.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(0.0, 0.0, 1.0) + block, normal: Vec3::new(-1.0, 0.0, 0.0), uv: Vec2::new(1.0, 1.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(0.0, 1.0, 1.0) + block, normal: Vec3::new(-1.0, 0.0, 0.0), uv: Vec2::new(1.0, 0.0)});
            }

            Direction::East => {
                vertices.push(ChunkVertices{vertices: Vec3::new(1.0, 1.0, 1.0) + block, normal: Vec3::new(1.0, 0.0, 0.0), uv: Vec2::new(0.0, 0.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(1.0, 0.0, 1.0) + block, normal: Vec3::new(1.0, 0.0, 0.0), uv: Vec2::new(0.0, 1.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(1.0, 0.0, 0.0) + block, normal: Vec3::new(1.0, 0.0, 0.0), uv: Vec2::new(1.0, 1.0)});
                vertices.push(ChunkVertices{vertices: Vec3::new(1.0, 1.0, 0.0) + block, normal: Vec3::new(1.0, 0.0, 0.0), uv: Vec2::new(1.0, 0.0)});
            }
        }
    }
}