use crate::{
    world::{
        blocks::{BlockBehaviors, block_registry},
        world_gen::biomes::BiomeBase,
    }
};


pub struct PlainsBiome {

}

impl BiomeBase for PlainsBiome {
    fn get_surface_block(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().grass_block
    }

    fn get_underground_block(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().stone
    }

    fn get_surface_decorations(&self) -> &'static dyn BlockBehaviors {
        block_registry::get().short_grass
    }
}

impl PlainsBiome {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn start(&mut self) {
    }
}
