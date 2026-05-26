use crate::world::{blocks::{BlockFunctions, BlockProperties, BlockTypes}, items::ItemCreation};


pub struct SnowLayer {
    properties: BlockProperties
}

impl BlockFunctions for SnowLayer {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }
    fn get_properties_mut(&mut self) -> &mut BlockProperties { &mut self.properties }
}

impl ItemCreation for SnowLayer {
    type ItemType = Self;

    fn new(internal_name: &'static str, name: &'static str, id: usize) -> Self {
        let mut properties = BlockProperties::new(internal_name, name, id);
        properties.can_replaced = true;
        properties.is_transparent = true;
        properties.light_filter = 0;
        properties.block_type = BlockTypes::SnowLayer;

        Self { properties }
    }
}
