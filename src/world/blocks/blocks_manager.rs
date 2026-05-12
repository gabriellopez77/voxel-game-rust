use std::rc::Rc;

use crate::world::blocks::*;


pub struct BlocksManager {
    blocks: Vec<Rc<dyn BlockFunctions>>,

    air_block: Option<Rc<dyn BlockFunctions>>,
    dirt_block: Option<Rc<dyn BlockFunctions>>,
    stone_block: Option<Rc<dyn BlockFunctions>>,
    grass_block: Option<Rc<dyn BlockFunctions>>,
    bedrock_block: Option<Rc<dyn BlockFunctions>>,
}

impl BlocksManager {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),

            air_block: None,
            dirt_block: None,
            stone_block: None,
            grass_block: None,
            bedrock_block: None,
        }
    }

    pub fn start(&mut self) {
        self.air_block = self.add::<Air>("air", "AIR");
        self.dirt_block = self.add::<Dirt>("dirt", "Dirt");
        self.stone_block = self.add::<Stone>("stone", "Stone");
        self.grass_block = self.add::<GrassBlock>("grass_block", "Grass Block");
        self.bedrock_block = self.add::<Bedrock>("bedrock", "Bedrock");
    }
    
    pub fn get(&self, id: usize) -> Rc<dyn BlockFunctions> {
         self.blocks[id].clone()
    }

    fn add<T>(&mut self, internal_name: &'static str, name: &'static str) -> Option<Rc<dyn BlockFunctions>>
    where 
        T: BlockCreation<BlockType: BlockFunctions>,
        for<'a> T::BlockType: 'a
    {
        let block = Rc::new(T::new(internal_name, name, self.blocks.len()));
        self.blocks.push(block.clone());

        return Some(block.clone());
    }
}
