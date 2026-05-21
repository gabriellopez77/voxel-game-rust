use std::{cell::RefCell, sync::Arc};

use crate::world::Chunk;


pub struct ChunkGetter {
    pub chunk: Option<Arc<RefCell<Chunk>>>
}

impl ChunkGetter {
    pub fn new(chunk: Option<Arc<RefCell<Chunk>>>) -> Self { Self { chunk } }

    pub fn exists(&self) -> bool { self.chunk.is_some() }

    pub fn get(&self) -> Arc<RefCell<Chunk>> { self.chunk.as_ref().unwrap().clone() }
}