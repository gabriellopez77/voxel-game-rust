use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicI32;
use crate::math::{self, Vec3, Vec3i};
use crate::render::{BlockModelMesh, ChunkRenderer, ChunkVertices, GlobalRenderer};
use crate::utils::SafePtr;
use crate::world::blocks::{BlockProperties, BlockTypes, BlocksManager};
use crate::world::chunk::neighbors_data::NeighborsData;
use crate::world::chunk::{ChunkData, ChunkMeshResult};
use crate::world::world_gen::WorldGen;


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Directions {
    Up,
    Down,
    North,
    South,
    West,
    East,
    Nothing,
}

impl Directions {
    pub fn is_vertical(self) -> bool {
        self == Self::Up || self == Self::Down
    }
}

pub struct Chunk {
    pub position: Vec3i,
    pub visual_position: Vec3,

    pub chunk_data: Arc<RwLock<ChunkData>>,

    pub renderer: Option<ChunkRenderer>,
    pub inside_frustum: bool,

    using_count: AtomicI32,
}

impl Chunk {
    pub const CHUNK_SIZE: Vec3i = Vec3i { x: 16, y: 128, z: 16 };
    pub const CHUNK_SIZE_MINUS_ONE: Vec3i = Vec3i { x: 15, y: 127, z: 15 };
    pub const CHUNK_SIZEF: Vec3 = Vec3 { x: 16.0, y: 128.0, z: 16.0 };
    pub const CHUNK_DATA_SIZE: usize = (Self::CHUNK_SIZE.x * Self::CHUNK_SIZE.y * Self::CHUNK_SIZE.z) as usize;
    pub const REGION_SIZE: i32 = 16;

    pub fn new(position: Vec3i, chunk_data: Option<Arc<RwLock<ChunkData>>>, blocks_manager: SafePtr<BlocksManager>) -> Self {
        let visual_position = position * Self::CHUNK_SIZE;

        Self {
            position,
            visual_position: visual_position.as_vec3(),

            chunk_data: if chunk_data.is_none() { Arc::new(RwLock::new(ChunkData::new(position, blocks_manager))) } else { chunk_data.unwrap() },

            renderer: None,
            inside_frustum: false,

            using_count: AtomicI32::new(0)
        }
    }

    pub fn start(&mut self, world_gen: &mut WorldGen, blocks_manager: &BlocksManager) {
        world_gen.gen_data(self.position, &mut self.chunk_data.write().unwrap(), blocks_manager);
    }

    pub fn draw(&mut self, dt: f32, global_renderer: &mut GlobalRenderer) {
        self.renderer.as_mut().unwrap().draw(dt, global_renderer);
    }

    pub fn erase(&mut self) {
        if let Some(ref mut renderer) = self.renderer {
            renderer.erase();
        }
    }

