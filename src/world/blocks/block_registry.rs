use std::sync::OnceLock;

use crate::{
    resources::ResourceManager,
    world::{
        blocks::*,
        items::*,
        player::PlayerInventory,
    }
};


static BLOCKS_MANAGER: OnceLock<BlockRegistry> = OnceLock::new();

pub fn init(resources: &ResourceManager, inventory: &mut PlayerInventory) {
    BLOCKS_MANAGER.set(BlockRegistry::new(resources, inventory)).ok();
}

pub fn get() -> &'static BlockRegistry {
    BLOCKS_MANAGER.get().unwrap()
}


#[allow(dead_code)]
pub struct BlockRegistry {
    blocks: Vec<&'static dyn BlockBehaviors>,

    pub air: &'static dyn BlockBehaviors,
    pub dirt: &'static dyn BlockBehaviors,
    pub stone: &'static dyn BlockBehaviors,
    pub grass_block: &'static dyn BlockBehaviors,
    pub bedrock: &'static dyn BlockBehaviors,
    pub cobblestone: &'static dyn BlockBehaviors,
    pub sand: &'static dyn BlockBehaviors,
    pub snow_block: &'static dyn BlockBehaviors,
    pub ice_block: &'static dyn BlockBehaviors,
    pub water_block: &'static dyn BlockBehaviors,
    pub snow_layer: &'static dyn BlockBehaviors,
    pub short_grass: &'static dyn BlockBehaviors,
    pub red_flower: &'static dyn BlockBehaviors,
    pub yellow_flower: &'static dyn BlockBehaviors,
    pub dead_bush: &'static dyn BlockBehaviors,
    pub sandstone: &'static dyn BlockBehaviors,
    pub smooth_stone_slab: &'static dyn BlockBehaviors,
    pub torch: &'static dyn BlockBehaviors,
    pub glass_block: &'static dyn BlockBehaviors,
    pub oak_planks: &'static dyn BlockBehaviors,
    pub white_oak_planks: &'static dyn BlockBehaviors,
    pub oak_leaves: &'static dyn BlockBehaviors,
    pub white_oak_leaves: &'static dyn BlockBehaviors,
    pub oak_log: &'static dyn BlockBehaviors,
    pub white_oak_log: &'static dyn BlockBehaviors,
}

// SAFETY: &dyn is readonly
unsafe impl Send for BlockRegistry{}
unsafe impl Sync for BlockRegistry{}

impl BlockRegistry {
    pub fn new(resources: &ResourceManager, inventory: &mut PlayerInventory) -> Self {
        let mut blocks: Vec<&'static dyn BlockBehaviors> = Vec::new();

        Self {
            air: Self::add::<Air>("air", "AIR", &mut blocks, resources, inventory),
            dirt: Self::add::<DefaultOpaqueCube>("dirt", "Dirt", &mut blocks, resources, inventory),
            stone: Self::add::<DefaultOpaqueCube>("stone", "Stone", &mut blocks, resources, inventory),
            grass_block: Self::add::<DefaultOpaqueCube>("grass_block", "Grass Block", &mut blocks, resources, inventory),
            bedrock: Self::add::<DefaultOpaqueCube>("bedrock", "Bedrock", &mut blocks, resources, inventory),
            cobblestone: Self::add::<DefaultOpaqueCube>("cobblestone", "Cobblestone", &mut blocks, resources, inventory),
            sand: Self::add::<DefaultOpaqueCube>("sand", "Sand", &mut blocks, resources, inventory),
            snow_block: Self::add::<DefaultOpaqueCube>("snow_block", "Snow Block", &mut blocks, resources, inventory),
            ice_block: Self::add::<DefaultOpaqueCube>("ice_block", "Ice Block", &mut blocks, resources, inventory),
            water_block: Self::add::<WaterBlock>("water_block", "Water", &mut blocks, resources, inventory),
            snow_layer: Self::add::<SnowLayer>("snow_layer", "Snow Layer", &mut blocks, resources, inventory),
            short_grass: Self::add::<ShortGrass>("short_grass", "Short Grass", &mut blocks, resources, inventory),
            red_flower: Self::add::<RedFlower>("red_flower", "Red Flower", &mut blocks, resources, inventory),
            yellow_flower: Self::add::<YellowFlower>("yellow_flower", "Red Flower", &mut blocks, resources, inventory),
            dead_bush: Self::add::<DeadBush>("dead_bush", "Dead Bush", &mut blocks, resources, inventory),
            sandstone: Self::add::<DefaultOpaqueCube>("sandstone", "Sandstone", &mut blocks, resources, inventory),
            smooth_stone_slab: Self::add::<SmoothStoneSlab>("smooth_stone_slab", "Smooth Stone Slab", &mut blocks, resources, inventory),
            torch: Self::add::<Torch>("torch", "Torch", &mut blocks, resources, inventory),
            glass_block: Self::add::<GlassBlock>("glass_block", "Glass Block", &mut blocks, resources, inventory),
            oak_planks: Self::add::<DefaultOpaqueCube>("oak_planks", "Oak Planks", &mut blocks, resources, inventory),
            white_oak_planks: Self::add::<DefaultOpaqueCube>("white_oak_planks", "White Oak Planks", &mut blocks, resources, inventory),
            oak_leaves: Self::add::<OakLeaves>("oak_leaves", "Oak Leaves", &mut blocks, resources, inventory),
            white_oak_leaves: Self::add::<WhiteOakLeaves>("white_oak_leaves", "White Oak Leaves", &mut blocks, resources, inventory),
            oak_log: Self::add::<DefaultOpaqueCube>("oak_log", "Oak Log", &mut blocks, resources, inventory),
            white_oak_log: Self::add::<DefaultOpaqueCube>("white_oak_log", "White Oak Log", &mut blocks, resources, inventory),

            blocks,
        }
    }

    pub fn get(&self, id_state: BlockIdState) -> &dyn BlockBehaviors {
        &*self.blocks[id_state.id as usize]
    }

    pub fn get_properties(&self, id_stete: BlockIdState) -> &BlockProperties {
        &self.blocks[id_stete.id as usize].get_properties(id_stete.state)
    }


    fn add<T>(
        internal_name: &'static str,
        name: &'static str,
        blocks: &mut Vec<&'static dyn BlockBehaviors>,
        resources: &ResourceManager,
        inventory: &mut PlayerInventory
    ) -> &'static dyn BlockBehaviors
    where
        T: ItemCreation<ItemType: BlockBehaviors>,
        for<'a> T::ItemType: 'a,
    {
        let parent_id = blocks.len();

        let mut creation_args = ItemCreationArgs {
            internal_name,
            name,
            parent_id,
            resources,
            inventory,
        };

        let block_box = Box::leak(Box::new(T::new(&mut creation_args)));

        blocks.push(block_box);

        return block_box;
    }
}
