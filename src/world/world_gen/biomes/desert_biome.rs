use crate::{utils::NullSafePtr, world::{blocks::{BlockProperties, BlocksManager}, world_gen::biomes::BiomeBase}};


pub struct DesertBiome {
    blocks_manager: NullSafePtr<BlocksManager>,
}

impl BiomeBase for DesertBiome {
    fn get_surface_block(&self) -> &BlockProperties {
        self.blocks_manager.sand.get_properties(0)
    }

    fn get_underground_block(&self) -> &BlockProperties {
        self.blocks_manager.sandstone.get_properties(0)
    }

    fn get_surface_decorations(&self) -> &BlockProperties {
        self.blocks_manager.dead_bush.get_properties(0)
    }
}

impl DesertBiome {
    pub fn new() -> Self {
        Self {
            blocks_manager: NullSafePtr::null(),
        }
    }

    pub fn start(&mut self, blocks_manager: &BlocksManager) {
        self.blocks_manager = NullSafePtr::new(blocks_manager)
    }
}
