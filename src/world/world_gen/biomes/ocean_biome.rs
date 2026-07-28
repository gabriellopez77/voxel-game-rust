use crate::{utils::NullSafePtr, world::{blocks::{BlocksManager}, world_gen::biomes::BiomeBase}};


pub struct OceanBiome {
    blocks_manager: NullSafePtr<BlocksManager>,
}

impl BiomeBase for OceanBiome {
    fn get_surface_block(&self) -> (u16, u8) {
        self.blocks_manager.sand
    }

    fn get_underground_block(&self) -> (u16, u8) {
        self.blocks_manager.sand
    }

    fn get_surface_decorations(&self) -> (u16, u8) {
        self.blocks_manager.water_block
    }
}

impl OceanBiome {
    pub fn new() -> Self {
        Self {
            blocks_manager: NullSafePtr::null(),
        }
    }

    pub fn start(&mut self, blocks_manager: &BlocksManager) {
        self.blocks_manager = NullSafePtr::new(blocks_manager)
    }
}
