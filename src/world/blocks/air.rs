use crate::world::{Aabb, blocks::BlockBehaviors, items::{ItemCreation, ItemCreationArgs}};

use super::block_properties::*;


pub struct Air {
    properties: BlockProperties
}

impl BlockBehaviors for Air {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }

    fn causes_ambient_occlusion(&self) -> bool { return false }
    fn is_opaque(&self) -> bool { return false }
    fn get_selection_box(&self, state: u8) -> Option<Aabb> { return None }
    fn get_collision_box(&self, state: u8) -> Option<Aabb> { return None }
}

impl ItemCreation for Air {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args);
        properties.can_replace = true;

        Self {
            properties: properties,
        }
    }
}
