use std::{cell::RefCell, sync::Arc};
use crate::math::Vec3i;
use crate::world::{Chunk, Planet};


pub struct ChunkGetter {
    pub chunk: Option<Arc<RefCell<Chunk>>>,

    last_chunk_pos: Vec3i,
}

impl ChunkGetter {
    pub fn new() -> Self {
        Self {
            chunk: None,
            last_chunk_pos: Vec3i::ZERO,
        }
    }

    pub fn change(&mut self, chunk_pos: Vec3i, planet: &Planet) -> &mut Option<Arc<RefCell<Chunk>>> {
        if self.chunk.is_none() || self.last_chunk_pos != chunk_pos {
            self.chunk = planet.get_chunk(chunk_pos);
        }

        self.last_chunk_pos = chunk_pos;

        return &mut self.chunk;
    }
}
