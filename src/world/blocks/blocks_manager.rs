use std::{rc::Rc, cell::RefCell};
use std::ptr::NonNull;
use crate::resources::ResourceManager;
use crate::world::blocks::*;
use crate::world::items::*;


pub struct BlocksManager {
    blocks: Vec<BlocksWrapper>,

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
        let mut blocks: Vec<BlocksWrapper> = Vec::new();

        Self {
            air: Self::add::<Air>("air", "AIR", &resources_manager, &mut blocks),
            dirt: Self::add::<Dirt>("dirt", "Dirt", &resources_manager, &mut blocks),
            stone: Self::add::<Stone>("stone", "Stone", &resources_manager, &mut blocks),
            grass_block: Self::add::<GrassBlock>("grass_block", "Grass Block", &resources_manager, &mut blocks),
            bedrock: Self::add::<Bedrock>("bedrock", "Bedrock", &resources_manager, &mut blocks),
            cobblestone: Self::add::<Cobblestone>("cobblestone", "Cobblestone", &resources_manager, &mut blocks),
            sand: Self::add::<Sand>("sand", "Sand", &resources_manager, &mut blocks),
            snow_block: Self::add::<SnowBlock>("snow_block", "Snow Block", &resources_manager, &mut blocks),
            ice_block: Self::add::<IceBlock>("ice_block", "Ice Block", &resources_manager, &mut blocks),
            water_block: Self::add::<WaterBlock>("water_block", "Water", &resources_manager, &mut blocks),
            snow_layer: Self::add::<SnowLayer>("snow_layer", "Snow Layer", &resources_manager, &mut blocks),
            short_grass: Self::add::<ShortGrass>("short_grass", "Short Grass", &resources_manager, &mut blocks),
            red_flower: Self::add::<RedFlower>("red_flower", "Red Flower", &resources_manager, &mut blocks),
            yellow_flower: Self::add::<YellowFlower>("yellow_flower", "Red Flower", &resources_manager, &mut blocks),
            dead_bush: Self::add::<DeadBush>("dead_bush", "Dead Bush", &resources_manager, &mut blocks),

            blocks,
        }
    }

    pub fn get(&self, id: u16) -> BlocksWrapper {
         self.blocks[id as usize]
    }

    fn add<T>(internal_name: &'static str, name: &'static str,
        resources_manager: &Rc<RefCell<ResourceManager>>, blocks: &mut Vec<BlocksWrapper>) -> Box<dyn BlockFunctions>
    where
        T: ItemCreation<ItemType: BlockFunctions>,
        for<'a> T::ItemType: 'a,
    {
        let mut block = T::new(internal_name, name, blocks.len());
        block.get_base_mut().load_model(&resources_manager);

        let block_box: Box<T::ItemType> = Box::new(block);
        blocks.push(BlocksWrapper::new(block_box.as_ref()));

        return block_box;
    }
}
