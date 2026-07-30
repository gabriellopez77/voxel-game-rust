use crate::{math::{self, Vec3i}, utils::SafePtr, world::{Chunk, blocks::{BlockIdState, BlockProperties, BlocksManager}}};


#[derive(Clone, Copy)]
pub struct ChunkBlockInfo {
    pub id: u16,
    pub state: u8
}

#[derive(Clone)]
pub struct ChunkData {
    blocks_id: [u16; Chunk::CHUNK_DATA_SIZE],
    blocks_state: [u8; Chunk::CHUNK_DATA_SIZE],

    pub position: Vec3i,

    pub regen_mesh: bool,

    blocks_manager: SafePtr<BlocksManager>,
}

unsafe impl Send for ChunkData {}
unsafe impl Sync for ChunkData {}

impl ChunkData {
    pub fn new(position: Vec3i, blocks_manager: SafePtr<BlocksManager>) -> Self {
        Self {
            blocks_id: [0; Chunk::CHUNK_DATA_SIZE],
            blocks_state: [0; Chunk::CHUNK_DATA_SIZE],

            position: position,

            regen_mesh: false,

            blocks_manager,
        }
    }

    pub fn clear(&mut self, new_position: Vec3i) {
        self.position = new_position;

        self.regen_mesh = false;
        self.blocks_id.fill(0);
        self.blocks_state.fill(0);
    }

    pub fn get_block_properties(&self, chunk_block: Vec3i) -> SafePtr<BlockProperties> {
        self.blocks_manager.get_properties_from_block_info(self.get_block_info(chunk_block))
    }

    pub fn get_block_propertiesi(&self, x: i32, y: i32, z: i32) -> SafePtr<BlockProperties> {
        self.blocks_manager.get_properties_from_block_info(self.get_block_info(Vec3i::new(x, y, z)))
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

    pub fn set_block(&mut self, chunk_block: Vec3i, id_state: BlockIdState) {
        self.set_block_index(math::get_index(chunk_block.x, chunk_block.y, chunk_block.z), id_state);
    }

    pub fn set_block_index(&mut self, index: usize, id_state: BlockIdState) {
        let current_id = &mut self.blocks_id[index];
        let current_state = &mut self.blocks_state[index];

        self.regen_mesh |= *current_id != id_state.id || *current_state != id_state.state;

        *current_id = id_state.id;
        *current_state = id_state.state;
    }
}
