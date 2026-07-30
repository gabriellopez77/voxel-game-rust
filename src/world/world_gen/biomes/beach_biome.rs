use crate::{utils::NullSafePtr, world::{blocks::{BlockIdState, BlocksManager}, world_gen::biomes::BiomeBase}};


pub struct BeachBiome {
    blocks_manager: NullSafePtr<BlocksManager>,
}

impl BiomeBase for BeachBiome {
    fn get_surface_block(&self) -> BlockIdState {
        self.blocks_manager.sand
    }

    fn get_underground_block(&self) -> BlockIdState {
        self.blocks_manager.stone
    }

    fn get_surface_decorations(&self) -> BlockIdState {
        self.blocks_manager.air
    }
}

impl BeachBiome {
    pub fn new() -> Self {
        Self {
            blocks_manager: NullSafePtr::null(),
        }
    }

    pub fn start(&mut self, blocks_manager: &BlocksManager) {
        self.blocks_manager = NullSafePtr::new(blocks_manager)
    }
}
