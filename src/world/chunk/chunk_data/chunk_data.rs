use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, atomic::{AtomicU16, Ordering}};
use crate::{math::Vec3i, utils::SafePtr, world::{Chunk, blocks::{BlockIdState, BlockProperties, BlocksManager}, chunk::chunk_data::{ChunkDataReadBehavior, ChunkDataSharedContent, InternalReadBehavior}, light_engine::{LightSectionLevel, LightType}}};


#[derive(Clone, Copy)]
pub struct ChunkBlockInfo {
    pub id: u16,
}

#[derive(Clone, Copy)]
pub struct ChunkDataFlags(pub(super) u16);

impl ChunkDataFlags {
    pub const REGEN_MESH_FLAG: Self = Self(0b00000001);
    pub const CONTAINS_EMISSIVE_BLOCKS_FLAG: Self = Self(0b00000010);
    pub const LIGHT_COMPUTE_STAGE_FLAG: Self = Self(0b00000100);

    pub fn default() -> u16 { Self::LIGHT_COMPUTE_STAGE_FLAG.0 }

    pub(super) fn contains(self, state: ChunkDataFlags) -> bool { self.0 & state.0 != 0 }
}


pub struct ChunkDataContent {
    pub position: Vec3i,

    pub flags: AtomicU16,

    pub blocks_manager: SafePtr<BlocksManager>,
}


pub struct ChunkData {
    pub(super) content: ChunkDataContent,
    pub(super) shared_content: RwLock<ChunkDataSharedContent>,
}

unsafe impl Sync for ChunkData {}

impl ChunkData {
    /// uses the order: y, x, z
    pub fn get_index(x: i32, y: i32, z: i32) -> usize {
        ((y * Chunk::CHUNK_SIZE.x * Chunk::CHUNK_SIZE.z) + (x * Chunk::CHUNK_SIZE.z) + z) as usize
    }

    pub fn new(position: Vec3i, blocks_manager: SafePtr<BlocksManager>) -> Self {
        Self {
            content: ChunkDataContent {
                position,
                flags: (AtomicU16::new(ChunkDataFlags::default())),
                blocks_manager
            },
            shared_content: RwLock::new(ChunkDataSharedContent {
                blocks_id: [0; Chunk::CHUNK_DATA_SIZE],
                light_levels: [0; Chunk::CHUNK_DATA_SIZE],

                light_sections: [LightSectionLevel::Two; Chunk::SUB_CHUNK_COUNT],
            }),
        }
    }

    pub fn read_guard(&self) -> ChunkDataReadGuard<'_> {
        ChunkDataReadGuard {
            content: &self.content,
            shared_content_lock: self.shared_content.read().unwrap()
        }
    }
    pub fn write_guard(&self) -> ChunkDataWriteGuard<'_> {
        ChunkDataWriteGuard {
            content: &self.content,
            shared_content_lock: self.shared_content.write().unwrap()
        }
    }

    pub fn get_position(&self) -> Vec3i { self.content.position }
    pub fn flag_contains(&self, flag: ChunkDataFlags) -> bool { (self.content.flags.load(Ordering::Relaxed) & flag.0) != 0 }
    pub fn need_regen_mesh(&self) -> bool {
        let flags = ChunkDataFlags { 0: self.content.flags.load(Ordering::Relaxed) };

        flags.contains(ChunkDataFlags::REGEN_MESH_FLAG) && !flags.contains(ChunkDataFlags::LIGHT_COMPUTE_STAGE_FLAG)
    }

    pub fn get_block_properties(&self, chunk_block: Vec3i) -> SafePtr<BlockProperties> {
        self.shared_content.read().unwrap().get_block_properties(chunk_block, &self.content)
    }

    pub fn get_block_info(&self, chunk_block: Vec3i) -> ChunkBlockInfo {
        self.shared_content.read().unwrap().get_block_info(chunk_block)
    }

    pub fn get_light(&self, chunk_block: Vec3i, light_type: LightType) -> u8 {
        self.shared_content.read().unwrap().get_light(chunk_block, light_type)
    }


    pub fn turn_on_flag(&self, flag: ChunkDataFlags) { self.content.flags.fetch_or(flag.0, Ordering::Relaxed); }
    pub fn turn_off_flag(&self, flag: ChunkDataFlags) { self.content.flags.fetch_and(!flag.0, Ordering::Relaxed); }

    // change the block in chunk_block by the id_state and return the old block
    pub fn change_block(&self, chunk_block: Vec3i, id_state: BlockIdState) -> SafePtr<BlockProperties> {
        self.shared_content.write().unwrap().change_block(chunk_block, id_state, &self.content)
    }

    pub fn set_block(&self, chunk_block: Vec3i, id_state: BlockIdState) {
        self.shared_content.write().unwrap().set_block(chunk_block, id_state, &self.content);
    }

    pub fn set_light(&self, chunk_block: Vec3i, value: u8, light_type: LightType) {
        self.shared_content.write().unwrap().set_light(chunk_block, value, light_type, &self.content);
    }
}



pub struct ChunkDataReadGuard<'a> {
    pub(super) content: &'a ChunkDataContent,
    pub(super) shared_content_lock: RwLockReadGuard<'a, ChunkDataSharedContent>,
}

impl<'a> InternalReadBehavior for ChunkDataReadGuard<'a> {
    fn get_content(&self) -> &ChunkDataContent { self.content }
    fn get_shared_content(&self) -> &ChunkDataSharedContent { &self.shared_content_lock }
}

impl<'a> ChunkDataReadBehavior for ChunkDataReadGuard<'a> {}



pub struct ChunkDataWriteGuard<'a> {
    pub(super) content: &'a ChunkDataContent,
    pub(super) shared_content_lock: RwLockWriteGuard<'a, ChunkDataSharedContent>,
}

impl<'a> InternalReadBehavior for ChunkDataWriteGuard<'a> {
    fn get_content(&self) -> &ChunkDataContent { self.content }
    fn get_shared_content(&self) -> &ChunkDataSharedContent { &self.shared_content_lock }
}

impl<'a> ChunkDataReadBehavior for ChunkDataWriteGuard<'a> {}

impl<'a> ChunkDataWriteGuard<'a> {
    pub fn change_block(&mut self, chunk_block: Vec3i, id_state: BlockIdState) -> SafePtr<BlockProperties> {
        self.shared_content_lock.change_block(chunk_block, id_state, &self.content)
    }

    pub fn set_block(&mut self, chunk_block: Vec3i, id_state: BlockIdState) {
        self.shared_content_lock.set_block(chunk_block, id_state, &self.content);
    }

    pub fn set_light(&mut self, chunk_block: Vec3i, value: u8, light_type: LightType) {
        self.shared_content_lock.set_light(chunk_block, value, light_type, &self.content);
    }

    pub fn turn_on_flag(&self, flag: ChunkDataFlags) { self.content.flags.fetch_or(flag.0, Ordering::Relaxed); }
    pub fn turn_off_flag(&self, flag: ChunkDataFlags) { self.content.flags.fetch_and(!flag.0, Ordering::Relaxed); }
}
