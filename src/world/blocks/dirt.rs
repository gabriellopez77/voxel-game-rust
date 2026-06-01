use crate::{resources::ResourceManager, world::{blocks::{BlockFunctions, BlockProperties}, items::ItemCreation}};


pub struct Dirt {
    properties: Vec<BlockProperties>
}

impl BlockFunctions for Dirt {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties[state as usize]
    }
}

impl ItemCreation for Dirt {
    type ItemType = Self;

    fn new(internal_name: &'static str, name: &'static str, id: usize, resources: &ResourceManager) -> Self {
        let mut properties = BlockProperties::new(internal_name, name, resources.get_model(internal_name), id, 0);
        properties.can_replaced = false;
        properties.is_transparent = false;
        properties.light_filter = 15;

        Self {
            properties: vec![properties],
        }
    }
}
