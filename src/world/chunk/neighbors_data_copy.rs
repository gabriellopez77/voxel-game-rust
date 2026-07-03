use std::{cell::RefCell, sync::Arc};

use crate::{math::Vec3i, utils::ObjectPool, world::{Chunk, Planet, chunk::ChunkData}};


pub struct NeighborsDataCopy {
    pub north: Option<Box<RefCell<ChunkData>>>,
    pub south: Option<Box<RefCell<ChunkData>>>,
    pub west: Option<Box<RefCell<ChunkData>>>,
    pub east: Option<Box<RefCell<ChunkData>>>,

    pub northwest: Option<Box<RefCell<ChunkData>>>,
    pub northeast: Option<Box<RefCell<ChunkData>>>,
    pub southwest: Option<Box<RefCell<ChunkData>>>,
    pub southeast: Option<Box<RefCell<ChunkData>>>,
}

impl NeighborsDataCopy {
    pub fn new(planet: &mut Planet, pos: Vec3i) -> Self {
        let mut data_north: Option<Box<RefCell<ChunkData>>> = None;
        let mut data_south: Option<Box<RefCell<ChunkData>>> = None;
        let mut data_west: Option<Box<RefCell<ChunkData>>> = None;
        let mut data_east: Option<Box<RefCell<ChunkData>>> = None;

        let mut data_northwest: Option<Box<RefCell<ChunkData>>> = None;
        let mut data_northeast: Option<Box<RefCell<ChunkData>>> = None;
        let mut data_southwest: Option<Box<RefCell<ChunkData>>> = None;
        let mut data_southeast: Option<Box<RefCell<ChunkData>>> = None;

        let north = planet.get_chunk_int(pos.x, pos.y, pos.z - 1);
        let south = planet.get_chunk_int(pos.x, pos.y, pos.z + 1);
        let west = planet.get_chunk_int(pos.x - 1, pos.y, pos.z);
        let east = planet.get_chunk_int(pos.x + 1, pos.y, pos.z);

        let northwest = planet.get_chunk_int(pos.x - 1, pos.y, pos.z - 1);
        let northeast = planet.get_chunk_int(pos.x + 1, pos.y, pos.z - 1);
        let southwest = planet.get_chunk_int(pos.x - 1, pos.y, pos.z + 1);
        let southeast = planet.get_chunk_int(pos.x + 1, pos.y, pos.z + 1);

        let pool = &mut planet.chunk_data_pool;

        Self::set_data(&north, pool, &mut data_north);
        Self::set_data(&south, pool, &mut data_south);
        Self::set_data(&west, pool, &mut data_west);
        Self::set_data(&east, pool, &mut data_east);

        Self::set_data(&northwest, pool, &mut data_northwest);
        Self::set_data(&northeast, pool, &mut data_northeast);
        Self::set_data(&southwest, pool, &mut data_southwest);
        Self::set_data(&southeast, pool, &mut data_southeast);

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

    pub fn restore(self, pool: &mut ObjectPool<Box<RefCell<ChunkData>>>) {
        if let Some(data) = self.north { pool.restore(data); }
        if let Some(data) = self.south { pool.restore(data); }
        if let Some(data) = self.west { pool.restore(data); }
        if let Some(data) = self.east { pool.restore(data); }

        if let Some(data) = self.northwest { pool.restore(data); }
        if let Some(data) = self.northeast { pool.restore(data); }
        if let Some(data) = self.southwest { pool.restore(data); }
        if let Some(data) = self.southeast { pool.restore(data); }
    }

    fn set_data(chunk: & Option<Arc<RefCell<Chunk>>>, pool: &mut ObjectPool<Box<RefCell<ChunkData>>>,
                chunk_data: &mut Option<Box<RefCell<ChunkData>>>) {
        if let Some(chunk) = chunk {
            let data = pool.get();

            if let Some(allocated_data) = data {
                chunk.borrow().chunk_data.copy_to(&mut allocated_data.borrow_mut());
                *chunk_data = Some(allocated_data);
            }
            else {
                *chunk_data = Some(Box::new(RefCell::new(chunk.borrow().chunk_data.clone())))
            }
        }
    }
}
