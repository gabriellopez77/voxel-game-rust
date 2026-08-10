use std::{cell::RefCell, sync::Arc};

use crate::{math::Vec3i, world::{Chunk, Planet}};


pub struct NeighborChunks {
    pub north: Option<Arc<RefCell<Chunk>>>,
    pub south: Option<Arc<RefCell<Chunk>>>,
    pub west: Option<Arc<RefCell<Chunk>>>,
    pub east: Option<Arc<RefCell<Chunk>>>,

    pub northwest: Option<Arc<RefCell<Chunk>>>,
    pub northeast: Option<Arc<RefCell<Chunk>>>,
    pub southwest: Option<Arc<RefCell<Chunk>>>,
    pub southeast: Option<Arc<RefCell<Chunk>>>,

    chunk_pos: Vec3i,
    first_time: bool,
    disposable: bool,
}

impl NeighborChunks {
    pub fn new() -> Self {
        Self {
            north: None,
            south: None,
            west: None,
            east: None,

            northwest: None,
            northeast: None,
            southwest: None,
            southeast: None,

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

            self.disposable = false;

            self.north = planet.get_chunk_int(pos.x, pos.y, pos.z - 1);
            self.south = planet.get_chunk_int(pos.x, pos.y, pos.z + 1);
            self.west = planet.get_chunk_int(pos.x - 1, pos.y, pos.z);
            self.east = planet.get_chunk_int(pos.x + 1, pos.y, pos.z);

            if corners {
                self.northwest = planet.get_chunk_int(pos.x - 1, pos.y, pos.z - 1);
                self.northeast = planet.get_chunk_int(pos.x + 1, pos.y, pos.z - 1);
                self.southwest = planet.get_chunk_int(pos.x - 1, pos.y, pos.z + 1);
                self.southeast = planet.get_chunk_int(pos.x + 1, pos.y, pos.z + 1);
            }
        }

        self.chunk_pos = pos;
    }
}