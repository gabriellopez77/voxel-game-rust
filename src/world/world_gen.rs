use fastnoise_lite::{FastNoiseLite, NoiseType};

use rand::rngs::ThreadRng;
use crate::math;
use crate::math::Vec3i;
use crate::world::Chunk;
use crate::world::blocks::{BlocksManager, blocks_manager};
use crate::world::chunk::ChunkData;


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

    pub fn gen_data(&self, chunk_pos: Vec3i, data: &mut ChunkData, blocks_manager: &BlocksManager) {
        fn chance(min: i32, max: i32) -> i32 { rand::random_range(min..=max) }
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
                    let current_block = Vec3i::new(x, y, z);

                    if y == 0 {
                        data.set_block(current_block, &blocks_manager.bedrock);
                    }

                    // surface features
                    if y > surface_height {
                        data.set_block(current_block, &blocks_manager.air);

                        if (y < WATER_HEIGHT) {
                    //        //if (y == 49) {
                    //        //    if (chance(0..100) < 2)
                    //        //        data.set_block(current_block, bLOCKS_manager::LILY_PAD);
                    //        //}
                    //        //else
                                data.set_block(current_block, &blocks_manager.water_block);
                        }

                    //    if (y == surface_height + 1) {
                    //        if (y >= 100) {
                    //            data.set_block(current_block, blocks_manager.SNOW_LAYER);
                    //        }
                    //        else if (y >= WATER_HEIGHT + 3 && y <= 81) {
                    //            if (chance(0, 100) < 20) {
                    //                data.set_block(current_block, blocks_manager.SHORT_GRASS);
                    //            }
                    //            else if (chance(0, 100) < 1) {
                    //                data.set_block(current_block, blocks_manager.MUSHROOM_BLUE_GROUP);
                    //            }

                    //            else if (chance(0, 1000) < 12) {
                    //                data.set_block(current_block, blocks_manager.RED_FLOWER);
                    //            }
                    //            else if (chance(0, 1000) < 12) {
                    //                data.set_block(current_block, blocks_manager.YELLOW_FLOWER);
                    //            }

                    //            //else if (chance(0, 1000) < 50) {
                    //            //    treesPos.Add(currentBlock);
                    //            //}
                    //        }
                    //        else
                    //        {
                    //            if (y >= WATER_HEIGHT && y <= WATER_HEIGHT + 3) {
                    //                if (chance(0, 100) < 2) {
                    //                    data.set_block(current_block, blocks_manager.DEAD_BUSH);
                    //                }
                    //            }
                    //        }
                    //    }
                    }
//
                    // Ground
                    else {
                        if (surface_height > 80) {
                            if (y == surface_height || y == surface_height - 1 || y == surface_height - 2) && y > 100 {
                                if chance(0, 100) < 2 { data.set_block(current_block, &blocks_manager.ice_block) }
                                else {
                                    data.set_block(current_block, &blocks_manager.snow_block);
                                }
                            }
                            else if (chance(0, 100) < 20) { data.set_block(current_block, &blocks_manager.cobblestone) }
                            else { data.set_block(current_block, &blocks_manager.stone) }
                        }

                        else if y == surface_height {
                            if surface_height <= WATER_HEIGHT + 1 {
                                data.set_block(current_block, &blocks_manager.sand);
                            }
                            else {
                                data.set_block(current_block, &blocks_manager.grass_block);
                            }
                        }

                        else if y <= WATER_HEIGHT && (y == surface_height - 1 || y == surface_height - 2 || y == surface_height - 3) {
                            data.set_block(current_block, &blocks_manager.sand);
                        }
                        else if y == surface_height - 1 || y == surface_height - 2 || y == surface_height - 3 {
                            data.set_block(current_block, &blocks_manager.dirt);
                        }
                        else {
                            data.set_block(current_block, &blocks_manager.stone);
                        }
                    }

                }
            }
        }
    }
}
