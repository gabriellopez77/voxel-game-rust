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
}

impl NeighborChunks {
    pub fn new(planet: &Planet, pos: Vec3i, corners: bool) -> Self {
        let mut neighbors = Self {
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
        };

        neighbors.change(planet, pos, corners);
        neighbors
    }

    pub fn change(&mut self, planet: &Planet, pos: Vec3i, corners: bool) {
        if self.chunk_pos != pos || self.first_time {
            self.first_time = false;

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