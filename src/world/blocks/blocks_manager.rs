use std::{rc::Rc, cell::RefCell};
use std::sync::Arc;
use crate::resources::ResourceManager;
use crate::utils::SafePtr;
use crate::world::blocks::*;
use crate::world::chunk::chunk_data::ChunkDataInfo;
use crate::world::items::*;


pub struct BlocksManager {
    blocks: Vec<SafePtr<dyn BlockFunctions>>,

    pub air: Box<dyn BlockFunctions>,
    pub dirt: Box<dyn BlockFunctions>,
    pub stone: Box<dyn BlockFunctions>,
    pub grass_block: Box<dyn BlockFunctions>,
    pub bedrock: Box<dyn BlockFunctions>,
    pub cobblestone: Box<dyn BlockFunctions>,
    pub sand: Box<dyn BlockFunctions>,
    pub snow_block: Box<dyn BlockFunctions>,
    pub ice_block: Box<dyn BlockFunctions>,
    pub water_block: Box<dyn BlockFunctions>,
    pub snow_layer: Box<dyn BlockFunctions>,
    pub short_grass: Box<dyn BlockFunctions>,
    pub red_flower: Box<dyn BlockFunctions>,
    pub yellow_flower: Box<dyn BlockFunctions>,
    pub dead_bush: Box<dyn BlockFunctions>,
}

impl BlocksManager {
    pub fn new(resources_manager: &Rc<RefCell<ResourceManager>>) -> Self {
        let mut blocks: Vec<SafePtr<dyn BlockFunctions>> = Vec::new();
        let resources = &resources_manager.borrow();

        Self {
            air: Self::add::<Air>("air", "AIR", resources, &mut blocks),
            dirt: Self::add::<Dirt>("dirt", "Dirt", resources, &mut blocks),
            stone: Self::add::<Stone>("stone", "Stone", resources, &mut blocks),
            grass_block: Self::add::<GrassBlock>("grass_block", "Grass Block", resources, &mut blocks),
            bedrock: Self::add::<Bedrock>("bedrock", "Bedrock", resources, &mut blocks),
            cobblestone: Self::add::<Cobblestone>("cobblestone", "Cobblestone", resources, &mut blocks),
            sand: Self::add::<Sand>("sand", "Sand", resources, &mut blocks),
            snow_block: Self::add::<SnowBlock>("snow_block", "Snow Block", resources, &mut blocks),
            ice_block: Self::add::<IceBlock>("ice_block", "Ice Block", resources, &mut blocks),
            water_block: Self::add::<WaterBlock>("water_block", "Water", resources, &mut blocks),
            snow_layer: Self::add::<SnowLayer>("snow_layer", "Snow Layer", resources, &mut blocks),
            short_grass: Self::add::<ShortGrass>("short_grass", "Short Grass", resources, &mut blocks),
            red_flower: Self::add::<RedFlower>("red_flower", "Red Flower", resources, &mut blocks),
            yellow_flower: Self::add::<YellowFlower>("yellow_flower", "Red Flower", resources, &mut blocks),
            dead_bush: Self::add::<DeadBush>("dead_bush", "Dead Bush", resources, &mut blocks),

            blocks,
        }
    }

    pub fn get(&self, id: u16) -> SafePtr<dyn BlockFunctions> {
        self.blocks[id as usize].clone()
    }

    pub fn get_from_item_base(&self, item_base: &Arc<ItemBaseProperties>) -> SafePtr<dyn BlockFunctions> {
        self.blocks[item_base.parent_index as usize].clone()
    }

    pub fn get_properties_from_block_info(&self, block_info: ChunkDataInfo) -> &BlockProperties {
         return &self.blocks[block_info.id as usize].get_properties(block_info.state);
    }

    pub fn get_properties_from_item_base(&self, item_base: &Arc<ItemBaseProperties>) -> &BlockProperties {
        return &self.blocks[item_base.id as usize].get_properties(item_base.state);
    }

    fn add<T>(internal_name: &'static str, name: &'static str,
        resources: &ResourceManager, blocks: &mut Vec<SafePtr<dyn BlockFunctions>>) -> Box<dyn BlockFunctions>
    where
        T: ItemCreation<ItemType: BlockFunctions>,
        for<'a> T::ItemType: 'a,
    {
        let block_box: Box<T::ItemType> = Box::new(T::new(internal_name, name, blocks.len(), resources));
        blocks.push(SafePtr::from(block_box.as_ref()));

        return block_box;
    }
}
