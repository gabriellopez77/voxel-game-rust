use crate::world::blocks::{BlockFunctions, BlockProperties, BlockCreation};


pub struct Stone {
    properties: BlockProperties
}

impl BlockFunctions for Stone {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }
}

impl BlockCreation for Stone {
    type BlockType = Self;

    fn new(internal_name: &'static str, name: &'static str, id: usize) -> Self {
        let mut properties = BlockProperties::new(internal_name, name, id);
        properties.can_replaced = false;
        properties.is_transparent = false;
        properties.light_filter = 15;
        
        Self { properties }
    }
}