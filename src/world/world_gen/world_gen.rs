use fastnoise_lite::{FastNoiseLite, NoiseType};

use rand::RngExt;
use rand::rngs::ThreadRng;
use crate::math::Vec3i;
use crate::world::world_gen::biomes::*;
use crate::world::Chunk;
use crate::world::blocks::BlocksManager;
use crate::world::chunk::ChunkData;


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Biomes {
    Ocean,
    Beach,
    Desert,
    Plains,
    Mountains,
    SnowMountains,
}

pub struct WorldGen {
    elevation_noise: FastNoiseLite,
    moisture_noise: FastNoiseLite,
    rand: ThreadRng,

    plains_biome: PlainsBiome,
    desert_biome: DesertBiome,
    mountains_biome: MountainsBiome,
    ocean_biome: OceanBiome,
    beach_biome: BeachBiome,
    snow_mountains_biome: SnowMountainsBiome,
}

unsafe impl Send for WorldGen {}

impl WorldGen {
    pub fn new() -> Self {
        let mut elevation_noise = FastNoiseLite::new();
        elevation_noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        elevation_noise.set_seed(Some(-444));
        elevation_noise.set_frequency(Some(0.01));
        elevation_noise.set_fractal_octaves(Some(3));
        elevation_noise.set_fractal_lacunarity(Some(2.17));
        elevation_noise.set_fractal_gain(Some(0.62));

        let mut moisture_noise = FastNoiseLite::new();
        moisture_noise.set_noise_type(Some(NoiseType::ValueCubic));
        moisture_noise.set_seed(Some(-444));
        moisture_noise.set_frequency(Some(0.006));


        Self {
            elevation_noise,
            moisture_noise,
            rand: rand::rng(),

            plains_biome: PlainsBiome::new(),
            desert_biome: DesertBiome::new(),
            mountains_biome: MountainsBiome::new(),
            ocean_biome: OceanBiome::new(),
            beach_biome: BeachBiome::new(),
            snow_mountains_biome: SnowMountainsBiome::new(),
        }
    }

    pub fn start(&mut self, blocks_manager: &BlocksManager) {
        self.plains_biome.start(blocks_manager);
        self.desert_biome.start(blocks_manager);
        self.mountains_biome.start(blocks_manager);
        self.ocean_biome.start(blocks_manager);
        self.beach_biome.start(blocks_manager);
        self.snow_mountains_biome.start(blocks_manager);
    }

