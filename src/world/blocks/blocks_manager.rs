use std::sync::Arc;

use crate::resources::ResourceManager;
use crate::utils::SafePtr;
use crate::world::chunk::chunk_data::ChunkBlockInfo;
use crate::world::blocks::*;
use crate::world::items::*;
use crate::world::player::PlayerInventory;

#[allow(dead_code)]


#[derive(Default)]
pub struct BlocksManager {
    blocks: Vec<Box<dyn BlockFunctions>>,

    pub air: BlockIdState,
    pub dirt: BlockIdState,
    pub stone: BlockIdState,
    pub grass_block: BlockIdState,
    pub bedrock: BlockIdState,
    pub cobblestone: BlockIdState,
    pub sand: BlockIdState,
    pub snow_block: BlockIdState,
    pub ice_block: BlockIdState,
    pub water_block: BlockIdState,
    pub snow_layer: BlockIdState,
    pub short_grass: BlockIdState,
    pub red_flower: BlockIdState,
    pub yellow_flower: BlockIdState,
    pub dead_bush: BlockIdState,
    pub sandstone: BlockIdState,
    pub smooth_stone_slab: BlockIdState,
}

impl BlocksManager {
    pub fn new(resources: &ResourceManager, inventory: &mut PlayerInventory) -> Self {
        let mut blocks: Vec<Box<dyn BlockFunctions>> = Vec::new();

        Self {
            air: Self::add::<Air>("air", "AIR", &mut blocks, resources, inventory),
            dirt: Self::add::<Dirt>("dirt", "Dirt", &mut blocks, resources, inventory),
            stone: Self::add::<Stone>("stone", "Stone", &mut blocks, resources, inventory),
            grass_block: Self::add::<GrassBlock>("grass_block", "Grass Block", &mut blocks, resources, inventory),
            bedrock: Self::add::<Bedrock>("bedrock", "Bedrock", &mut blocks, resources, inventory),
            cobblestone: Self::add::<Cobblestone>("cobblestone", "Cobblestone", &mut blocks, resources, inventory),
            sand: Self::add::<Sand>("sand", "Sand", &mut blocks, resources, inventory),
            snow_block: Self::add::<SnowBlock>("snow_block", "Snow Block", &mut blocks, resources, inventory),
            ice_block: Self::add::<IceBlock>("ice_block", "Ice Block", &mut blocks, resources, inventory),
            water_block: Self::add::<WaterBlock>("water_block", "Water", &mut blocks, resources, inventory),
            snow_layer: Self::add::<SnowLayer>("snow_layer", "Snow Layer", &mut blocks, resources, inventory),
            short_grass: Self::add::<ShortGrass>("short_grass", "Short Grass", &mut blocks, resources, inventory),
            red_flower: Self::add::<RedFlower>("red_flower", "Red Flower", &mut blocks, resources, inventory),
            yellow_flower: Self::add::<YellowFlower>("yellow_flower", "Red Flower", &mut blocks, resources, inventory),
            dead_bush: Self::add::<DeadBush>("dead_bush", "Dead Bush", &mut blocks, resources, inventory),
            sandstone: Self::add::<Sandstone>("sandstone", "Sandstone", &mut blocks, resources, inventory),
            smooth_stone_slab: Self::add::<SmoothStoneSlab>("smooth_stone_slab", "Smooth Stone Slab", &mut blocks, resources, inventory),

            blocks,
        }
    }

    pub fn get_from_id(&self, id: u16) -> &Box<dyn BlockFunctions> {
        &self.blocks[id as usize]
    }

    pub fn get_from_block_info(&self, block_info: ChunkBlockInfo) -> &Box<dyn BlockFunctions> {
        &self.blocks[block_info.id as usize]
    }

    pub fn get_from_item_base(&self, item_base: &Arc<ItemBaseProperties>) -> &Box<dyn BlockFunctions> {
        &self.blocks[item_base.parent_index as usize]
    }


    pub fn get_properties_from_id(&self, id: u16, state: u8) -> SafePtr<BlockProperties> {
        SafePtr::new(self.blocks[id as usize].get_properties(state))
    }

    pub fn get_properties_from_block_info(&self, block_info: ChunkBlockInfo) -> SafePtr<BlockProperties> {
         SafePtr::new(self.blocks[block_info.id as usize].get_properties(block_info.state))
    }

    pub fn get_properties_from_item_base(&self, item_base: &Arc<ItemBaseProperties>) -> SafePtr<BlockProperties> {
        SafePtr::new(self.blocks[item_base.id as usize].get_properties(item_base.state))
    }



    fn add<T>(internal_name: &'static str, name: &'static str, blocks: &mut Vec<Box<dyn BlockFunctions>>,
              resources: &ResourceManager, inventory: &mut PlayerInventory) -> BlockIdState
    where
        T: ItemCreation<ItemType: BlockFunctions>,
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

        let block_box = Box::new(T::new(&mut creation_args));
        let id = block_box.get_base().id;

        blocks.push(block_box);

        return BlockIdState { id: id as u16, state: 0 };
    }
}
