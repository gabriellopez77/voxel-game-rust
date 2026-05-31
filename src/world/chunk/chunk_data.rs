use crate::{math::{self, Vec3i}, world::{Chunk, blocks::BlockFunctions}};


pub struct ChunkData {
    blocks_data: [u16; Chunk::CHUNK_DATA_SIZE],

    pub regen_mesh: bool,
}

impl ChunkData {
    pub fn new() -> Self {
        Self {
            blocks_data: [0; Chunk::CHUNK_DATA_SIZE],

            regen_mesh: false,
        }
    }

    pub fn get_data_mut(&mut self) -> &mut [u16] {
        &mut self.blocks_data
    }

    pub fn get_block(&self, chunk_block: Vec3i) -> u16 {
        let index = math::get_index(chunk_block.x, chunk_block.y, chunk_block.z);
        self.blocks_data[index]
    }

    pub fn get_blocki(&self, x: i32, y: i32, z: i32) -> u16 {
        let index = math::get_index(x, y, z);
        self.blocks_data[index]
    }

    pub fn set_blocki(&mut self, x: i32, y: i32, z: i32, block: &Box<dyn BlockFunctions>) {
        let index = math::get_index(x, y, z);
        self.set_block_index(index, block);
    }

    pub fn set_block(&mut self, chunk_block: Vec3i, block: &Box<dyn BlockFunctions>) {
        let index = math::get_index(chunk_block.x, chunk_block.y, chunk_block.z);
        self.set_block_index(index, block);
    }

    pub fn set_block_index(&mut self, index: usize, block: &Box<dyn BlockFunctions>) {
        let current_block = &mut self.blocks_data[index];

        // SAFETY: ptr is always valid
        let id = block.get_base().id;

        self.regen_mesh |= *current_block != id;

        *current_block = id;
    }
}
