use crate::math::{Vec3, Vec3i};
use crate::world::Chunk;


pub fn align_up(value: i32, alignment: i32) -> i32 {
    ((value + alignment - 1) / alignment) * alignment
}

pub fn get_index(x: i32, y: i32, z: i32) -> usize  {
    (z *  Chunk::CHUNK_SIZE.y * Chunk::CHUNK_SIZE.x + (y * Chunk::CHUNK_SIZE.x) + x) as usize
}

pub fn get_chunk_pos(global_coords: Vec3) -> Vec3i {
     Vec3i {
        x: (global_coords.x / Chunk::CHUNK_SIZEF.x) as i32,
        y: (global_coords.y / Chunk::CHUNK_SIZEF.y) as i32,
        z: (global_coords.z / Chunk::CHUNK_SIZEF.z) as i32
    }
}