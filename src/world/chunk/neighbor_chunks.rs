use std::{cell::RefCell, sync::Arc};

use crate::{math::Vec3i, world::{Chunk, Planet, chunk::{ChunkGetter, neighbor_chunks}}};


pub struct NeighborChunks {
    pub north: ChunkGetter,
    pub south: ChunkGetter,
    pub west: ChunkGetter,
    pub east: ChunkGetter,

    pub northwest: ChunkGetter,
    pub northeast: ChunkGetter,
    pub southwest: ChunkGetter,
    pub southeast: ChunkGetter,

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

            northwest: ChunkGetter::new(None),
            northeast: ChunkGetter::new(None),
            southwest: ChunkGetter::new(None),
            southeast: ChunkGetter::new(None),

            chunk_pos: Vec3i::ZERO,
            first_time: true,
            disposable: false,
        }
    }

    pub fn new_set(planet: &Planet, pos: Vec3i, corners: bool) -> Self {
        let mut neighbor_chunks = Self::new();
        neighbor_chunks.change(planet, pos, corners);

        return neighbor_chunks;
    }

    pub fn change(&mut self, planet: &Planet, pos: Vec3i, corners: bool) {
        if self.chunk_pos != pos || self.first_time {
            self.first_time = false;

            self.dispose();

            self.disposable = false;

            self.north = planet.get_chunk_int(pos.x, pos.y, pos.z - 1);
            self.south = planet.get_chunk_int(pos.x, pos.y, pos.z + 1);
            self.west = planet.get_chunk_int(pos.x - 1, pos.y, pos.z);
            self.east = planet.get_chunk_int(pos.x + 1, pos.y, pos.z);

            if let Some(ref north) = self.north.chunk { north.borrow().lock() }
            if let Some(ref south) = self.south.chunk { south.borrow().lock() }
            if let Some(ref west) = self.west.chunk { west.borrow().lock() }
            if let Some(ref east) = self.east.chunk { east.borrow().lock() }

            if corners {
                self.northwest = planet.get_chunk_int(pos.x - 1, pos.y, pos.z - 1);
                self.northeast = planet.get_chunk_int(pos.x + 1, pos.y, pos.z - 1);
                self.southwest = planet.get_chunk_int(pos.x - 1, pos.y, pos.z + 1);
                self.southeast = planet.get_chunk_int(pos.x + 1, pos.y, pos.z + 1);

                if let Some(ref northwest) = self.northwest.chunk { northwest.borrow().lock() }
                if let Some(ref northeast) = self.northeast.chunk { northeast.borrow().lock() }
                if let Some(ref southwest) = self.southwest.chunk { southwest.borrow().lock() }
                if let Some(ref southeast) = self.southeast.chunk { southeast.borrow().lock() }
            }
        }

        self.chunk_pos = pos;
    }

    pub fn dispose(&mut self) {
        if self.disposable { return }

        if let Some(ref north) = self.north.chunk { north.borrow().unlock() }
        if let Some(ref south) = self.south.chunk { south.borrow().unlock() }
        if let Some(ref west) = self.west.chunk { west.borrow().unlock() }
        if let Some(ref east) = self.east.chunk { east.borrow().unlock() }

        if let Some(ref northwest) = self.northwest.chunk { northwest.borrow().unlock() }
        if let Some(ref northeast) = self.northeast.chunk { northeast.borrow().unlock() }
        if let Some(ref southwest) = self.southwest.chunk { southwest.borrow().unlock() }
        if let Some(ref southeast) = self.southeast.chunk { southeast.borrow().unlock() }

        self.disposable = true;
    }
}

impl Drop for NeighborChunks {
    fn drop(&mut self) {
        self.dispose();
    }
}
