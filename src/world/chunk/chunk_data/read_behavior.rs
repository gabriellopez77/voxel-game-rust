use std::sync::atomic::Ordering;
use std::ops::Deref;

use crate::{
    math::Vec3i,
    world::{
        blocks::{BlockBehaviors, BlockIdState, BlockProperties, block_registry},
        chunk::chunk_data::{
            ChunkDataContent,
            ChunkDataFlags,
            ChunkDataSharedContent,
        },
        light_engine::LightType,
    },
};


pub(super) trait InternalReadBehavior<'a, T: Deref<Target = ChunkDataSharedContent>> {
    fn get_content(&self) -> &ChunkDataContent;
    fn get_shared_content(&'a self) -> T;
}

#[allow(private_bounds)]
pub trait ChunkDataReadBehavior<'a, T> : InternalReadBehavior<'a, T>
where T: Deref<Target = ChunkDataSharedContent> {
    fn get_block_properties(&'a self, chunk_block: Vec3i) -> &'a BlockProperties {
        self.get_shared_content().get_block_properties(chunk_block)
    }

    fn get_block_behaviors(&'a self, chunk_block: Vec3i) -> &'a dyn BlockBehaviors {
        block_registry::get().get(self.get_shared_content().get_block_id_state(chunk_block))
    }

    fn get_block_id_state(&'a self, chunk_block: Vec3i) -> BlockIdState {
        self.get_shared_content().get_block_id_state(chunk_block)
    }

    fn get_light(&'a self, chunk_block: Vec3i, light_type: LightType) -> u8 {
        self.get_shared_content().get_light(chunk_block, light_type)
    }

    fn get_position(&self) -> Vec3i {
        self.get_content().position
    }

    fn flag_contains(&self, flag: ChunkDataFlags) -> bool {
        (self.get_content().flags.load(Ordering::Relaxed) & flag.0) != 0
    }

    fn need_regen_mesh(&self) -> bool {
        let flags = ChunkDataFlags { 0: self.get_content().flags.load(Ordering::Relaxed) };

        flags.contains(ChunkDataFlags::REGEN_MESH_FLAG) && !flags.contains(ChunkDataFlags::LIGHT_COMPUTE_STAGE_FLAG)
    }
}
