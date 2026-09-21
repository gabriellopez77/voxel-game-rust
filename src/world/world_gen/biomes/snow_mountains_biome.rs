use crate::world::{
        blocks::{BlockBehaviors, block_registry},
        world_gen::biomes::BiomeBase
    };


pub struct SnowMountainsBiome {
}

impl BiomeBase for SnowMountainsBiome {
    fn get_surface_block(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().snow_block
    }

    fn get_underground_block(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().stone
    }

    fn get_surface_decorations(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().snow_layer
    }
}

impl SnowMountainsBiome {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn start(&mut self) {
    }
}