    pub fn lock(&self) { self.using_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst); }
    pub fn unlock(&self) { self.using_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst); }

    pub fn gen_mesh(mesh_result: &mut ChunkMeshResult, blocks_manager: &BlocksManager) {
        let chunk_data = &*mesh_result.chunk_data.read().unwrap();
        let neighbors_data = &mesh_result.neighbors_data;

        let chunk_pos = chunk_data.position.as_vec3() * Self::CHUNK_SIZEF;

        for x in 0..Chunk::CHUNK_SIZE.x {
        for y in 0..Chunk::CHUNK_SIZE.y {
        for z in 0..Chunk::CHUNK_SIZE.z {
            let block_info = chunk_data.get_block_infoi(x, y, z);

            // air does not have model
            if block_info.id == 0 { continue }

            let block_properties = blocks_manager.get_properties_from_block_info(block_info);
            let model = &block_properties.base_properties.model;
            let ambient_occlusion = model.ambient_occlusion;

            let chunk_block = Vec3::new(x as f32, y as f32, z as f32);
            let mut vertices = &mut mesh_result.vertices[block_properties.renderer_type as usize];
            let mut draw = false;

            // add nothing faces
            Self::add_face(chunk_data, neighbors_data, &mut vertices, &model.nothing_vertices, chunk_block, chunk_pos, Directions::Nothing, ambient_occlusion);


            // up
            if y < Chunk::CHUNK_SIZE_MINUS_ONE.y {
                let around = chunk_data.get_block_propertiesi(x, y + 1, z);
                draw = Self::draw_face(&block_properties, &around, Directions::Up);
            }
            else if y == Chunk::CHUNK_SIZE_MINUS_ONE.y { draw = true }

            if draw { Self::add_face(chunk_data, neighbors_data, &mut vertices, &model.up_vertices, chunk_block, chunk_pos, Directions::Up, ambient_occlusion); }
            draw = false;


            // down
            if y > 0 {
                let around = chunk_data.get_block_propertiesi(x, y - 1, z);
                draw = Self::draw_face(&block_properties, &around, Directions::Down);
            }

            if draw { Self::add_face(chunk_data, neighbors_data, &mut vertices, &model.down_vertices, chunk_block, chunk_pos, Directions::Down, ambient_occlusion); }
            draw = false;


            // south
            if z < Chunk::CHUNK_SIZE_MINUS_ONE.z {
                let around = chunk_data.get_block_propertiesi(x, y, z + 1);
                draw = Self::draw_face(&block_properties, &around, Directions::South);
            }
            else if let Some(ref south) = neighbors_data.south {
                let around = south.read().unwrap().get_block_propertiesi(x, y, 0);
                draw = Self::draw_face(&block_properties, &around, Directions::South);
            }

            if draw { Self::add_face(chunk_data, neighbors_data, &mut vertices, &model.south_vertices, chunk_block, chunk_pos, Directions::South, ambient_occlusion); }
            draw = false;


            // north
            if z > 0 {
                let around = chunk_data.get_block_propertiesi(x, y, z - 1);
                draw = Self::draw_face(&block_properties, &around, Directions::North);
            }
            else if let Some(ref north) = neighbors_data.north {
                let around = north.read().unwrap().get_block_propertiesi(x, y, Self::CHUNK_SIZE_MINUS_ONE.z);
                draw = Self::draw_face(&block_properties, &around, Directions::North);
            }

            if draw { Self::add_face(chunk_data, neighbors_data, &mut vertices, &model.north_vertices, chunk_block, chunk_pos, Directions::North, ambient_occlusion); }
            draw = false;


            // east
            if x < Chunk::CHUNK_SIZE_MINUS_ONE.x {
                let around = chunk_data.get_block_propertiesi(x + 1, y, z);
                draw = Self::draw_face(&block_properties, &around, Directions::East);
            }
            else if let Some(ref east) = neighbors_data.east {
                let around = east.read().unwrap().get_block_propertiesi(0, y, z);
                draw = Self::draw_face(&block_properties, &around, Directions::East);
            }

            if draw { Self::add_face(chunk_data, neighbors_data, &mut vertices, &model.east_vertices, chunk_block, chunk_pos, Directions::East, ambient_occlusion); }
            draw = false;


            // west
            if x > 0 {
                let around = chunk_data.get_block_propertiesi(x - 1, y, z);
                draw = Self::draw_face(&block_properties, &around, Directions::West);
            }
            else if let Some(ref west) = neighbors_data.west {
                let around = west.read().unwrap().get_block_propertiesi(Self::CHUNK_SIZE_MINUS_ONE.x, y, z);
                draw = Self::draw_face(&block_properties, &around, Directions::West);
            }

            if draw { Self::add_face(chunk_data, neighbors_data, &mut vertices, &model.west_vertices, chunk_block, chunk_pos, Directions::West, ambient_occlusion); }
        }
        }
        }
    }

    fn add_face(
        chunk_data: &ChunkData,
        neighbors: &NeighborsData,
        vertices: &mut Vec<ChunkVertices>,
        model_vertices: &Vec<BlockModelMesh>,
        chunk_block: Vec3,
        chunk_pos: Vec3,
        dir: Directions,
        ambient_occlusion: bool
    ) {
        for i in (0..model_vertices.len()).step_by(4) {
            let vert1 = &model_vertices[i + 0];
            let vert2 = &model_vertices[i + 1];
            let vert3 = &model_vertices[i + 2];
            let vert4 = &model_vertices[i + 3];

            let mut ao_level1: u8 = 3;
			let mut ao_level2: u8 = 3;
			let mut ao_level3: u8 = 3;
			let mut ao_level4: u8 = 3;

			if ambient_occlusion && dir != Directions::Nothing {
			    let chunk_block = chunk_block.as_vec3i();
                ao_level1 = Self::get_ao_level(chunk_data, neighbors, chunk_block, vert1.vertices, dir, 1);
                ao_level2 = Self::get_ao_level(chunk_data, neighbors, chunk_block, vert2.vertices, dir, 2);
                ao_level3 = Self::get_ao_level(chunk_data, neighbors, chunk_block, vert3.vertices, dir, 3);
                ao_level4 = Self::get_ao_level(chunk_data, neighbors, chunk_block, vert4.vertices, dir, 4);
			}

            let flag1 = ao_level1 | ((vert1.shade as u8) << 2);
            let flag2 = ao_level2 | ((vert2.shade as u8) << 2);
            let flag3 = ao_level3 | ((vert3.shade as u8) << 2);
            let flag4 = ao_level4 | ((vert4.shade as u8) << 2);

            vertices.push(ChunkVertices { vertices: vert1.vertices + chunk_block + chunk_pos, normal: vert1.normal, uv: vert1.uv, flags: flag1 });
            vertices.push(ChunkVertices { vertices: vert2.vertices + chunk_block + chunk_pos, normal: vert2.normal, uv: vert2.uv, flags: flag2 });
            vertices.push(ChunkVertices { vertices: vert3.vertices + chunk_block + chunk_pos, normal: vert3.normal, uv: vert3.uv, flags: flag3 });
            vertices.push(ChunkVertices { vertices: vert4.vertices + chunk_block + chunk_pos, normal: vert4.normal, uv: vert4.uv, flags: flag4 });
        }
    }

    fn draw_face(current: &BlockProperties, other: &BlockProperties, dir: Directions) -> bool {
        let current_type = current.block_type;
        let other_type = other.block_type;

	    if other.is_transparent {
	        if current_type == other_type {
				// glasses
			    if current_type == BlockTypes::Glass { return false }

			    // water
			    if current_type == BlockTypes::Water { return false }

			    // slabs
			    if current_type == BlockTypes::Slab && !dir.is_vertical() { return false }

			    // snow layer
			    if current_type == BlockTypes::SnowLayer && !dir.is_vertical() { return false }
		    }
		    else {
			    if current_type != BlockTypes::Water && other_type == BlockTypes::Slab && dir == Directions::Up { return false }
			    if current_type != BlockTypes::Water && other_type == BlockTypes::SnowLayer && dir == Directions::Up { return false }
		    }

		    return true;
	    }

		if current_type == BlockTypes::Water && dir == Directions::Up { return true }
        if current_type == BlockTypes::Slab && dir == Directions::Up { return true }
        if current_type == BlockTypes::SnowLayer && dir == Directions::Up { return true }
	    if !current.is_transparent && other.is_transparent { return true }

	    return false;
    }

    fn get_ao_level(
        chunk_data: &ChunkData,
        neighbors_data: &NeighborsData,
        ch_block: Vec3i,
        face_pos: Vec3,
        dir: Directions,
        vertex: u8
    ) -> u8 {
        let ch_pos = { chunk_data.position };

   	    let get_ao = |dx: f32, dy: f32, dz: f32| -> u8 {
            let ndx = (if dx < 0.0 { dx.ceil() } else { dx.floor() }).clamp(-1.0, 1.0) as i32;
            let ndy = (if dy < 0.0 { dy.ceil() } else { dy.floor() }).clamp(-1.0, 1.0) as i32;
            let ndz = (if dz < 0.0 { dz.ceil() } else { dz.floor() }).clamp(-1.0, 1.0) as i32;

      		// block in same chunk
		    if ch_block.x >= 1 && ch_block.x <= 14 && ch_block.z >= 1 && ch_block.z <= 14 {
			    let new_ch_block = ch_block + Vec3i::new(ndx, ndy, ndz);

			    if new_ch_block.y > Self::CHUNK_SIZE_MINUS_ONE.y || new_ch_block.y < 0 {
				    return 0;
				}

			    return !chunk_data.get_block_properties(new_ch_block).is_transparent as u8;
		    }


		    // in another chunk
		    let global_block = (ch_pos * Self::CHUNK_SIZE) + ch_block + Vec3i::new(ndx, ndy, ndz);

		    if global_block.y > Self::CHUNK_SIZE_MINUS_ONE.y || global_block.y < 0 {
			    return 0;
			}

		    let other_ch_pos = math::get_chunk_pos(global_block.as_vec3());
		    let other_chunk_block = math::get_chunk_block(other_ch_pos, global_block.as_vec3());

            enum Tee<'a> {
                Same(&'a ChunkData),
                Other(Option<&'a Arc<RwLock<ChunkData>>>)
            }

			let mut ch = Tee::Same(chunk_data);

            if other_ch_pos != ch_pos {
                // around chunks
                if other_ch_pos      == Vec3i::new(ch_pos.x, 0, ch_pos.z - 1) { ch = Tee::Other(neighbors_data.north.as_ref()) }
                else if other_ch_pos == Vec3i::new(ch_pos.x, 0, ch_pos.z + 1) { ch = Tee::Other(neighbors_data.south.as_ref()) }
                else if other_ch_pos == Vec3i::new(ch_pos.x - 1, 0, ch_pos.z) { ch = Tee::Other(neighbors_data.west.as_ref()) }
                else if other_ch_pos == Vec3i::new(ch_pos.x + 1, 0, ch_pos.z) { ch = Tee::Other(neighbors_data.east.as_ref()) }

                // corner chunks
                else if other_ch_pos == Vec3i::new(ch_pos.x - 1, 0, ch_pos.z - 1) { ch = Tee::Other(neighbors_data.northwest.as_ref()) }
                else if other_ch_pos == Vec3i::new(ch_pos.x + 1, 0, ch_pos.z - 1) { ch = Tee::Other(neighbors_data.northeast.as_ref()) }
                else if other_ch_pos == Vec3i::new(ch_pos.x - 1, 0, ch_pos.z + 1) { ch = Tee::Other(neighbors_data.southwest.as_ref()) }
                else if other_ch_pos == Vec3i::new(ch_pos.x + 1, 0, ch_pos.z + 1) { ch = Tee::Other(neighbors_data.southeast.as_ref()) }
            }


            match ch {
                Tee::Same(c) => {
                    return !c.get_block_properties(other_chunk_block).is_transparent as u8;
                }
                Tee::Other(o) if let Some(c) = o => {
                    return !c.read().unwrap().get_block_properties(other_chunk_block).is_transparent as u8;
                }
                _ => return 0
            }
        };

   	    let mut ao_level: u8 = 3;

	    if ch_block.y > Self::CHUNK_SIZE_MINUS_ONE.y || ch_block.y < 0 {
		    return ao_level;
		}


        if dir == Directions::Up {
            ao_level -= get_ao(0.0, face_pos.y, 0.0);

            if vertex == 1 {
                ao_level -= get_ao(-1.0 + face_pos.x, face_pos.y, 0.0);
                if ao_level > 1 { ao_level -= get_ao(0.0, face_pos.y, face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(-1.0 + face_pos.x, face_pos.y, face_pos.z) }
            }
            else if vertex == 2 {
                ao_level -= get_ao(0.0, face_pos.y, face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(face_pos.x, face_pos.y, 0.0) }
                if ao_level > 2 { ao_level -= get_ao(face_pos.x, face_pos.y, face_pos.z) }
            }
            else if vertex == 3 {
                ao_level -= get_ao(face_pos.x, face_pos.y, 0.0);
                if ao_level > 1 { ao_level -= get_ao(0.0, face_pos.y, -1.0 + face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(face_pos.x, face_pos.y, -1.0 + face_pos.z) }
            }
            else {
                ao_level -= get_ao(0.0, face_pos.y, -1.0 + face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(-1.0 + face_pos.x, face_pos.y, 0.0) }
                if ao_level > 2 { ao_level -= get_ao(-1.0 + face_pos.x, face_pos.y, -1.0 + face_pos.z) }
            }
        }
        else if dir == Directions::Down {
            ao_level -= get_ao(0.0, -1.0 + face_pos.y, 0.0);

            if vertex == 2 {
                ao_level -= get_ao(-1.0 + face_pos.x, -1.0 + face_pos.y, 0.0);
                if ao_level > 1 { ao_level -= get_ao(0.0, -1.0 + face_pos.y, face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(-1.0 + face_pos.x, -1.0 + face_pos.y, face_pos.z) }
            }
            else if vertex == 1 {
                ao_level -= get_ao(0.0, -1.0 + face_pos.y, face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(face_pos.x, -1.0 + face_pos.y, 0.0) }
                if ao_level > 2 { ao_level -= get_ao(face_pos.x, -1.0 + face_pos.y, face_pos.z) }
            }
            else if vertex == 4 {
                ao_level -= get_ao(face_pos.x, -1.0 + face_pos.y, 0.0);
                if ao_level > 1 { ao_level -= get_ao(0.0, -1.0 + face_pos.y, -1.0 + face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(face_pos.x, -1.0 + face_pos.y, -1.0 + face_pos.z) }
            }
            else {
                ao_level -= get_ao(0.0, -1.0 + face_pos.y, -1.0 + face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(-1.0 + face_pos.x, -1.0 + face_pos.y, 0.0) }
                if ao_level > 2 { ao_level -= get_ao(-1.0 + face_pos.x, -1.0 + face_pos.y, -1.0 + face_pos.z) }
            }
        }
        else if dir == Directions::South {
            ao_level -= get_ao(0.0, 0.0, face_pos.z);

            if vertex == 1 {
                ao_level -= get_ao(-1.0 + face_pos.x, 0.0, face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(0.0, face_pos.y, face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(-1.0 + face_pos.x, face_pos.y, face_pos.z) }
            }
            else if vertex == 2 {
                ao_level -= get_ao(-1.0 + face_pos.x, 0.0, face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(0.0, -1.0 + face_pos.y, face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(-1.0 + face_pos.x, -1.0 + face_pos.y, face_pos.z) }
            }
            else if vertex == 3 {
                ao_level -= get_ao(0.0, -1.0 + face_pos.y, face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(face_pos.x, 0.0, face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(face_pos.x, -1.0 + face_pos.y, face_pos.z) }
            }
            else {
                ao_level -= get_ao(0.0, face_pos.y, face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(face_pos.x, 0.0, face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(face_pos.x, face_pos.y, face_pos.z) }
            }
        }
        else if dir == Directions::North {
            ao_level -= get_ao(0.0, 0.0, -1.0 + face_pos.z);

            if vertex == 4 {
                ao_level -= get_ao(-1.0 + face_pos.x, 0.0, -1.0 + face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(0.0, face_pos.y, -1.0 + face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(-1.0 + face_pos.x, face_pos.y, -1.0 + face_pos.z) }
            }
            else if vertex == 3 {
                ao_level -= get_ao(-1.0 + face_pos.x, 0.0, -1.0 + face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(0.0, -1.0 + face_pos.y, -1.0 + face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(-1.0 + face_pos.x, -1.0 + face_pos.y, -1.0 + face_pos.z) }
            }
            else if vertex == 2 {
                ao_level -= get_ao(0.0, -1.0 + face_pos.y, -1.0 + face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(face_pos.x, 0.0, -1.0 + face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(face_pos.x, -1.0 + face_pos.y, -1.0 + face_pos.z) }
            }
            else {
                ao_level -= get_ao(0.0, face_pos.y, -1.0 + face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(face_pos.x, 0.0, -1.0 + face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(face_pos.x, face_pos.y, -1.0 + face_pos.z) }
            }
        }
        else if dir == Directions::West {
            ao_level -= get_ao(-1.0 + face_pos.x, 0.0, 0.0);

            if vertex == 1 {
                ao_level -= get_ao(-1.0 + face_pos.x, face_pos.y, 0.0);
                if ao_level > 1 { ao_level -= get_ao(-1.0 + face_pos.x, 0.0, -1.0 + face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(-1.0 + face_pos.x, face_pos.y, -1.0 + face_pos.z) }
            }
            else if vertex == 2 {
                ao_level -= get_ao(-1.0 + face_pos.x, 0.0, -1.0 + face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(-1.0 + face_pos.x, -1.0 + face_pos.y, 0.0) }
                if ao_level > 2 { ao_level -= get_ao(-1.0 + face_pos.x, -1.0 + face_pos.y, -1.0 + face_pos.z) }
            }
            else if vertex == 3 {
                ao_level -= get_ao(-1.0 + face_pos.x, -1.0 + face_pos.y, 0.0);
                if ao_level > 1 { ao_level -= get_ao(-1.0 + face_pos.x, 0.0, face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(-1.0 + face_pos.x, -1.0 + face_pos.y, face_pos.z) }
            }
            else {
                ao_level -= get_ao(-1.0 + face_pos.x, face_pos.y, 0.0);
                if ao_level > 1 { ao_level -= get_ao(-1.0 + face_pos.x, 0.0, face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(-1.0 + face_pos.x, face_pos.y, face_pos.z) }
            }
        }
        else if dir == Directions::East {
            ao_level -= get_ao(face_pos.x, 0.0, 0.0);

            if vertex == 4 {
                ao_level -= get_ao(face_pos.x, face_pos.y, 0.0);
                if ao_level > 1 { ao_level -= get_ao(face_pos.x, 0.0, -1.0 + face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(face_pos.x, face_pos.y, -1.0 + face_pos.z) }
            }
            else if vertex == 3 {
                ao_level -= get_ao(face_pos.x, 0.0, -1.0 + face_pos.z);
                if ao_level > 1 { ao_level -= get_ao(face_pos.x, -1.0 + face_pos.y, 0.0) }
                if ao_level > 2 { ao_level -= get_ao(face_pos.x, -1.0 + face_pos.y, -1.0 + face_pos.z) }
            }
            else if vertex == 2 {
                ao_level -= get_ao(face_pos.x, -1.0 + face_pos.y, 0.0);
                if ao_level > 1 { ao_level -= get_ao(face_pos.x, 0.0, face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(face_pos.x, -1.0 + face_pos.y, face_pos.z) }
            }
            else {
                ao_level -= get_ao(face_pos.x, face_pos.y, 0.0);
                if ao_level > 1 { ao_level -= get_ao(face_pos.x, 0.0, face_pos.z) }
                if ao_level > 2 { ao_level -= get_ao(face_pos.x, face_pos.y, face_pos.z) }
            }
        }

        return ao_level;

    }
}
