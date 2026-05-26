use crate::world::{blocks::{BlockFunctions, BlockProperties}, items::ItemCreation};


pub struct YellowFlower {
    properties: BlockProperties
}

impl BlockFunctions for YellowFlower {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }
    fn get_properties_mut(&mut self) -> &mut BlockProperties { &mut self.properties }
}

impl ItemCreation for YellowFlower {
    type ItemType = Self;

    fn new(internal_name: &'static str, name: &'static str, id: usize) -> Self {
        let mut properties = BlockProperties::new(internal_name, name, id);
        properties.can_replaced = true;
        properties.is_transparent = true;
        properties.light_filter = 0;

        Self { properties }
    }
}
