pub mod biome_base;
pub mod plains_biome;
pub mod desert_biome;
pub mod mountains_biome;
pub mod ocean_biome;
pub mod beach_biome;
pub mod snow_mountains_biome;

pub use {
    biome_base::BiomeBase,
    plains_biome::PlainsBiome,
    desert_biome::DesertBiome,
    mountains_biome::MountainsBiome,
    ocean_biome::OceanBiome,
    beach_biome::BeachBiome,
    snow_mountains_biome::SnowMountainsBiome,
};
