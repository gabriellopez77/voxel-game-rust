use crate::{math::{self, Vec3i}, world::Chunk};
use crate::world::blocks::BlockProperties;


#[derive(Clone, Copy)]
pub struct ChunkBlockInfo {
    pub id: u16,
    pub state: u8
}

pub struct ChunkData {
    blocks_id: [u16; Chunk::CHUNK_DATA_SIZE],
    blocks_state: [u8; Chunk::CHUNK_DATA_SIZE],

    pub regen_mesh: bool,
}

impl ChunkData {
    pub fn new() -> Self {
        Self {
            blocks_id: [0; Chunk::CHUNK_DATA_SIZE],
            blocks_state: [0; Chunk::CHUNK_DATA_SIZE],

            regen_mesh: false,
        }
    }

    pub fn get_block_id(&self, chunk_block: Vec3i) -> u16 {
        self.get_blocki_id(chunk_block.x, chunk_block.y, chunk_block.z)
    }

    pub fn get_blocki_id(&self, x: i32, y: i32, z: i32) -> u16 {
        let index = math::get_index(x, y, z);
        self.blocks_id[index]
    }

    pub fn get_block_info(&self, chunk_block: Vec3i) -> ChunkBlockInfo {
        self.get_block_infoi(chunk_block.x, chunk_block.y, chunk_block.z)
    }

    pub fn get_block_infoi(&self, x: i32, y: i32, z: i32) -> ChunkBlockInfo {
        let index = math::get_index(x, y, z);

        return ChunkBlockInfo {
            id: self.blocks_id[index],
            state: self.blocks_state[index]
        };
    }


    pub fn set_blocki(&mut self, x: i32, y: i32, z: i32, block: &BlockProperties) {
        self.set_block_index(math::get_index(x, y, z), block);
    }

    pub fn set_block(&mut self, chunk_block: Vec3i, block: &BlockProperties) {
        self.set_block_index(math::get_index(chunk_block.x, chunk_block.y, chunk_block.z), block);
    }

    pub fn set_block_index(&mut self, index: usize, block: &BlockProperties) {
        let current_id = &mut self.blocks_id[index];
        let current_state = &mut self.blocks_state[index];

        let base_properties = &block.base_properties;

        let id = base_properties.id;

        self.regen_mesh |= *current_id != id || *current_state != base_properties.state;

        *current_id = id;
        *current_state = base_properties.state;
    }
}
