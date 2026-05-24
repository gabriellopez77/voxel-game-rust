use crate::world::items::ItemCreation;

use super::block_properties::*;


pub struct Air {
    pub properties: BlockProperties
}

impl BlockFunctions for Air {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }
    fn get_properties_mut(&mut self) -> &mut BlockProperties { &mut self.properties }
}

impl ItemCreation for Air {
    type ItemType = Self;

    fn new(internal_name: &'static str, name: &'static str, id: usize) -> Self {
        let mut properties = BlockProperties::new(internal_name, name, id);
        properties.can_replaced = true;
        properties.is_transparent = true;

        Self { properties }
    }
}
