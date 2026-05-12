use fastnoise_lite::FastNoiseLite;
use rand::RngExt;
use crate::math;
use crate::math::Vec3i;
use crate::world::Chunk;


pub struct WorldGen {
    noise: FastNoiseLite
}

impl WorldGen {
    pub fn new() -> Self { Self { noise: FastNoiseLite::new() } }

    pub fn gen_data(&self, chunk_pos: Vec3i , data: &mut [u16; Chunk::CHUNK_DATA_SIZE]) {
        let mut rng = rand::rng();

        for x in 0..Chunk::CHUNK_SIZE.x {
        for y in 0..Chunk::CHUNK_SIZE.y {
        for z in 0..Chunk::CHUNK_SIZE.z {
            let index = math::get_index(x, y, z);
            let current_block =  &mut data[index];

            if y < 60 {
                *current_block = 1;
            }
            else if y == 60 {
                if rng.random_range(0..=100) < 50 {
                    *current_block = 1;
                }
            }
            
        }
        }
        }
    }
}