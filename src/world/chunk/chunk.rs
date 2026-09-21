use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;


use crate::{
   game::Directions,
   math::{self, Vec3, Vec3i},
   render::{
       ChunkMesh,
       ChunksRenderer,
       chunks_renderer::ChunkMeshResult,
       vertices_data::{BlockItemVertices, ChunkVertices},
   },
   world::{
       blocks::{BlockBehaviors, block_registry},
       chunk::{
           chunk_data::{
               ChunkData,
               ChunkDataFlags,
               ChunkDataReadBehavior,
               ChunkDataReadGuard,
           },
           neighbors_chunks_data::NeighborsChunksData,
       },
       light_engine::LightType,
       player::Camera,
       world_gen::WorldGen,
   }
};


pub struct SingleThreadContent {
    pub renderer: ChunkMesh,
    pub inside_frustum: bool,
}


pub struct Chunk {
    pub position: Vec3i,
    pub visual_position: Vec3,

    pub data: Arc<ChunkData>,

    pub content: RefCell<SingleThreadContent>,
}

// 'content' is acessed only on the main thread
unsafe impl Sync for Chunk {}

impl Chunk {
    pub const CHUNK_SIZE: Vec3i = Vec3i { x: 16, y: 128, z: 16 };
    pub const CHUNK_SIZE_MINUS_ONE: Vec3i = Vec3i { x: 15, y: 127, z: 15 };
    pub const CHUNK_SIZEF: Vec3 = Vec3 { x: 16.0, y: 128.0, z: 16.0 };
    pub const SUB_CHUNK_SIZE: Vec3i = Vec3i { x: 16, y: 16, z: 16 };
    pub const CHUNK_DATA_SIZE: usize = (Self::CHUNK_SIZE.x * Self::CHUNK_SIZE.y * Self::CHUNK_SIZE.z) as usize;
    pub const SUB_CHUNK_DATA_SIZE: usize = (Self::SUB_CHUNK_SIZE.x * Self::SUB_CHUNK_SIZE.y * Self::SUB_CHUNK_SIZE.z) as usize;
    pub const SUB_CHUNK_COUNT: usize = (Self::CHUNK_SIZE.y / Self::SUB_CHUNK_SIZE.y) as usize;
    pub const REGION_SIZE: usize = 16;

    pub fn new(position: Vec3i, chunk_data: Option<Arc<ChunkData>>) -> Self {
        let visual_position = position * Self::CHUNK_SIZE;

        Self {
            position,
            visual_position: visual_position.as_vec3(),

            data: if let Some(data) = chunk_data { data } else { Arc::new(ChunkData::new(position)) },

            content: RefCell::new(SingleThreadContent {
                renderer: ChunkMesh::new(),
                inside_frustum: false,
            }),
        }
    }

    pub fn start(&mut self, world_gen: &mut WorldGen) {
        world_gen.gen_data(self.position, &self.data);
    }

    pub fn draw(&self,
        chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<Chunk>>>>>,
        camera: &Camera,
        renderer: &mut ChunksRenderer,
        dt: f32,
    ) {
        let mut content = self.content.borrow_mut();

        if camera.view_changed {
            content.inside_frustum = camera.chunk_inside_frustum(self.visual_position);
        }

        if !content.inside_frustum {
            return;
        }

        if self.data.need_regen_mesh() {
            self.data.turn_off_flag(ChunkDataFlags::REGEN_MESH_FLAG);

            renderer.gen_mesh(chunks_map, self.data.clone(), self.position);
        }

        content.renderer.draw(dt, renderer);
    }

