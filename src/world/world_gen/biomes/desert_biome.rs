use crate::{
    world::{
        blocks::{BlockBehaviors, block_registry},
        world_gen::biomes::BiomeBase
    }
};


pub struct DesertBiome {
}

impl BiomeBase for DesertBiome {
    fn get_surface_block(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().sand
    }

    fn get_underground_block(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().sandstone
    }

    fn get_surface_decorations(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().dead_bush
    }
}

impl DesertBiome {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn start(&mut self) {
    }
}
