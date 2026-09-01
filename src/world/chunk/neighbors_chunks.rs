use std::sync::{Arc, RwLock};

use crate::{math::Vec3i, world::{Chunk}};
use crate::world::ChunksManager;

pub struct NeighborsChunks {
    pub north: Option<Arc<RwLock<Chunk>>>,
    pub south: Option<Arc<RwLock<Chunk>>>,
    pub west: Option<Arc<RwLock<Chunk>>>,
    pub east: Option<Arc<RwLock<Chunk>>>,

    pub northwest: Option<Arc<RwLock<Chunk>>>,
    pub northeast: Option<Arc<RwLock<Chunk>>>,
    pub southwest: Option<Arc<RwLock<Chunk>>>,
    pub southeast: Option<Arc<RwLock<Chunk>>>,

    chunk_pos: Vec3i,
    first_time: bool,
}

impl NeighborsChunks {
    pub const EMPTY: Self = Self {
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

    pub fn new(chunks_manager: &ChunksManager, pos: Vec3i, corners: bool) -> Self {
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

        neighbors.change(chunks_manager, pos, corners);
        neighbors
    }

    pub fn change(&mut self, chunks_manager: &ChunksManager, pos: Vec3i, corners: bool) {
        if self.chunk_pos != pos || self.first_time {
            self.first_time = false;

            self.north = chunks_manager.get_chunki(pos.x, pos.y, pos.z - 1);
            self.south = chunks_manager.get_chunki(pos.x, pos.y, pos.z + 1);
            self.west = chunks_manager.get_chunki(pos.x - 1, pos.y, pos.z);
            self.east = chunks_manager.get_chunki(pos.x + 1, pos.y, pos.z);

            if corners {
                self.northwest = chunks_manager.get_chunki(pos.x - 1, pos.y, pos.z - 1);
                self.northeast = chunks_manager.get_chunki(pos.x + 1, pos.y, pos.z - 1);
                self.southwest = chunks_manager.get_chunki(pos.x - 1, pos.y, pos.z + 1);
                self.southeast = chunks_manager.get_chunki(pos.x + 1, pos.y, pos.z + 1);
            }
        }

        self.chunk_pos = pos;
    }
}