    pub fn gen_mesh(mesh_result: &mut ChunkMeshResult) {
        let guard = mesh_result.chunk_data.read_guard();
        let neighbors_data = &mesh_result.neighbors_data;

        let chunk_pos = guard.get_position().as_vec3() * Self::CHUNK_SIZEF;
        let blocks_manager = block_registry::get();

        for x in 0..Chunk::CHUNK_SIZE.x {
        for y in 0..Chunk::CHUNK_SIZE.y {
        for z in 0..Chunk::CHUNK_SIZE.z {
            let id_state = guard.get_block_id_state(Vec3i::new(x, y, z));

            // air does not have model
            if id_state.id == 0 { continue }

            let behaviors = blocks_manager.get(id_state);
            let properties = behaviors.get_properties(0);
            let mesh = &properties.base_properties.model;

            let chunk_block = Vec3::new(x as f32, y as f32, z as f32);
            let mut vertices = &mut mesh_result.vertices[properties.renderer_type as usize];
            let mut draw = false;

            // add nothing faces
            Self::add_face(&guard, neighbors_data, &mut vertices, &mesh.nothing_vertices, chunk_block, chunk_pos, Directions::Nothing, behaviors);


            // up
            if y < Chunk::CHUNK_SIZE_MINUS_ONE.y {
                let around = guard.get_block_behaviors(Vec3i::new(x, y + 1, z));
                draw = Self::draw_face(behaviors, around, Directions::Up);
            }
            else if y == Chunk::CHUNK_SIZE_MINUS_ONE.y { draw = true }

            if draw { Self::add_face(&guard, neighbors_data, &mut vertices, &mesh.up_vertices, chunk_block, chunk_pos, Directions::Up, behaviors); }
            draw = false;


            // down
            if y > 0 {
                let around = guard.get_block_behaviors(Vec3i::new(x, y - 1, z));
                draw = Self::draw_face(behaviors, around, Directions::Down);
            }

            if draw { Self::add_face(&guard, neighbors_data, &mut vertices, &mesh.down_vertices, chunk_block, chunk_pos, Directions::Down, behaviors) }
            draw = false;


            // south
            if z < Chunk::CHUNK_SIZE_MINUS_ONE.z {
                let around = guard.get_block_behaviors(Vec3i::new(x, y, z + 1));
                draw = Self::draw_face(behaviors, around, Directions::South);
            }
            else if let Some(ref south) = neighbors_data.south {
                let around = south.get_block_behaviors(Vec3i::new(x, y, 0));
                draw = Self::draw_face(behaviors, around, Directions::South);
            }

            if draw { Self::add_face(&guard, neighbors_data, &mut vertices, &mesh.south_vertices, chunk_block, chunk_pos, Directions::South, behaviors) }
            draw = false;


            // north
            if z > 0 {
                let around = guard.get_block_behaviors(Vec3i::new(x, y, z - 1));
                draw = Self::draw_face(behaviors, around, Directions::North);
            }
            else if let Some(ref north) = neighbors_data.north {
                let around = north.get_block_behaviors(Vec3i::new(x, y, Self::CHUNK_SIZE_MINUS_ONE.z));
                draw = Self::draw_face(behaviors, around, Directions::North);
            }

            if draw { Self::add_face(&guard, neighbors_data, &mut vertices, &mesh.north_vertices, chunk_block, chunk_pos, Directions::North, behaviors) }
            draw = false;


            // east
            if x < Chunk::CHUNK_SIZE_MINUS_ONE.x {
                let around = guard.get_block_behaviors(Vec3i::new(x + 1, y, z));
                draw = Self::draw_face(behaviors, around, Directions::East);
            }
            else if let Some(ref east) = neighbors_data.east {
                let around = east.get_block_behaviors(Vec3i::new(0, y, z));
                draw = Self::draw_face(behaviors, around, Directions::East);
            }

            if draw { Self::add_face(&guard, neighbors_data, &mut vertices, &mesh.east_vertices, chunk_block, chunk_pos, Directions::East, behaviors) }
            draw = false;


            // west
            if x > 0 {
                let around = guard.get_block_behaviors(Vec3i::new(x - 1, y, z));
                draw = Self::draw_face(behaviors, around, Directions::West);
            }
            else if let Some(ref west) = neighbors_data.west {
                let around = west.get_block_behaviors(Vec3i::new(Self::CHUNK_SIZE_MINUS_ONE.x, y, z));
                draw = Self::draw_face(behaviors, around, Directions::West);
            }

            if draw { Self::add_face(&guard, neighbors_data, &mut vertices, &mesh.west_vertices, chunk_block, chunk_pos, Directions::West, behaviors) }
        }
        }
        }
    }

