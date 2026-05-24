use crate::world::{blocks::{BlockFunctions, BlockProperties}, items::ItemCreation};


pub struct SnowBlock {
    properties: BlockProperties
}

impl BlockFunctions for SnowBlock {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }
    fn get_properties_mut(&mut self) -> &mut BlockProperties { &mut self.properties }
}

impl ItemCreation for SnowBlock {
    type ItemType = Self;

    fn new(internal_name: &'static str, name: &'static str, id: usize) -> Self {
        let mut properties = BlockProperties::new(internal_name, name, id);
        properties.can_replaced = false;
        properties.is_transparent = false;
        properties.light_filter = 15;

        Self { properties }
    }
}
