use crate::world::{
        blocks::{BlockBehaviors, block_registry},
        world_gen::biomes::BiomeBase,
    };


pub struct BeachBiome {
}

impl BiomeBase for BeachBiome {
    fn get_surface_block(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().sand
    }

    fn get_underground_block(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().stone
    }

    fn get_surface_decorations(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().air
    }
}

impl BeachBiome {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn start(&mut self) {
    }
}
