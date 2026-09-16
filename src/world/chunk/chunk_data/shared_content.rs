use std::sync::atomic::Ordering;

use crate::{
    math::Vec3i,
    utils::SafePtr,
    world::{
        chunk::{
            Chunk,
            chunk_data::{ChunkBlockInfo, ChunkData, ChunkDataContent},
        },
        blocks::{BlockIdState, BlockProperties},
        light_engine::{self, LightSectionLevel, LightType},
    }
};


pub struct ChunkDataSharedContent {
    pub blocks_id: [u16; Chunk::CHUNK_DATA_SIZE],
    pub light_levels: [u8; Chunk::CHUNK_DATA_SIZE],

    pub light_sections: [LightSectionLevel; Chunk::SUB_CHUNK_COUNT],
}

impl ChunkDataSharedContent {
    pub fn get_block_properties(&self,
        chunk_block: Vec3i,
        content: &ChunkDataContent
    ) -> SafePtr<BlockProperties> {
        content.blocks_manager.get_properties_from_block_info(self.get_block_info(chunk_block))
    }

    pub fn get_block_info(&self, chunk_block: Vec3i) -> ChunkBlockInfo {
        let index = ChunkData::get_index(chunk_block.x, chunk_block.y, chunk_block.z);

        return ChunkBlockInfo {
            id: self.blocks_id[index],
        };
    }

    pub fn get_light(&self, chunk_block: Vec3i, light_type: LightType) -> u8 {
        let index = ChunkData::get_index(chunk_block.x, chunk_block.y, chunk_block.z);

        let value = self.light_levels[index];

        return light_engine::get_level(value, light_type);
    }

    pub fn change_block(&mut self,
        chunk_block: Vec3i,
        id_state: BlockIdState,
        content: &ChunkDataContent
    ) -> SafePtr<BlockProperties> {
        let old = self.get_block_properties(chunk_block, content);
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

        content.flags.fetch_or((*current_id != id_state.id) as u16, Ordering::Relaxed);

        *current_id = id_state.id;
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
