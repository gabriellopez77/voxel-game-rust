use crate::world::{blocks::{BlockBehaviors, block_registry}, world_gen::biomes::BiomeBase};


pub struct OceanBiome {

}

impl BiomeBase for OceanBiome {
    fn get_surface_block(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().sand
    }

    fn get_underground_block(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().sand
    }

    fn get_surface_decorations(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().water_block
    }
}

impl OceanBiome {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn start(&mut self) {
    }
}
