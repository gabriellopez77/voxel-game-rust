use crate::{utils::NullSafePtr, world::{blocks::{BlockProperties, BlocksManager}, world_gen::biomes::BiomeBase}};


pub struct SnowMountainsBiome {
    blocks_manager: NullSafePtr<BlocksManager>,
}

impl BiomeBase for SnowMountainsBiome {
    fn get_surface_block(&self) -> &BlockProperties {
        self.blocks_manager.snow_block.get_properties(0)
    }

    fn get_underground_block(&self) -> &BlockProperties {
        self.blocks_manager.stone.get_properties(0)
    }

    fn get_surface_decorations(&self) -> &BlockProperties {
        self.blocks_manager.snow_layer.get_properties(0)
    }
}

impl SnowMountainsBiome {
    pub fn new() -> Self {
        Self {
            blocks_manager: NullSafePtr::null(),
        }
    }

    pub fn start(&mut self, blocks_manager: &BlocksManager) {
        self.blocks_manager = NullSafePtr::new(blocks_manager)
    }
}
