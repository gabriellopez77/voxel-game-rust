use fastnoise_lite::{FastNoiseLite, NoiseType};

use rand::rngs::ThreadRng;
use crate::math;
use crate::math::Vec3i;
use crate::world::Chunk;


pub struct WorldGen {
    noise: FastNoiseLite,
    rand: ThreadRng,
}

impl WorldGen {
    pub fn new() -> Self { 
        let mut noise = FastNoiseLite::new();
        noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        noise.set_seed(Some(-444));
        noise.set_frequency(Some(0.01));
        noise.set_fractal_octaves(Some(3));
        noise.set_fractal_lacunarity(Some(2.17));
        noise.set_fractal_gain(Some(0.62));

        Self { 
            noise,
            rand: rand::rng(),
        }

        
    }

    pub fn gen_data(&self, chunk_pos: Vec3i, data: &mut [u16; Chunk::CHUNK_DATA_SIZE]) {
        let start_x = chunk_pos.x * Chunk::CHUNK_SIZE.x;
        let start_z = chunk_pos.z * Chunk::CHUNK_SIZE.z;

        const SURFACE_HEIGHT:i32 = 40;
        const WATER_HEIGHT:i32 = 45;

        for x in 0..Chunk::CHUNK_SIZE.x {
            for z in 0..Chunk::CHUNK_SIZE.z {
                let mut surface_height = SURFACE_HEIGHT;

                let noise_x = x + start_x;
                let noise_z = z + start_z;
                let mut ridge = 0.0f32;
                let mut p = 1.0f32;

                for i in 0..6 {
                    let h = self.noise.get_noise_2d(
                        noise_x as f32 * p / 8.0,
                        noise_z as f32 * p / 10.0
                    );

                    ridge += h / p;
                    p *= 2.0;
                }

                surface_height += (ridge * ridge * ridge * ridge * 11.0) as i32;

                for y in 0..Chunk::CHUNK_SIZE.y {
                    let index = math::get_index(x, y, z);
                    let current_block =  &mut data[index];

                    if y <= surface_height {
                        *current_block = 1;
                    }
//
                    //if y < 60 {
                    //    *current_block = 1;
                    //}
                    //else if y == 60 {
                    //    if self.chance(0, 100) < 50 {
                    //        *current_block = 1;
                    //    }
                    //}
                
                }
            }
        }
    }

    pub fn chance(&self, min: i32, max: i32) -> i32 { rand::random_range(min..=max) }
}