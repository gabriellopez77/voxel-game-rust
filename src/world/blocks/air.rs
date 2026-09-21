use crate::world::{blocks::BlockBehaviors, items::{ItemCreation, ItemCreationArgs}};

use super::block_properties::*;


pub struct Air {
    properties: BlockProperties
}

impl BlockBehaviors for Air {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties
    }

    fn causes_ambient_occlusion(&self) -> bool { return false }
    fn is_opaque(&self) -> bool { return false }
}

impl ItemCreation for Air {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        properties.can_replace = true;
        properties.collision_box = None;
        properties.selection_box = None;

        Self {
            properties: properties,
        }
    }
}
