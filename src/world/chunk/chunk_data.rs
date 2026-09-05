use std::sync::atomic::{AtomicBool, Ordering};

use crate::{math::Vec3i, utils::SafePtr, world::{Chunk, blocks::{BlockIdState, BlockProperties, BlocksManager}, light_engine::{self, LightSectionLevel, LightType}}};


#[derive(Clone, Copy)]
pub struct ChunkBlockInfo {
    pub id: u16,
}

pub struct ChunkData {
    pub blocks_id: [u16; Chunk::CHUNK_DATA_SIZE],
    pub light_levels: [u8; Chunk::CHUNK_DATA_SIZE],
    //blocks_state: [u8; Chunk::CHUNK_DATA_SIZE],

    pub light_sections: [LightSectionLevel; Chunk::SUB_CHUNK_COUNT as usize],

    pub position: Vec3i,

    pub regen_mesh: AtomicBool,
    pub contains_emissive_blocks: bool,
    pub light_gen_stage: AtomicBool,

    blocks_manager: SafePtr<BlocksManager>,
}

unsafe impl Send for ChunkData {}
unsafe impl Sync for ChunkData {}

impl ChunkData {
    /// uses the order: y, x, z
    pub fn get_index(x: i32, y: i32, z: i32) -> usize {
        ((y * Chunk::CHUNK_SIZE.x * Chunk::CHUNK_SIZE.z) + (x * Chunk::CHUNK_SIZE.z) + z) as usize
    }

    pub fn new(position: Vec3i, blocks_manager: SafePtr<BlocksManager>) -> Self {
        Self {
            blocks_id: [0; Chunk::CHUNK_DATA_SIZE],
            //blocks_state: [0; Chunk::CHUNK_DATA_SIZE],
            light_levels: [0; Chunk::CHUNK_DATA_SIZE],

            light_sections: [LightSectionLevel::Two; Chunk::SUB_CHUNK_COUNT],

            position,

            regen_mesh: AtomicBool::new(false),
            contains_emissive_blocks: false,
            light_gen_stage: AtomicBool::new(true),

            blocks_manager,
        }
    }

    pub fn clear(&mut self, new_position: Vec3i) {
        self.blocks_id.fill(0);
        self.light_levels.fill(0);
        //self.blocks_state.fill(0);

        self.light_sections.fill(LightSectionLevel::Two);

        self.position = new_position;

        self.regen_mesh = AtomicBool::new(false);
        self.contains_emissive_blocks = false;
        self.light_gen_stage = AtomicBool::new(true);
    }

    // change the block in chunk_block by the id_state and return the old block
    pub fn change_block(&mut self, chunk_block: Vec3i, id_state: BlockIdState) -> SafePtr<BlockProperties> {
        let old = self.get_block_properties(chunk_block);
        self.set_block(chunk_block, id_state);

        old
    }

    pub fn need_regen_mesh(&self) -> bool {
        self.regen_mesh.load(Ordering::Relaxed) && !self.light_gen_stage.load(Ordering::Relaxed)
        //self.regen_mesh
    }

    pub fn get_block_properties(&self, chunk_block: Vec3i) -> SafePtr<BlockProperties> {
        self.blocks_manager.get_properties_from_block_info(self.get_block_info(chunk_block))
    }

    pub fn get_block_info(&self, chunk_block: Vec3i) -> ChunkBlockInfo {
        let index = Self::get_index(chunk_block.x, chunk_block.y, chunk_block.z);

        return ChunkBlockInfo {
            id: self.blocks_id[index],
            //id: unsafe { *self.blocks_id.get_unchecked(index) },
            //state: self.blocks_state[index]
        };
    }

    pub fn get_light(&self, chunk_block: Vec3i, light_type: LightType) -> u8 {
        let index = Self::get_index(chunk_block.x, chunk_block.y, chunk_block.z);

        let mut light = self.light_levels[index];
        //let mut light = unsafe { *self.light_levels.get_unchecked(index) };

        if light_type == LightType::Sky {
            light &= light_engine::SKY_MASK;
        }
        else if light_type == LightType::Block {
            light >>= 4;
        }

        return light;
    }

    pub fn set_block(&mut self, chunk_block: Vec3i, id_state: BlockIdState) {
        let index = Self::get_index(chunk_block.x, chunk_block.y, chunk_block.z);

        let current_id = &mut self.blocks_id[index];
        //let current_state = &mut self.blocks_state[index];

        //self.regen_mesh |= *current_id != id_state.id || *current_state != id_state.state;
        self.regen_mesh.fetch_or(*current_id != id_state.id, Ordering::Relaxed);

        *current_id = id_state.id;
        //*current_state = id_state.state;
    }

    pub fn set_light(&mut self, chunk_block: Vec3i, value: u8, light_type: LightType) {
        let index = Self::get_index(chunk_block.x, chunk_block.y, chunk_block.z);

        let current_value = &mut self.light_levels[index];
        //let current_value = unsafe { self.light_levels.get_unchecked_mut(index) };

        let final_value = if light_type == LightType::Block {
            (*current_value & light_engine::SKY_MASK) | (value << 4)
        }
        else {
            (*current_value & light_engine::BLOCK_MASK) | value
        };

        self.regen_mesh.fetch_or(*current_value != final_value, Ordering::Relaxed);

        *current_value = final_value;
    }
}
