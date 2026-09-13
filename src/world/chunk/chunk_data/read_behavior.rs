use std::sync::atomic::Ordering;

use crate::{math::Vec3i, utils::SafePtr, world::{blocks::BlockProperties, chunk::chunk_data::{ChunkBlockInfo, ChunkDataContent, ChunkDataFlags, ChunkDataSharedContent}, light_engine::LightType}};


pub(super) trait InternalReadBehavior {
    fn get_content(&self) -> &ChunkDataContent;
    fn get_shared_content(&self) -> &ChunkDataSharedContent;
}

pub trait ChunkDataReadBehavior : InternalReadBehavior {
    fn get_block_properties(&self, chunk_block: Vec3i) -> SafePtr<BlockProperties> {
        self.get_shared_content().get_block_properties(chunk_block, self.get_content())
    }

    fn get_block_info(&self, chunk_block: Vec3i) -> ChunkBlockInfo {
        self.get_shared_content().get_block_info(chunk_block)
    }

    fn get_light(&self, chunk_block: Vec3i, light_type: LightType) -> u8 {
        self.get_shared_content().get_light(chunk_block, light_type)
    }

    fn get_position(&self) -> Vec3i {
        self.get_content().position
    }

    fn flag_contains(&self, flag: ChunkDataFlags) -> bool { (self.get_content().flags.load(Ordering::Relaxed) & flag.0) != 0 }

    fn need_regen_mesh(&self) -> bool {
        let flags = ChunkDataFlags { 0: self.get_content().flags.load(Ordering::Relaxed) };

        flags.contains(ChunkDataFlags::REGEN_MESH_FLAG) && !flags.contains(ChunkDataFlags::LIGHT_COMPUTE_STAGE_FLAG)
    }
}
