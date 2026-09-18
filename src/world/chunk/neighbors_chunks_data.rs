use std::{collections::HashMap, sync::Arc};
use parking_lot::RwLock;

use crate::{
    math::Vec3i,
    world::{
        chunk::{
            Chunk,
            chunk_data::ChunkData,
        },
        ChunksManager,
    }
};


pub struct NeighborsChunksData {
    pub north: Option<Arc<ChunkData>>,
    pub south: Option<Arc<ChunkData>>,
    pub west: Option<Arc<ChunkData>>,
    pub east: Option<Arc<ChunkData>>,

    pub northwest: Option<Arc<ChunkData>>,
    pub northeast: Option<Arc<ChunkData>>,
    pub southwest: Option<Arc<ChunkData>>,
    pub southeast: Option<Arc<ChunkData>>,

    chunk_pos: Vec3i,
    first_time: bool,
}

impl NeighborsChunksData {
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
        Self::new_from_map(chunks_manager.chunks.clone(), pos, corners)
    }

    pub fn new_from_map(chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<Chunk>>>>>, pos: Vec3i, corners: bool) -> Self {
        let mut data_northwest: Option<Arc<ChunkData>> = None;
        let mut data_northeast: Option<Arc<ChunkData>> = None;
        let mut data_southwest: Option<Arc<ChunkData>> = None;
        let mut data_southeast: Option<Arc<ChunkData>> = None;

        if corners {
            data_northwest = Self::set_data(&chunks_map, pos.x - 1, pos.y, pos.z - 1);
            data_northeast = Self::set_data(&chunks_map, pos.x + 1, pos.y, pos.z - 1);
            data_southwest = Self::set_data(&chunks_map, pos.x - 1, pos.y, pos.z + 1);
            data_southeast = Self::set_data(&chunks_map, pos.x + 1, pos.y, pos.z + 1);
        }

        Self {
            north: Self::set_data(&chunks_map, pos.x, pos.y, pos.z - 1),
            south: Self::set_data(&chunks_map, pos.x, pos.y, pos.z + 1),
            west: Self::set_data(&chunks_map, pos.x - 1, pos.y, pos.z),
            east: Self::set_data(&chunks_map, pos.x + 1, pos.y, pos.z),

            northwest: data_northwest,
            northeast: data_northeast,
            southwest: data_southwest,
            southeast: data_southeast,

            chunk_pos: Vec3i::ZERO,
            first_time: true,
        }
    }

    pub fn change_from_map(&mut self,
        chunks_map: Arc<RwLock<HashMap<Vec3i, Option<Arc<Chunk>>>>>,
        pos: Vec3i,
        corners: bool
    ) {
        if self.chunk_pos != pos || self.first_time {
            self.first_time = false;

            self.north = Self::set_data(&chunks_map, pos.x, pos.y, pos.z - 1);
            self.south = Self::set_data(&chunks_map, pos.x, pos.y, pos.z + 1);
            self.west = Self::set_data(&chunks_map, pos.x - 1, pos.y, pos.z);
            self.east = Self::set_data(&chunks_map, pos.x + 1, pos.y, pos.z);

            if corners {
                self.northwest = Self::set_data(&chunks_map, pos.x - 1, pos.y, pos.z - 1);
                self.northeast = Self::set_data(&chunks_map, pos.x + 1, pos.y, pos.z - 1);
                self.southwest = Self::set_data(&chunks_map, pos.x - 1, pos.y, pos.z + 1);
                self.southeast = Self::set_data(&chunks_map, pos.x + 1, pos.y, pos.z + 1);
            }
        }

        self.chunk_pos = pos;
    }

    pub fn change(&mut self, chunks_manager: &ChunksManager, pos: Vec3i, corners: bool) {
        self.change_from_map(chunks_manager.chunks.clone(), pos, corners);
    }

    fn set_data(chunks_map: &Arc<RwLock<HashMap<Vec3i, Option<Arc<Chunk>>>>>, x: i32, y: i32, z: i32) -> Option<Arc<ChunkData>> {
        if let Some(chunk1) = chunks_map.read().get(&Vec3i::new(x, y, z)) && let Some(chunk2) = chunk1 {
            return Some(chunk2.data.clone());
        }

        return None;
    }
}
