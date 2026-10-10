use crate::{
    game::Directions,
    math::{self, Vec3, Vec3i},
    world::{
        chunk::{
            Chunk,
            NeighborsChunksGetter,
            NeighborsChunksData,
            chunk_data::{ChunkDataReadBehavior, ChunkDataReadGuard}
        }
    }
};


const FACES_DIRECTIONS: [[Vec3; 8]; 6] = [
    [   // up
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(0.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, 0.0),
    ],
    [   // down
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(0.0, -1.0, -1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(1.0, -1.0, 0.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(0.0, -1.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(-1.0, -1.0, 0.0),
    ],
    [   // forward
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(-1.0, 0.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(0.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
    ],
    [   // back
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(0.0, 1.0, -1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(1.0, 0.0, -1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(0.0, -1.0, -1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(-1.0, 0.0, -1.0),
    ],
    [   // west
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 0.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, 0.0, -1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(-1.0, -1.0, 0.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(-1.0, 0.0, 1.0),
    ],
    [   // east
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, 0.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(1.0, 0.0, -1.0),
    ],
];

const FACES_INDICES: [(usize, usize, usize); 4] = [
    (7, 6, 5),
    (5, 4, 3),
    (1, 0, 7),
    (3, 2, 1)
];

pub fn get_ao_level2(
    chunk_data: &ChunkDataReadGuard,
    neighbors_data: &NeighborsChunksGetter,
    ch_block: Vec3i,
    face_pos: Vec3,
    face_dir: Directions,
    vertex: u8
) -> [u8; 4] {
    let mut ao_values: [u8; 4] = [0; 4];

    for i in 0..4 {
        let (side1_idx, side2_idx, corner_idx) = FACES_INDICES[i];

        let side1 = get_dir(face_pos, face_dir, side1_idx);
        let side2 = get_dir(face_pos, face_dir, side2_idx);
        let corner = get_dir(face_pos, face_dir, corner_idx);


    }

    return ao_values;
}

fn get_dir(face_pos: Vec3, face_dir: Directions, face_idx: usize) -> Vec3 {
    return match face_dir {
        Directions::Up => FACES_DIRECTIONS[0][face_idx],
        Directions::Down => FACES_DIRECTIONS[1][face_idx],
        Directions::North => FACES_DIRECTIONS[2][face_idx],
        Directions::South => FACES_DIRECTIONS[3][face_idx],
        Directions::West => FACES_DIRECTIONS[4][face_idx],
        Directions::East => FACES_DIRECTIONS[5][face_idx],
    }
}

fn get_ao_values(
    chunk_data: &ChunkDataReadGuard,
    neighbors_data: &NeighborsChunksData,
    chunk_block: Vec3i,
    dir: Vec3
) -> bool {
    let ch_pos = chunk_data.get_position();

    let ndy = (if dir.y < 0.0 { dir.y.ceil() } else { dir.y.floor() }).clamp(-1.0, 1.0) as i32;
    let ndz = (if dir.z < 0.0 { dir.z.ceil() } else { dir.z.floor() }).clamp(-1.0, 1.0) as i32;
    let ndx = (if dir.x < 0.0 { dir.x.ceil() } else { dir.x.floor() }).clamp(-1.0, 1.0) as i32;

    // block in same chunk
    if chunk_block.x >= 1 && chunk_block.x <= 14 && chunk_block.z >= 1 && chunk_block.z <= 14 {
	    let new_ch_block = chunk_block + Vec3i::new(ndx, ndy, ndz);

	    if new_ch_block.y > Chunk::CHUNK_SIZE_MINUS_ONE.y || new_ch_block.y < 0 {
		    return false;
		}

	    return chunk_data.get_block_behaviors(new_ch_block).causes_ambient_occlusion();
    }


    // in another chunk
    let global_block = (ch_pos * Chunk::CHUNK_SIZE) + chunk_block + Vec3i::new(ndx, ndy, ndz);

    if global_block.y > Chunk::CHUNK_SIZE_MINUS_ONE.y || global_block.y < 0 {
	    return false;
	}

    let other_ch_pos = math::get_chunk_pos(global_block.as_vec3());
    let other_chunk_block = math::get_chunk_block(other_ch_pos, global_block.as_vec3());

    if other_ch_pos != ch_pos {
        let ch = {
            if      other_ch_pos == Vec3i::new(ch_pos.x, 0, ch_pos.z - 1) { neighbors_data.north.as_ref() }
            else if other_ch_pos == Vec3i::new(ch_pos.x, 0, ch_pos.z + 1) { neighbors_data.south.as_ref() }
            else if other_ch_pos == Vec3i::new(ch_pos.x - 1, 0, ch_pos.z) { neighbors_data.west.as_ref() }
            else if other_ch_pos == Vec3i::new(ch_pos.x + 1, 0, ch_pos.z) { neighbors_data.east.as_ref() }

            else if other_ch_pos == Vec3i::new(ch_pos.x - 1, 0, ch_pos.z - 1) { neighbors_data.northwest.as_ref() }
            else if other_ch_pos == Vec3i::new(ch_pos.x + 1, 0, ch_pos.z - 1) { neighbors_data.northeast.as_ref() }
            else if other_ch_pos == Vec3i::new(ch_pos.x - 1, 0, ch_pos.z + 1) { neighbors_data.southwest.as_ref() }
            else { neighbors_data.southeast.as_ref() }
        };

        if let Some(ch) = ch {
            return ch.get_block_behaviors(other_chunk_block).causes_ambient_occlusion();
        }
    }

    return chunk_data.get_block_behaviors(other_chunk_block).causes_ambient_occlusion();
}
