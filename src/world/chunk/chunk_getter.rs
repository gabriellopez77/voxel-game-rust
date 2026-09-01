use std::cell::RefCell;
use std::sync::{Arc, RwLock};
use crate::math::Vec3i;
use crate::world::{Chunk, ChunksManager};


pub struct ChunkGetter {
    pub chunk: Option<Arc<RwLock<Chunk>>>,

    last_chunk_pos: Vec3i,
}

impl ChunkGetter {
    pub fn new() -> Self {
        Self {
            chunk: None,
            last_chunk_pos: Vec3i::ZERO,
        }
    }

    pub fn change(&mut self, chunk_pos: Vec3i, chunks_manager: &ChunksManager) -> &Option<Arc<RwLock<Chunk>>> {
        if self.chunk.is_none() || self.last_chunk_pos != chunk_pos {
            self.chunk = chunks_manager.get_chunk(chunk_pos);
        }

        self.last_chunk_pos = chunk_pos;

        return &self.chunk;
    }
}
