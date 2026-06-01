use crate::{resources::ResourceManager, world::items::ItemCreation};

use super::block_properties::*;


pub struct Air {
    properties: Vec<BlockProperties>
}

impl BlockFunctions for Air {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties[state as usize]
    }
}

impl ItemCreation for Air {
    type ItemType = Self;

    fn new(internal_name: &'static str, name: &'static str, id: usize, resources: &ResourceManager) -> Self {
        let mut properties = BlockProperties::new(internal_name, name, resources.get_model(internal_name), id, 0);
        properties.can_replaced = true;
        properties.is_transparent = true;

        Self {
            properties: vec![properties],
        }
    }
}
