use std::f32;

use crate::math::{Matrix4, Vec2i, Vec3, Vec3i, Vec4};
use crate::world::Chunk;


pub const FRICTION: f32 = 10.0;
pub const EPSILON: f32 = 0.0001;

struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn get_chunk_pos(global_coords: Vec3) -> Vec3i {
     Vec3i {
        x: (global_coords.x / Chunk::CHUNK_SIZEF.x).floor() as i32,
        y: (global_coords.y / Chunk::CHUNK_SIZEF.y).floor() as i32,
        z: (global_coords.z / Chunk::CHUNK_SIZEF.z).floor() as i32
    }
}

pub fn get_chunk_block(chunk_pos: Vec3i, global_coords: Vec3) -> Vec3i {
    Vec3i {
        x: -(chunk_pos.x * Chunk::CHUNK_SIZE.x - global_coords.x.floor() as i32),
        y: -(chunk_pos.y * Chunk::CHUNK_SIZE.y - global_coords.y.floor() as i32),
        z: -(chunk_pos.z * Chunk::CHUNK_SIZE.z - global_coords.z.floor() as i32)
    }
}

pub fn get_global_block(global_coords: Vec3) -> Vec3i {
    Vec3i {
        x: global_coords.x.floor() as i32,
        y: global_coords.y.floor() as i32,
        z: global_coords.z.floor() as i32,
    }
}

pub fn get_chunk_region(chunk_pos: Vec3i) -> Vec2i {
    Vec2i {
        x: (chunk_pos.x / Chunk::REGION_SIZE),
        y: (chunk_pos.z / Chunk::REGION_SIZE)
    }
}

pub fn get_chunk_distance(pos1: Vec3i, pos2: Vec3i ) -> i32 {
    let x = (pos1.x - pos2.x).pow(2) as f32;
    let z = (pos1.z - pos2.z).pow(2) as f32;

    return (x + z).sqrt() as i32;
}

pub fn get_distance(pos1: Vec3, pos2: Vec3) -> f32 {
    let x = (pos1.x - pos2.x).powf(2.0);
    let y = (pos1.y - pos2.y).powf(2.0);
    let z = (pos1.z - pos2.z).powf(2.0);

    return (x + y + z).sqrt();
}

pub fn look_at_rotation(center: Vec3, object_pos: Vec3) -> Matrix4 {
    let q = look_rotation_quaternion(center, object_pos);

    let sqx = q.x * q.x;
    let sqy = q.y * q.y;
    let sqz = q.z * q.z;
    let sqw = q.w * q.w;

    let xy = q.x * q.y;
    let xz = q.x * q.z;
    let xw = q.x * q.w;

    let yz = q.y * q.z;
    let yw = q.y * q.w;

    let zw = q.z * q.w;

    let s2 = 2.0 / (sqx + sqy + sqz + sqw);

    let mut result = Matrix4::IDENTITY;

    result.values[0].x = 1.0 - (s2 * (sqy + sqz));
    result.values[1].y = 1.0 - (s2 * (sqx + sqz));
    result.values[2].z = 1.0 - (s2 * (sqx + sqy));

    result.values[0].y = s2 * (xy + zw);
    result.values[1].x = s2 * (xy - zw);

    result.values[2].x = s2 * (xz + yw);
    result.values[0].z = s2 * (xz - yw);

    result.values[2].y = s2 * (yz - xw);
    result.values[1].z = s2 * (yz + xw);

    result.values[0].w = 0.0;
    result.values[1].w = 0.0;
    result.values[2].w = 0.0;
    result.values[3] = Vec4::new(0.0, 0.0, 0.0, 1.0);

    return result;
}

fn look_rotation_quaternion(center: Vec3, object_pos: Vec3) -> Quaternion {
    let dir = (object_pos - center).normalized();

    let right = Vec3::UP.cross(dir).normalized();
    let up = dir.cross(right).normalized();

    let num8 = right.x + up.y + dir.z;

    let mut quaternion = Quaternion{ x: 0.0, y: 0.0, z: 0.0, w: 0.0 };

    if num8 > 0.0 {
        let mut num = (num8 + 1.0).sqrt();
        quaternion.w = num * 0.5;
        num = 0.5 / num;
        quaternion.x = (up.z - dir.y) * num;
        quaternion.y = (dir.x - right.z) * num;
        quaternion.z = (right.y - up.x) * num;

        return quaternion;
    }

    if right.x >= up.y && right.x >= dir.z {
        let num7 = (1.0 + right.x - up.y - dir.z).sqrt();
        let num4 = 0.5 / num7;
        quaternion.x = 0.5 * num7;
        quaternion.y = (right.y + up.x) * num4;
        quaternion.z = (right.z + dir.x) * num4;
        quaternion.w = (up.z - dir.y) * num4;

        return quaternion;
    }

    if up.y > dir.z {
        let num6 = (1.0 + up.y - right.x - dir.z).sqrt();
        let num3 = 0.5 / num6;
        quaternion.x = (up.x + right.y) * num3;
        quaternion.y = 0.5 * num6;
        quaternion.z = (dir.y + up.z) * num3;
        quaternion.w = (dir.x - right.z) * num3;

        return quaternion;
    }

    let num5 = (1.0 + dir.z - right.x - up.y).sqrt();
    let num2 = 0.5 / num5;
    quaternion.x = (dir.x + right.z) * num2;
    quaternion.y = (dir.y + up.z) * num2;
    quaternion.z = 0.5 * num5;
    quaternion.w = (right.y - up.x) * num2;

    return quaternion;
}
