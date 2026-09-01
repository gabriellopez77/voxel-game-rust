use crate::{world::items::{ItemCreation, ItemCreationArgs}};

use super::block_properties::*;


pub struct Air {
    properties: BlockProperties
}

impl BlockFunctions for Air {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties
    }
}

impl ItemCreation for Air {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        properties.can_replace = true;
        properties.is_transparent = true;
        properties.collision_box = None;
        properties.selection_box = None;

        Self {
            properties: properties,
        }
    }
}
