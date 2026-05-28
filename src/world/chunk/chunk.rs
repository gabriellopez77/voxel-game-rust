use std::sync::Arc;
use std::{cell::RefCell, rc::Rc};
use std::sync::atomic::AtomicI32;
use crate::math::{self, Vec2, Vec3, Vec3i};
use crate::render::chunk_renderer::RendererType;
use crate::render::{BlockModelMesh, ChunkRenderer, ChunkVertices, Shader, Texture};
use crate::world::blocks::{BlockProperties, BlockTypes, BlocksManager};
use crate::world::WorldGen;
use crate::world::chunk::{ChunkData, ChunkMeshResult, NeighborChunks};


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

pub struct Chunk {
    pub position: Vec3i,

    pub chunk_data: ChunkData,

    pub mesh_generated: bool,
    pub renderer: ChunkRenderer,
    pub inside_frustum: bool,

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

    pub fn start(&mut self, world_gen: &mut WorldGen, blocks_manager: &BlocksManager) {
        world_gen.gen_data(self.position, &mut self.chunk_data, blocks_manager);
    }

    pub fn draw(&self, render_type: RendererType) {
        self.renderer.draw(render_type);
    }

    pub fn erase(&mut self) {
        self.renderer.erase();
    }

    pub fn lock(&self) { self.using_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst); }
    pub fn unlock(&self) { self.using_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst); }

    pub fn gen_mesh(chunk: &Chunk, neighbors: &NeighborChunks, blocks_manager: &BlocksManager,
                    mesh_result: &mut ChunkMeshResult) {
        for x in 0..Chunk::CHUNK_SIZE.x {
        for y in 0..Chunk::CHUNK_SIZE.y {
        for z in 0..Chunk::CHUNK_SIZE.z {
            let block_id = chunk.chunk_data.get_blocki(x, y, z);

            // air does not have model
            if block_id == 0 { continue; }

            let chunk_block = Vec3{x: x as f32, y: y as f32, z: z as f32};
            let mut draw = false;

            let block_functions = blocks_manager.get(block_id);
            let block_properties = block_functions.get_properties();
            let model = block_properties.base_properties.get_model();
            let ambient_occlusion = model.ambient_occlusion;

            let mut vertices = mesh_result.get_vertices(block_properties.renderer_type);

            // add nothing faces
            Self::add_face(&chunk, blocks_manager, neighbors, &mut vertices, &model.nothing_vertices, chunk_block, Directions::Nothing, ambient_occlusion);


            // up
            if y < Chunk::CHUNK_SIZE_MINUS_ONE.y {
                let temp = blocks_manager.get(chunk.chunk_data.get_blocki(x, y + 1, z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::Up);
            }
            else if y == Chunk::CHUNK_SIZE_MINUS_ONE.y { draw = true }

            if draw { Self::add_face(&chunk, blocks_manager, neighbors, &mut vertices, &model.up_vertices, chunk_block, Directions::Up, ambient_occlusion); }
            draw = false;


            // down
            if y > 0 {
                let temp = blocks_manager.get(chunk.chunk_data.get_blocki(x, y - 1, z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::Down);
            }

            if draw { Self::add_face(&chunk, blocks_manager, neighbors, &mut vertices, &model.down_vertices, chunk_block, Directions::Down, ambient_occlusion); }
            draw = false;


            // south
            if z < Chunk::CHUNK_SIZE_MINUS_ONE.z {
                let temp = blocks_manager.get(chunk.chunk_data.get_blocki(x, y, z + 1));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::South);
            }
            else if let Some(ref south) = neighbors.south.chunk {
                let temp = blocks_manager.get(south.borrow().chunk_data.get_blocki(x, y, 0));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::South);
            }

            if draw { Self::add_face(&chunk, blocks_manager, neighbors, &mut vertices, &model.south_vertices, chunk_block, Directions::South, ambient_occlusion); }
            draw = false;


            // north
            if z > 0 {
                let temp = blocks_manager.get(chunk.chunk_data.get_blocki(x, y, z - 1));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::North);
            }
            else if let Some(ref north) = neighbors.north.chunk {
                let temp = blocks_manager.get(north.borrow().chunk_data.get_blocki(x, y, Self::CHUNK_SIZE_MINUS_ONE.z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::North);
            }

            if draw { Self::add_face(&chunk, blocks_manager, neighbors, &mut vertices, &model.north_vertices, chunk_block, Directions::North, ambient_occlusion); }
            draw = false;


            // east
            if x < Chunk::CHUNK_SIZE_MINUS_ONE.x {
                let temp = blocks_manager.get(chunk.chunk_data.get_blocki(x + 1, y, z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::East);
            }
            else if let Some(ref east) = neighbors.east.chunk {
                let temp = blocks_manager.get(east.borrow().chunk_data.get_blocki(0, y, z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::East);
            }

            if draw { Self::add_face(&chunk, blocks_manager, neighbors, &mut vertices, &model.east_vertices, chunk_block, Directions::East, ambient_occlusion); }
            draw = false;


            // west
            if x > 0 {
                let temp = blocks_manager.get(chunk.chunk_data.get_blocki(x - 1, y, z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::West);
            }
            else if let Some(ref west) = neighbors.west.chunk {
                let temp = blocks_manager.get(west.borrow().chunk_data.get_blocki(Self::CHUNK_SIZE_MINUS_ONE.x, y, z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::West);
            }

            if draw { Self::add_face(&chunk, blocks_manager, neighbors, &mut vertices, &model.west_vertices, chunk_block, Directions::West, ambient_occlusion); }
        }
        }
        }

        mesh_result.gen_indices();
    }

    fn add_face(chunk: &Chunk, blocks_manager: &BlocksManager, neighbors: &NeighborChunks,
                vertices: &mut Vec<ChunkVertices>, model_vertices: &Vec<BlockModelMesh>, chunk_block: Vec3,
                dir: Directions, ambient_occlusion: bool) {
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
                ao_level1 = Self::get_ao_level(&chunk, blocks_manager, neighbors, chunk_block.as_vec3i(), vert1.vertices, dir, 1);
                ao_level2 = Self::get_ao_level(&chunk, blocks_manager, neighbors, chunk_block.as_vec3i(), vert2.vertices, dir, 2);
                ao_level3 = Self::get_ao_level(&chunk, blocks_manager, neighbors, chunk_block.as_vec3i(), vert3.vertices, dir, 3);
                ao_level4 = Self::get_ao_level(&chunk, blocks_manager, neighbors, chunk_block.as_vec3i(), vert4.vertices, dir, 4);
			}

            let flag1 = ao_level1 | ((vert1.shade as u8) << 2);
            let flag2 = ao_level2 | ((vert2.shade as u8) << 2);
            let flag3 = ao_level3 | ((vert3.shade as u8) << 2);
            let flag4 = ao_level4 | ((vert4.shade as u8) << 2);

            vertices.push(ChunkVertices { vertices: vert1.vertices + chunk_block, normal: vert1.normal, uv: vert1.uv, flags: flag1 });
            vertices.push(ChunkVertices { vertices: vert2.vertices + chunk_block, normal: vert2.normal, uv: vert2.uv, flags: flag2 });
            vertices.push(ChunkVertices { vertices: vert3.vertices + chunk_block, normal: vert3.normal, uv: vert3.uv, flags: flag3 });
            vertices.push(ChunkVertices { vertices: vert4.vertices + chunk_block, normal: vert4.normal, uv: vert4.uv, flags: flag4 });
        }
    }

    fn draw_face(current: &BlockProperties, around: &BlockProperties, dir: Directions) -> bool {
	    if around.is_transparent {
		    if current.block_type == around.block_type {
				// glasses
			    if current.block_type == BlockTypes::Glass { return false }

			    // water
			    if current.block_type == BlockTypes::Water { return false }

			    // slabs
			    if current.block_type == BlockTypes::Slab && dir != Directions::Up && dir != Directions::Down { return false }

			    // snow layer
			    if current.block_type == BlockTypes::SnowLayer && dir != Directions::Up && dir != Directions::Down { return false }
		    }
		    else {
			    if current.block_type != BlockTypes::Water && around.block_type == BlockTypes::Slab && dir == Directions::Up { return false }
			    if current.block_type != BlockTypes::Water && around.block_type == BlockTypes::SnowLayer && dir == Directions::Up { return false }
		    }

		    return true;
	    }

	    if current.block_type == BlockTypes::Slab && dir == Directions::Up { return true }
	    if current.block_type == BlockTypes::SnowLayer && dir == Directions::Up { return true }
	    if !current.is_transparent && around.is_transparent { return true }

	    return false;
    }

    fn get_ao_level(chunk: &Chunk, blocks_manager: &BlocksManager, neighbors: &NeighborChunks,
                    chunk_block: Vec3i, face_pos: Vec3, dir: Directions, vertex: u8) -> u8 {
        let chunk_pos = { chunk.position };
        let chunk_data = { &chunk.chunk_data };

   	    let get_ao = |dx: f32, dy: f32, dz: f32| -> u8 {
            let ndx = (if dx < 0.0 { dx.ceil() } else { dx.floor() }).clamp(-1.0, 1.0) as i32;
            let ndy = (if dy < 0.0 { dy.ceil() } else { dy.floor() }).clamp(-1.0, 1.0) as i32;
            let ndz = (if dz < 0.0 { dz.ceil() } else { dz.floor() }).clamp(-1.0, 1.0) as i32;

      		// block in same chunk
		    if chunk_block.x >= 1 && chunk_block.x <= 14 && chunk_block.z >= 1 && chunk_block.z <= 14 {
			    let new_chunk_block = chunk_block + Vec3i::new(ndx, ndy, ndz);

			    if new_chunk_block.y > Self::CHUNK_SIZE_MINUS_ONE.y || new_chunk_block.y < 0 {
				    return 0;
				}

			    return !blocks_manager.get(chunk_data.get_block(new_chunk_block)).get_properties().is_transparent as u8;
		    }


		    // in another chunk
		    let global_block = (chunk_pos * Self::CHUNK_SIZE) + chunk_block + Vec3i::new(ndx, ndy, ndz);

		    if global_block.y > Self::CHUNK_SIZE_MINUS_ONE.y || global_block.y < 0 {
			    return 0;
			}

		    let other_chunk_pos = math::get_chunk_pos(global_block.as_vec3());
		    let other_chunk_block = math::get_chunk_block(other_chunk_pos, global_block);

            enum Tee<'a> {
                Same(&'a Chunk),
                Other(Option<&'a Arc<RefCell<Chunk>>>)
            }

			let mut ch = Tee::Same(chunk);

            if other_chunk_pos != chunk_pos {
                // around chunks
                if other_chunk_pos      == Vec3i::new(chunk_pos.x, 0, chunk_pos.z - 1) { ch = Tee::Other(neighbors.north.chunk.as_ref()) }
                else if other_chunk_pos == Vec3i::new(chunk_pos.x, 0, chunk_pos.z + 1) { ch = Tee::Other(neighbors.south.chunk.as_ref()) }
                else if other_chunk_pos == Vec3i::new(chunk_pos.x - 1, 0, chunk_pos.z) { ch = Tee::Other(neighbors.west.chunk.as_ref()) }
                else if other_chunk_pos == Vec3i::new(chunk_pos.x + 1, 0, chunk_pos.z) { ch = Tee::Other(neighbors.east.chunk.as_ref()) }

                // corner chunks
                else if other_chunk_pos == Vec3i::new(chunk_pos.x - 1, 0, chunk_pos.z - 1) { ch = Tee::Other(neighbors.northwest.chunk.as_ref()) }
                else if other_chunk_pos == Vec3i::new(chunk_pos.x + 1, 0, chunk_pos.z - 1) { ch = Tee::Other(neighbors.northeast.chunk.as_ref()) }
                else if other_chunk_pos == Vec3i::new(chunk_pos.x - 1, 0, chunk_pos.z + 1) { ch = Tee::Other(neighbors.southwest.chunk.as_ref()) }
                else if other_chunk_pos == Vec3i::new(chunk_pos.x + 1, 0, chunk_pos.z + 1) { ch = Tee::Other(neighbors.southeast.chunk.as_ref()) }
            }

            match ch {
                Tee::Same(c) => return !blocks_manager.get(c.chunk_data.get_block(other_chunk_block)).get_properties().is_transparent as u8,
                Tee::Other(o) if let Some(c) = o => {
                    return !blocks_manager.get(c.borrow().chunk_data.get_block(other_chunk_block)).get_properties().is_transparent as u8
                }
                _ => {}
            }

			return 0;
        };

   	    let mut ao_level: u8 = 3;

	    if chunk_block.y > Self::CHUNK_SIZE_MINUS_ONE.y || chunk_block.y < 0 {
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
