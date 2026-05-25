use crate::{world::{blocks::{BlockFunctions, BlockProperties}, items::ItemCreation}};


pub struct WaterBlock {
    properties: BlockProperties
}

impl BlockFunctions for WaterBlock {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }
    fn get_properties_mut(&mut self) -> &mut BlockProperties { &mut self.properties }
}

impl ItemCreation for WaterBlock {
    type ItemType = Self;

    fn new(internal_name: &'static str, name: &'static str, id: usize) -> Self {
        let mut properties = BlockProperties::new(internal_name, name, id);
        properties.can_replaced = true;
        properties.is_transparent = true;
        properties.light_filter = 1;
        properties.block_type = super::BlockTypes::Water;

        Self { properties }
    }
}