    fn add_face(
        chunk_data: &ChunkDataReadGuard,
        neighbors_data: &NeighborsChunksData,
        vertices: &mut Vec<ChunkVertices>,
        mesh_vertices: &Vec<BlockItemVertices>,
        chunk_block: Vec3,
        chunk_pos: Vec3,
        dir: Directions,
        behaviors: &dyn BlockBehaviors
    ) {
        let ambient_occlusion = behaviors.affected_by_ambient_occlusion();

        for i in (0..mesh_vertices.len()).step_by(4) {
            let vert1 = &mesh_vertices[i + 0];
            let vert2 = &mesh_vertices[i + 1];
            let vert3 = &mesh_vertices[i + 2];
            let vert4 = &mesh_vertices[i + 3];

            let mut ao_level1: u8 = 3;
			let mut ao_level2: u8 = 3;
			let mut ao_level3: u8 = 3;
			let mut ao_level4: u8 = 3;

			let chunk_blocki = chunk_block.as_vec3i();

			let light_level = behaviors.compute_light_levels(Self::get_light_level(chunk_data, neighbors_data, chunk_blocki, vert1.vertices, dir));

			if ambient_occlusion && dir != Directions::Nothing {
                ao_level1 = Self::get_ao_level(chunk_data, neighbors_data, chunk_blocki, vert1.vertices, dir, 1);
                ao_level2 = Self::get_ao_level(chunk_data, neighbors_data, chunk_blocki, vert2.vertices, dir, 2);
                ao_level3 = Self::get_ao_level(chunk_data, neighbors_data, chunk_blocki, vert3.vertices, dir, 3);
                ao_level4 = Self::get_ao_level(chunk_data, neighbors_data, chunk_blocki, vert4.vertices, dir, 4);
			}

            let flag1 = ao_level1 | ((behaviors.compute_face_shade(vert1.shade) as u8) << 2);
            let flag2 = ao_level2 | ((behaviors.compute_face_shade(vert2.shade) as u8) << 2);
            let flag3 = ao_level3 | ((behaviors.compute_face_shade(vert3.shade) as u8) << 2);
            let flag4 = ao_level4 | ((behaviors.compute_face_shade(vert4.shade) as u8) << 2);

            vertices.push(ChunkVertices { vertices: vert1.vertices + chunk_block + chunk_pos, normal: vert1.normal, uv: vert1.uv, light: light_level, flags: flag1 });
            vertices.push(ChunkVertices { vertices: vert2.vertices + chunk_block + chunk_pos, normal: vert2.normal, uv: vert2.uv, light: light_level, flags: flag2 });
            vertices.push(ChunkVertices { vertices: vert3.vertices + chunk_block + chunk_pos, normal: vert3.normal, uv: vert3.uv, light: light_level, flags: flag3 });
            vertices.push(ChunkVertices { vertices: vert4.vertices + chunk_block + chunk_pos, normal: vert4.normal, uv: vert4.uv, light: light_level, flags: flag4 });
        }
    }

    fn draw_face(current: &dyn BlockBehaviors, other: &dyn BlockBehaviors, dir: Directions) -> bool {
        let current_prop = current.get_properties(0);
        let other_prop = other.get_properties(0);

        // both blocks is opaque, then current face should not be rendered
        if current.is_opaque() && other.is_opaque() {
            return false;
        }

	    if !other.is_opaque() {
      		if *current_prop == *other_prop {
      		    return current.should_render_face_twin(dir);
      		}
	    }

		return current.should_render_face(dir, other);
    }

