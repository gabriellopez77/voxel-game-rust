use std::{cell::RefCell, sync::Arc};

use crate::{math::Vec3i, world::{Chunk, Planet, chunk::{ChunkGetter, neighbor_chunks}}};


pub struct NeighborChunks {
    pub north: ChunkGetter,
    pub south: ChunkGetter,
    pub west: ChunkGetter,
    pub east: ChunkGetter,

    chunk_pos: Vec3i,
    first_time: bool,
    disposable: bool,
}

impl NeighborChunks {
    pub fn new() -> Self {
        Self {
            north: ChunkGetter::new(None),
            south: ChunkGetter::new(None),
            west: ChunkGetter::new(None),
            east: ChunkGetter::new(None),

            chunk_pos: Vec3i::ZERO,
            first_time: true,
            disposable: false,
        }
    }

    pub fn new_set(planet: &Planet, pos: Vec3i) -> Self {
        let mut neighbor_chunks = Self::new();
        neighbor_chunks.change(planet, pos);

        return neighbor_chunks;
    }

    pub fn change(&mut self, planet: &Planet, pos: Vec3i) {
        if self.chunk_pos != pos || self.first_time {
            self.first_time = false;
            
            self.dispose();

            self.disposable = false;
            
            self.north = planet.get_chunk_int(pos.x, pos.y, pos.z - 1);
            self.south = planet.get_chunk_int(pos.x, pos.y, pos.z + 1);
            self.west = planet.get_chunk_int(pos.x - 1, pos.y, pos.z);
            self.east = planet.get_chunk_int(pos.x + 1, pos.y, pos.z);
            
            if self.north.exists() { self.north.get().borrow().lock(); }
            if self.south.exists() { self.south.get().borrow().lock() }
            if self.west.exists() { self.west.get().borrow().lock() }
            if self.east.exists() { self.east.get().borrow().lock() }
        }
        
        self.chunk_pos = pos;
    }

    pub fn dispose(&mut self) {
        if self.disposable { return }

        if self.north.exists() { self.north.get().borrow().unlock() }
        if self.south.exists() { self.south.get().borrow().unlock() }
        if self.west.exists() { self.west.get().borrow().unlock() }
        if self.east.exists() { self.east.get().borrow().unlock() }

        self.disposable = true;
    }
}

impl Drop for NeighborChunks {
    fn drop(&mut self) {
        self.dispose();
    }
}