use std::{cell::RefCell, rc::Rc};
use std::sync::atomic::AtomicI32;
use crate::math::{Vec2, Vec3, Vec3i};
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
    East
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

    pub fn gen_mesh(&self, neighbor_chunks: &NeighborChunks, blocks_manager: &BlocksManager,
                    mesh_result: &mut ChunkMeshResult) {
        fn add_face(vertices: &mut Vec<ChunkVertices>, chunk_block: Vec3, model_vertices: &Vec<BlockModelMesh>) {
            for i in (0..model_vertices.len()).step_by(4) {
                let vert1 = &model_vertices[i + 0];
                let vert2 = &model_vertices[i + 1];
                let vert3 = &model_vertices[i + 2];
                let vert4 = &model_vertices[i + 3];

                let flag1 = (vert1.shade as u8) << 2;
			    let flag2 = (vert2.shade as u8) << 2;
			    let flag3 = (vert3.shade as u8) << 2;
			    let flag4 = (vert4.shade as u8) << 2;

                vertices.push(ChunkVertices { vertices: vert1.vertices + chunk_block, normal: vert1.normal, uv: vert1.uv, flags: flag1 });
                vertices.push(ChunkVertices { vertices: vert2.vertices + chunk_block, normal: vert2.normal, uv: vert2.uv, flags: flag2 });
                vertices.push(ChunkVertices { vertices: vert3.vertices + chunk_block, normal: vert3.normal, uv: vert3.uv, flags: flag3 });
                vertices.push(ChunkVertices { vertices: vert4.vertices + chunk_block, normal: vert4.normal, uv: vert4.uv, flags: flag4 });
            }
        }

        for x in 0..Chunk::CHUNK_SIZE.x {
        for y in 0..Chunk::CHUNK_SIZE.y {
        for z in 0..Chunk::CHUNK_SIZE.z {
            let block_id = self.chunk_data.get_blocki(x, y, z);

            // air does not have model
            if block_id == 0 { continue; }

            let chunk_block = Vec3{x: x as f32, y: y as f32, z: z as f32};
            let mut draw = false;

            let block_functions = blocks_manager.get(block_id);
            let block_properties = block_functions.get_properties();
            let model = block_properties.base_properties.get_model();

            let mut vertices = mesh_result.get_vertices(block_properties.renderer_type);

            // add nothing faces
            add_face(&mut vertices, chunk_block, &model.nothing_vertices);

            if y < Chunk::CHUNK_SIZE_MINUS_ONE.y {
                let temp = blocks_manager.get(self.chunk_data.get_blocki(x, y + 1, z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::Up);
            }
            else if y == Chunk::CHUNK_SIZE_MINUS_ONE.y { draw = true }

            if draw { add_face(&mut vertices, chunk_block, &model.up_vertices); }
            draw = false;


            if y > 0 {
                let temp = blocks_manager.get(self.chunk_data.get_blocki(x, y - 1, z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::Down);
            }

            if draw { add_face(&mut vertices, chunk_block, &model.down_vertices); }
            draw = false;


            if z < Chunk::CHUNK_SIZE_MINUS_ONE.z {
                let temp = blocks_manager.get(self.chunk_data.get_blocki(x, y, z + 1));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::South);
            }
            else if let Some(ref south) = neighbor_chunks.south.chunk {
                let temp = blocks_manager.get(south.borrow().chunk_data.get_blocki(x, y, 0));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::South);
            }

            if draw { add_face(&mut vertices, chunk_block, &model.south_vertices); }
            draw = false;


            if z > 0 {
                let temp = blocks_manager.get(self.chunk_data.get_blocki(x, y, z - 1));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::North);
            }
            else if let Some(ref north) = neighbor_chunks.north.chunk {
                let temp = blocks_manager.get(north.borrow().chunk_data.get_blocki(x, y, Self::CHUNK_SIZE_MINUS_ONE.z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::North);
            }

            if draw { add_face(&mut vertices, chunk_block, &model.north_vertices); }
            draw = false;


            // east
            if x < Chunk::CHUNK_SIZE_MINUS_ONE.x {
                let temp = blocks_manager.get(self.chunk_data.get_blocki(x + 1, y, z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::East);
            }
            else if let Some(ref east) = neighbor_chunks.east.chunk {
                let temp = blocks_manager.get(east.borrow().chunk_data.get_blocki(0, y, z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::East);
            }

            if draw { add_face(&mut vertices, chunk_block, &model.east_vertices); }
            draw = false;


            // west
            if x > 0 {
                let temp = blocks_manager.get(self.chunk_data.get_blocki(x - 1, y, z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::West);
            }
            else if let Some(ref west) = neighbor_chunks.west.chunk {
                let temp = blocks_manager.get(west.borrow().chunk_data.get_blocki(Self::CHUNK_SIZE_MINUS_ONE.x, y, z));
                draw = Self::draw_face(block_properties, temp.get_properties(), Directions::West);
            }

            if draw { add_face(&mut vertices, chunk_block, &model.west_vertices); }
        }
        }
        }

        mesh_result.gen_indices();
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
}
