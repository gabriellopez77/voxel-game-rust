use crate::math::{Vec2i, Vec3, Vec3i};
use crate::world::Chunk;


pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn align_up(value: i32, alignment: i32) -> i32 {
    ((value + alignment - 1) / alignment) * alignment
}

pub fn get_index(x: i32, y: i32, z: i32) -> usize  {
    (z *  Chunk::CHUNK_SIZE.y * Chunk::CHUNK_SIZE.x + (y * Chunk::CHUNK_SIZE.x) + x) as usize
}

pub fn get_chunk_pos(global_coords: Vec3) -> Vec3i {
     Vec3i {
        x: (global_coords.x / Chunk::CHUNK_SIZEF.x).floor() as i32,
        y: (global_coords.y / Chunk::CHUNK_SIZEF.y).floor() as i32,
        z: (global_coords.z / Chunk::CHUNK_SIZEF.z).floor() as i32
    }
}

pub fn get_chunk_posi(global_coords: Vec3i) -> Vec3i {
     Vec3i {
        x: (global_coords.x as f32 / Chunk::CHUNK_SIZEF.x) as i32,
        y: (global_coords.y as f32 / Chunk::CHUNK_SIZEF.y) as i32,
        z: (global_coords.z as f32 / Chunk::CHUNK_SIZEF.z) as i32
    }
}

pub fn get_chunk_block(chunk_pos: Vec3i, global_coords: Vec3i) -> Vec3i {
    Vec3i {
        x: -(chunk_pos.x * Chunk::CHUNK_SIZE.x - global_coords.x),
        y: -(chunk_pos.y * Chunk::CHUNK_SIZE.y - global_coords.y),
        z: -(chunk_pos.z * Chunk::CHUNK_SIZE.z - global_coords.z)
    }
}

pub fn get_chunk_region(chunk_pos: Vec3i) -> Vec2i {
    Vec2i {
        x: (chunk_pos.x / Chunk::REGION_SIZE),
        y: (chunk_pos.z  / Chunk::REGION_SIZE)
    }
}

pub fn get_chunk_distance(pos1: Vec3i, pos2: Vec3i ) -> i32 {
    let x = (pos1.x - pos2.x).pow(2) as f32;
    let z = (pos1.z - pos2.z).pow(2) as f32;

    return (x + z).sqrt() as i32;
}
