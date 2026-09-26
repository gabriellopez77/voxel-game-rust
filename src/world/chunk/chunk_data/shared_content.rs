use std::sync::atomic::Ordering;

use crate::{
    math::Vec3i,
    world::{
        blocks::{BlockIdState, BlockProperties, block_registry},
        chunk::{
            Chunk,
            chunk_data::{ChunkData, ChunkDataContent},
        },
        light_engine::{self, LightSectionLevel, LightType},
    }
};


pub struct ChunkDataSharedContent {
    pub blocks_id: [u16; Chunk::CHUNK_DATA_SIZE],
    pub blocks_states: [u8; Chunk::CHUNK_DATA_SIZE],
    pub light_levels: [u8; Chunk::CHUNK_DATA_SIZE],

    pub light_sections: [LightSectionLevel; Chunk::SUB_CHUNK_COUNT],
}

impl ChunkDataSharedContent {
    pub fn get_block_properties<'a>(&self, chunk_block: Vec3i) -> &'a BlockProperties {
        block_registry::get().get_properties(self.get_block_id_state(chunk_block))
    }

    pub fn get_block_id_state(&self, chunk_block: Vec3i) -> BlockIdState {
        let index = ChunkData::get_index(chunk_block.x, chunk_block.y, chunk_block.z);

        return BlockIdState::new(self.blocks_id[index], self.blocks_states[index]);
    }

    pub fn get_light(&self, chunk_block: Vec3i, light_type: LightType) -> u8 {
        let index = ChunkData::get_index(chunk_block.x, chunk_block.y, chunk_block.z);

        let value = self.light_levels[index];

        return light_engine::get_level(value, light_type);
    }

    pub fn change_block<'a>(&mut self,
        chunk_block: Vec3i,
        id_state: BlockIdState,
        content: &'a ChunkDataContent
    ) -> BlockIdState {
        let old = self.get_block_id_state(chunk_block);
        self.set_block(chunk_block, id_state, content);

        old
    }

    pub fn set_block(&mut self,
        chunk_block: Vec3i,
        id_state: BlockIdState,
        content: &ChunkDataContent
    ) {
        let index = ChunkData::get_index(chunk_block.x, chunk_block.y, chunk_block.z);

        let current_id = &mut self.blocks_id[index];
        let current_state = &mut self.blocks_states[index];

        let flag = *current_id != id_state.id || *current_state != id_state.state;
        content.flags.fetch_or(flag as u16, Ordering::Relaxed);

        *current_id = id_state.id;
        *current_state = id_state.state;
    }

    pub fn set_light(&mut self,
        chunk_block: Vec3i,
        value: u8,
        light_type: LightType,
        content: &ChunkDataContent
    ) {
        let index = ChunkData::get_index(chunk_block.x, chunk_block.y, chunk_block.z);

        let current_value = &mut self.light_levels[index];

        let final_value = if light_type == LightType::Block {
            (*current_value & light_engine::SKY_MASK) | (value << 4)
        }
        else {
            (*current_value & light_engine::BLOCK_MASK) | value
        };

        content.flags.fetch_or((*current_value != final_value) as u16, Ordering::Relaxed);

        *current_value = final_value;
    }
}
