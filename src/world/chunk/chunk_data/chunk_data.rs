use std::sync::atomic::{AtomicU16, Ordering};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};


use crate::{
    math::Vec3i,
    world::{
        blocks::{BlockIdState, BlockProperties},
        chunk::{
            chunk_data::{
                ChunkDataReadBehavior,
                ChunkDataSharedContent,
                InternalReadBehavior,
            },
            Chunk,
        },
        light_engine::{LightSectionLevel, LightType},
    }
};


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
}


pub struct ChunkData {
    pub(super) content: ChunkDataContent,
    pub(super) shared_content: RwLock<ChunkDataSharedContent>,
}

unsafe impl Sync for ChunkData {}

impl<'a> InternalReadBehavior<'a, RwLockReadGuard<'a, ChunkDataSharedContent>> for ChunkData {
    fn get_content(&self) -> &ChunkDataContent { &self.content }
    fn get_shared_content(&self) -> RwLockReadGuard<'_, ChunkDataSharedContent> {
        self.shared_content.read()
    }
}

impl<'a> ChunkDataReadBehavior<'a, RwLockReadGuard<'a, ChunkDataSharedContent>> for ChunkData {}

impl ChunkData {
    /// uses the order: y, x, z
    pub fn get_index(x: i32, y: i32, z: i32) -> usize {
        ((y * Chunk::CHUNK_SIZE.x * Chunk::CHUNK_SIZE.z) + (x * Chunk::CHUNK_SIZE.z) + z) as usize
    }

    pub fn new(position: Vec3i) -> Self {
        Self {
            content: ChunkDataContent {
                position,
                flags: (AtomicU16::new(ChunkDataFlags::default())),
            },
            shared_content: RwLock::new(ChunkDataSharedContent {
                blocks_id: [0; Chunk::CHUNK_DATA_SIZE],
                blocks_states: [0; Chunk::CHUNK_DATA_SIZE],
                light_levels: [0; Chunk::CHUNK_DATA_SIZE],

                light_sections: [LightSectionLevel::Two; Chunk::SUB_CHUNK_COUNT],
            }),
        }
    }

    pub fn read_guard(&self) -> ChunkDataReadGuard<'_> {
        ChunkDataReadGuard {
            content: &self.content,
            shared_content_lock: self.shared_content.read()
        }
    }
    pub fn write_guard(&self) -> ChunkDataWriteGuard<'_> {
        ChunkDataWriteGuard {
            content: &self.content,
            shared_content_lock: self.shared_content.write()
        }
    }

    pub fn turn_on_flag(&self, flag: ChunkDataFlags) { self.content.flags.fetch_or(flag.0, Ordering::Relaxed); }
    pub fn turn_off_flag(&self, flag: ChunkDataFlags) { self.content.flags.fetch_and(!flag.0, Ordering::Relaxed); }

    // change the block in chunk_block by the id_state and return the old block
    pub fn change_block(&self, chunk_block: Vec3i, id_state: BlockIdState) -> BlockIdState {
        self.shared_content.write().change_block(chunk_block, id_state, &self.content)
    }

    pub fn set_block(&self, chunk_block: Vec3i, id_state: BlockIdState) {
        self.shared_content.write().set_block(chunk_block, id_state, &self.content);
    }

    pub fn set_light(&self, chunk_block: Vec3i, value: u8, light_type: LightType) {
        self.shared_content.write().set_light(chunk_block, value, light_type, &self.content);
    }
}



pub struct ChunkDataReadGuard<'a> {
    pub(super) content: &'a ChunkDataContent,
    pub(super) shared_content_lock: RwLockReadGuard<'a, ChunkDataSharedContent>,
}

impl<'a> InternalReadBehavior<'a, &'a ChunkDataSharedContent> for ChunkDataReadGuard<'a> {
    fn get_content(&self) -> &ChunkDataContent { self.content }
    fn get_shared_content(&'a self) -> &'a ChunkDataSharedContent { &self.shared_content_lock }
}

impl<'a> ChunkDataReadBehavior<'a, &'a ChunkDataSharedContent> for ChunkDataReadGuard<'a> {}



pub struct ChunkDataWriteGuard<'a> {
    pub(super) content: &'a ChunkDataContent,
    pub(super) shared_content_lock: RwLockWriteGuard<'a, ChunkDataSharedContent>,
}

impl<'a> InternalReadBehavior<'a, &'a ChunkDataSharedContent> for ChunkDataWriteGuard<'a> {
    fn get_content(&self) -> &ChunkDataContent { self.content }
    fn get_shared_content(&'a self) -> &'a ChunkDataSharedContent { &self.shared_content_lock }
}

impl<'a> ChunkDataReadBehavior<'a, &'a ChunkDataSharedContent> for ChunkDataWriteGuard<'a> {}

impl<'a> ChunkDataWriteGuard<'a> {
    pub fn change_block(&mut self, chunk_block: Vec3i, id_state: BlockIdState) -> BlockIdState {
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