    pub fn gen_data(&mut self, chunk_pos: Vec3i, data: &mut ChunkData, blocks_manager: &BlocksManager) {
        let start_x = chunk_pos.x * Chunk::CHUNK_SIZE.x;
        let start_z = chunk_pos.z * Chunk::CHUNK_SIZE.z;

        //return;
        const SURFACE_HEIGHT:i32 = 40;
        const WATER_HEIGHT:i32 = 45;

        for x in 0..Chunk::CHUNK_SIZE.x {
        for z in 0..Chunk::CHUNK_SIZE.z {
            let mut surface_height = SURFACE_HEIGHT as i32;

            let noise_x = x + start_x;
            let noise_z = z + start_z;
            let mut ridge = 0.0f32;
            let mut p = 1.0f32;
            for _ in 0..6 {
                let h = self.elevation_noise.get_noise_2d(
                    noise_x as f32 * p / 8.0,
                    noise_z as f32 * p / 10.0
                );

                ridge += h / p;
                p *= 2.0;
            }

            surface_height += (ridge * ridge * ridge * ridge * 11.0) as i32;

            //let elevation = surface_height as f32;
            //let moisture = self.moisture_noise.get_noise_2d(noise_x as f32, noise_z as f32);
            //let biome = Self::choose_biome(elevation, moisture);

            //let n2 = self.elevation_noise.get_noise_2d(
            //    noise_x as f32,
            //    noise_z as f32
            //);
            //let surface_height = math::lerp(0.0, 128.0, (n2 + 1.0) / 2.0) as i32;

            for y in 0..Chunk::CHUNK_SIZE.y {
                let current_block = Vec3i::new(x, y, z);


                // surface features
                if y > surface_height {
                    if y < WATER_HEIGHT {
                        //if (y == 49) {
                        //    if (chance(0..100) < 2)
                        //        data.set_block(current_block, bLOCKS_manager::LILY_PAD);
                        //}
                        //else
                            data.set_block(current_block, blocks_manager.water_block)
                    }

                    if y == surface_height + 1 {
                        if y >= 100 {
                            data.set_block(current_block, blocks_manager.snow_layer)
                        }
                        else if y >= WATER_HEIGHT + 3 && y <= 81 {
                            if self.chance(0, 100) < 20 {
                                data.set_block(current_block, blocks_manager.short_grass)
                            }
                            //else if self.chance(0, 100) < 1 {
                            //    data.set_block(current_block, blocks_manager.MUSHROOM_BLUE_GROUP), 0;
                            //}
                            else if self.chance(0, 1000) < 12 {
                                data.set_block(current_block, blocks_manager.red_flower)
                            }
                            else if self.chance(0, 1000) < 12 {
                                data.set_block(current_block, blocks_manager.yellow_flower)
                            }

                            //else if self.chance(0, 1000) < 50 {
                            //    treesPos.Add(currentBlock);
                            //}
                        }
                        else {
                            if y >= WATER_HEIGHT && y <= WATER_HEIGHT + 3 {
                                if self.chance(0, 100) < 2 {
                                    data.set_block(current_block, blocks_manager.dead_bush)
                                }
                            }
                        }
                    }
                }
//
                // Ground
                else {
                    if surface_height > 80 {
                        if (y == surface_height || y == surface_height - 1 || y == surface_height - 2) && y > 100 {
                            if self.chance(0, 100) < 2 {
                                data.set_block(current_block, blocks_manager.ice_block)
                            }
                            else { data.set_block(current_block, blocks_manager.snow_block) }
                        }
                        else if self.chance(0, 100) < 20 {
                            data.set_block(current_block, blocks_manager.cobblestone)
                        }
                        else { data.set_block(current_block, blocks_manager.stone) }
                    }

                    else if y == surface_height {
                        if surface_height <= WATER_HEIGHT + 1 {
                            data.set_block(current_block, blocks_manager.sand)
                        }
                        else { data.set_block(current_block, blocks_manager.grass_block) }
                    }

                    else if y <= WATER_HEIGHT && (y == surface_height - 1 || y == surface_height - 2 || y == surface_height - 3) {
                        data.set_block(current_block, blocks_manager.sand)
                    }
                    else if y == surface_height - 1 || y == surface_height - 2 || y == surface_height - 3 {
                        data.set_block(current_block, blocks_manager.dirt)
                    }
                    else {
                        data.set_block(current_block, blocks_manager.stone)
                    }
                }

            }
        }
        }

        // set bedrock
        for x in 0..Chunk::CHUNK_SIZE.x {
            for z in 0..Chunk::CHUNK_SIZE.z {
                data.set_block(Vec3i::new(x, 0, z), blocks_manager.bedrock);
            }
        }
    }

    fn choose_biome(e: f32, m: f32) -> Biomes {
        const WATER_HEIGHT: f32 = 43.0;
        const SURFACE_HEIGHT: f32 = 40.0;

        if e < WATER_HEIGHT { return Biomes::Ocean }
        if e < WATER_HEIGHT + 3.0 { return Biomes::Beach }

        if e > 100.0 {
            return Biomes::SnowMountains;
        }

        if e > 80.0 {
          return Biomes::Mountains;
        }

        if e >= SURFACE_HEIGHT {
            if m < 0.16 { return Biomes::Desert; }
            if m < 0.50 { return Biomes::Plains; }

            return Biomes::Plains;
        }

        return Biomes::SnowMountains;
    }

    fn chance(&mut self, min: i32, max: i32) -> i32 {
        self.rand.random_range(min..=max)
    }
}