    fn get_light_level(
        chunk_data: &ChunkDataReadGuard,
        neighbors_data: &NeighborsChunksData,
        ch_block: Vec3i,
        face_pos: Vec3,
        dir: Directions,
    ) -> u8 {
        let get_light = |dx: f32, dy: f32, dz: f32| -> u8 {
            let ndx = (if dx < 0.0 { dx.ceil() } else { dx.floor() }).clamp(-1.0, 1.0) as i32;
            let ndy = (if dy < 0.0 { dy.ceil() } else { dy.floor() }).clamp(-1.0, 1.0) as i32;
            let ndz = (if dz < 0.0 { dz.ceil() } else { dz.floor() }).clamp(-1.0, 1.0) as i32;

            let chunk_pos = chunk_data.get_position();

            let global_block = (chunk_pos * Self::CHUNK_SIZE) + ch_block + Vec3i::new(ndx, ndy, ndz);

      		if global_block.y > Self::CHUNK_SIZE_MINUS_ONE.y || global_block.y < 0 {
			    return 0;
            }

            let other_chunk_pos = math::get_chunk_pos(global_block.as_vec3());
		    let other_chunk_block = math::get_chunk_block(other_chunk_pos, global_block.as_vec3());

            enum Tee<'a> {
                Same(&'a ChunkDataReadGuard<'a>),
                Other(Option<&'a Arc<ChunkData>>)
            }

		    let mut ch_data = Tee::Same(chunk_data);

		    if other_chunk_pos != chunk_pos {
			    if other_chunk_pos == Vec3i::new(chunk_pos.x, 0, chunk_pos.z - 1) {
				    ch_data = Tee::Other(neighbors_data.north.as_ref());
				}
			    else if other_chunk_pos == Vec3i::new(chunk_pos.x, 0, chunk_pos.z + 1) {
				    ch_data = Tee::Other(neighbors_data.south.as_ref());
				}
			    else if other_chunk_pos == Vec3i::new(chunk_pos.x - 1, 0, chunk_pos.z) {
				    ch_data = Tee::Other(neighbors_data.west.as_ref());
				}
			    else if other_chunk_pos == Vec3i::new(chunk_pos.x + 1, 0, chunk_pos.z) {
				    ch_data = Tee::Other(neighbors_data.east.as_ref());
				}
		    }

            return match ch_data {
                Tee::Same(c) => c.get_light(other_chunk_block, LightType::Both),
                Tee::Other(o) if let Some(c) = o => c.get_light(other_chunk_block, LightType::Both),
                _ => 0
            }
        };

        let level: u8;

        if dir == Directions::Up {         level = get_light(0.0, face_pos.y, 0.0) }
        else if dir == Directions::Down {  level = get_light(0.0, -1.0 + face_pos.y, 0.0) }
        else if dir == Directions::South { level = get_light(0.0, 0.0, face_pos.z) }
        else if dir == Directions::North { level = get_light(0.0, 0.0, -1.0 + face_pos.z) }
        else if dir == Directions::West {  level = get_light(-1.0 + face_pos.x, 0.0, 0.0) }
        else if dir == Directions::East {  level = get_light(face_pos.x, 0.0, 0.0) }
		else { level = chunk_data.get_light(ch_block, LightType::Both) }

		return level;
    }

    fn get_ao_level(
        chunk_data: &ChunkDataReadGuard,
        neighbors_data: &NeighborsChunksData,
        ch_block: Vec3i,
        face_pos: Vec3,
        dir: Directions,
        vertex: u8
    ) -> u8 {
        let ch_pos = { chunk_data.get_position() };

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

			    return chunk_data.get_block_behaviors(new_ch_block).causes_ambient_occlusion() as u8;
		    }


		    // in another chunk
		    let global_block = (ch_pos * Self::CHUNK_SIZE) + ch_block + Vec3i::new(ndx, ndy, ndz);

		    if global_block.y > Self::CHUNK_SIZE_MINUS_ONE.y || global_block.y < 0 {
			    return 0;
			}

		    let other_ch_pos = math::get_chunk_pos(global_block.as_vec3());
		    let other_chunk_block = math::get_chunk_block(other_ch_pos, global_block.as_vec3());

            enum Tee<'a> {
                Same(&'a ChunkDataReadGuard<'a>),
                Other(Option<&'a Arc<ChunkData>>)
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


            return match ch {
                Tee::Same(c) => {
                    c.get_block_behaviors(other_chunk_block).causes_ambient_occlusion() as u8
                }
                Tee::Other(o) if let Some(c) = o => {
                    c.get_block_behaviors(other_chunk_block).causes_ambient_occlusion() as u8
                }
                _ => 0
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
