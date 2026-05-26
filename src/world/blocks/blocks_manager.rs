use std::{rc::Rc, cell::RefCell};

use crate::resources::ResourceManager;
use crate::world::blocks::*;
use crate::world::items::*;


pub struct BlocksManager {
    blocks: Vec<Rc<dyn BlockFunctions>>,

    pub air: Option<Rc<dyn BlockFunctions>>,
    pub dirt: Option<Rc<dyn BlockFunctions>>,
    pub stone: Option<Rc<dyn BlockFunctions>>,
    pub grass_block: Option<Rc<dyn BlockFunctions>>,
    pub bedrock: Option<Rc<dyn BlockFunctions>>,
    pub cobblestone: Option<Rc<dyn BlockFunctions>>,
    pub sand: Option<Rc<dyn BlockFunctions>>,
    pub snow_block: Option<Rc<dyn BlockFunctions>>,
    pub ice_block: Option<Rc<dyn BlockFunctions>>,
    pub water_block: Option<Rc<dyn BlockFunctions>>,
    pub snow_layer: Option<Rc<dyn BlockFunctions>>,
    pub short_grass: Option<Rc<dyn BlockFunctions>>,
    pub red_flower: Option<Rc<dyn BlockFunctions>>,
    pub yellow_flower: Option<Rc<dyn BlockFunctions>>,
    pub dead_bush: Option<Rc<dyn BlockFunctions>>,
}

impl BlocksManager {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),

            air: None,
            dirt: None,
            stone: None,
            grass_block: None,
            bedrock: None,
            cobblestone: None,
            sand: None,
            snow_block: None,
            ice_block: None,
            water_block: None,
            snow_layer: None,
            short_grass: None,
            red_flower: None,
            yellow_flower: None,
            dead_bush: None,
        }
    }

    pub fn start(&mut self, resources_manager: &Rc<RefCell<ResourceManager>>) {
        self.air = self.add::<Air>("air", "AIR", &resources_manager);
        self.dirt = self.add::<Dirt>("dirt", "Dirt", &resources_manager);
        self.stone = self.add::<Stone>("stone", "Stone", &resources_manager);
        self.grass_block = self.add::<GrassBlock>("grass_block", "Grass Block", &resources_manager);
        self.bedrock = self.add::<Bedrock>("bedrock", "Bedrock", &resources_manager);
        self.cobblestone = self.add::<Cobblestone>("cobblestone", "Cobblestone", &resources_manager);
        self.sand = self.add::<Sand>("sand", "Sand", &resources_manager);
        self.snow_block = self.add::<SnowBlock>("snow_block", "Snow Block", &resources_manager);
        self.ice_block = self.add::<IceBlock>("ice_block", "Ice Block", &resources_manager);
        self.water_block = self.add::<WaterBlock>("water_block", "Water", &resources_manager);
        self.snow_layer = self.add::<SnowLayer>("snow_layer", "Snow Layer", &resources_manager);
        self.short_grass = self.add::<ShortGrass>("short_grass", "Short Grass", &resources_manager);
        self.red_flower = self.add::<RedFlower>("red_flower", "Red Flower", &resources_manager);
        self.yellow_flower = self.add::<YellowFlower>("yellow_flower", "Red Flower", &resources_manager);
        self.dead_bush = self.add::<DeadBush>("dead_bush", "Dead Bush", &resources_manager);
    }

    pub fn get(&self, id: u16) -> Rc<dyn BlockFunctions> {
         self.blocks[id as usize].clone()
    }

    fn add<T>(&mut self, internal_name: &'static str, name: &'static str,
        resources_manager: &Rc<RefCell<ResourceManager>>) -> Option<Rc<dyn BlockFunctions>>
    where
        T: ItemCreation<ItemType: BlockFunctions>,
        for<'a> T::ItemType: 'a,
    {
        let mut block = T::new(internal_name, name, self.blocks.len());
        block.get_base_mut().load_model(&resources_manager);

        let block_rc: Rc<T::ItemType> = Rc::from(block);
        self.blocks.push(block_rc.clone());

        return Some(block_rc);
    }
}
