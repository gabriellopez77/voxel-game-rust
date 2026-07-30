use std::{cell::RefCell, sync::{Arc, RwLock}};

use crate::{math::Vec3i, world::{Chunk, Planet, chunk::ChunkData}};


pub struct NeighborsData {
    pub north: Option<Arc<RwLock<ChunkData>>>,
    pub south: Option<Arc<RwLock<ChunkData>>>,
    pub west: Option<Arc<RwLock<ChunkData>>>,
    pub east: Option<Arc<RwLock<ChunkData>>>,

    pub northwest: Option<Arc<RwLock<ChunkData>>>,
    pub northeast: Option<Arc<RwLock<ChunkData>>>,
    pub southwest: Option<Arc<RwLock<ChunkData>>>,
    pub southeast: Option<Arc<RwLock<ChunkData>>>,
}

impl NeighborsData {
    pub fn new(planet: &mut Planet, pos: Vec3i) -> Self {
        let mut data_north: Option<Arc<RwLock<ChunkData>>> = None;
        let mut data_south: Option<Arc<RwLock<ChunkData>>> = None;
        let mut data_west: Option<Arc<RwLock<ChunkData>>> = None;
        let mut data_east: Option<Arc<RwLock<ChunkData>>> = None;

        let mut data_northwest: Option<Arc<RwLock<ChunkData>>> = None;
        let mut data_northeast: Option<Arc<RwLock<ChunkData>>> = None;
        let mut data_southwest: Option<Arc<RwLock<ChunkData>>> = None;
        let mut data_southeast: Option<Arc<RwLock<ChunkData>>> = None;

        let north = planet.get_chunk_int(pos.x, pos.y, pos.z - 1);
        let south = planet.get_chunk_int(pos.x, pos.y, pos.z + 1);
        let west = planet.get_chunk_int(pos.x - 1, pos.y, pos.z);
        let east = planet.get_chunk_int(pos.x + 1, pos.y, pos.z);

        let northwest = planet.get_chunk_int(pos.x - 1, pos.y, pos.z - 1);
        let northeast = planet.get_chunk_int(pos.x + 1, pos.y, pos.z - 1);
        let southwest = planet.get_chunk_int(pos.x - 1, pos.y, pos.z + 1);
        let southeast = planet.get_chunk_int(pos.x + 1, pos.y, pos.z + 1);

        Self::set_data(&north, &mut data_north);
        Self::set_data(&south, &mut data_south);
        Self::set_data(&west, &mut data_west);
        Self::set_data(&east, &mut data_east);

        Self::set_data(&northwest, &mut data_northwest);
        Self::set_data(&northeast, &mut data_northeast);
        Self::set_data(&southwest, &mut data_southwest);
        Self::set_data(&southeast, &mut data_southeast);

        Self {
            north: data_north,
            south: data_south,
            west: data_west,
            east: data_east,

            northwest: data_northwest,
            northeast: data_northeast,
            southwest: data_southwest,
            southeast: data_southeast,
        }
    }

    fn set_data(chunk: &Option<Arc<RefCell<Chunk>>>, chunk_data: &mut Option<Arc<RwLock<ChunkData>>>) {
        if let Some(chunk) = chunk {
            *chunk_data = Some(chunk.borrow().chunk_data.clone());
        }
    }
}
