use std::sync::Arc;
use crate::resources::ResourceManager;
use crate::utils::SafePtr;
use crate::world::chunk::chunk_data::ChunkBlockInfo;
use crate::world::blocks::*;
use crate::world::items::*;
use crate::world::player::PlayerInventory;


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
    pub sandstone: Box<dyn BlockFunctions>,
}

impl BlocksManager {
    pub fn new(resources: &ResourceManager, inventory: &mut PlayerInventory) -> Self {
        let mut blocks: Vec<SafePtr<dyn BlockFunctions>> = Vec::new();

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

            blocks,
        }
    }

    pub fn get(&self, id: u16) -> SafePtr<dyn BlockFunctions> {
        self.blocks[id as usize].clone()
    }

    pub fn get_from_item_base(&self, item_base: &Arc<ItemBaseProperties>) -> SafePtr<dyn BlockFunctions> {
        self.blocks[item_base.parent_index as usize].clone()
    }

    pub fn get_properties_from_block_info(&self, block_info: ChunkBlockInfo) -> &BlockProperties {
         return &self.blocks[block_info.id as usize].get_properties(block_info.state);
    }

    pub fn get_properties_from_item_base(&self, item_base: &Arc<ItemBaseProperties>) -> &BlockProperties {
        return &self.blocks[item_base.id as usize].get_properties(item_base.state);
    }

    fn add<T>(internal_name: &'static str, name: &'static str, blocks: &mut Vec<SafePtr<dyn BlockFunctions>>,
              resources: &ResourceManager, inventory: &mut PlayerInventory) -> Box<dyn BlockFunctions>
    where
        T: ItemCreation<ItemType: BlockFunctions>,
        for<'a> T::ItemType: 'a,
    {
        let mut creation_args = ItemCreationArgs {
            internal_name,
            name,
            id: blocks.len(),
            resources,
            inventory,
        };

        let block_box = Box::new(T::new(&mut creation_args));
        blocks.push(SafePtr::new(block_box.as_ref()));

        return block_box;
    }
}
