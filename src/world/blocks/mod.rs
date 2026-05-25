pub mod block_properties;
pub mod blocks_manager;

pub mod air;
pub mod stone;
pub mod dirt;
pub mod grass_block;
pub mod bedrock;
pub mod cobblestone;
pub mod sand;
pub mod snow_block;
pub mod ice_block;
pub mod water_block;

pub use {
    blocks_manager::BlocksManager,
    block_properties::*,

    air::Air,
    stone::Stone,
    dirt::Dirt,
    grass_block::GrassBlock,
    bedrock::Bedrock,
    cobblestone::Cobblestone,
    sand::Sand,
    snow_block::SnowBlock,
    ice_block::IceBlock,
    water_block::WaterBlock,
};
