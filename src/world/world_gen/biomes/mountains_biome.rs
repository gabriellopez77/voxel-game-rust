use crate::{utils::NullSafePtr, world::{blocks::{BlocksManager}, world_gen::biomes::BiomeBase}};


pub struct MountainsBiome {
    blocks_manager: NullSafePtr<BlocksManager>,
}

impl BiomeBase for MountainsBiome {
    fn get_surface_block(&self) -> (u16, u8) {
        self.blocks_manager.cobblestone
    }

    fn get_underground_block(&self) -> (u16, u8) {
        self.blocks_manager.stone
    }

    fn get_surface_decorations(&self) -> (u16, u8) {
        self.blocks_manager.air
    }
}

impl MountainsBiome {
    pub fn new() -> Self {
        Self {
            blocks_manager: NullSafePtr::null(),
        }
    }

    pub fn start(&mut self, blocks_manager: &BlocksManager) {
        self.blocks_manager = NullSafePtr::new(blocks_manager)
    }
}
