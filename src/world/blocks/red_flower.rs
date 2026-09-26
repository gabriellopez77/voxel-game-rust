use crate::world::{
        Aabb, blocks::{BlockBehaviors, BlockIdState, BlockProperties}, items::{ItemCreation, ItemCreationArgs}
    };


pub struct RedFlower {
    properties: BlockProperties
}

impl BlockBehaviors for RedFlower {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }

    fn causes_ambient_occlusion(&self) -> bool { return false }

    fn is_opaque(&self) -> bool { return false }
    fn get_collision_box(&self, state: u8) -> Option<Aabb> { return None }
    fn get_selection_box(&self, state: u8) -> Option<Aabb> { Some(Aabb::new_cube(5, 0, 5, 6, 10, 6)) }
}

impl ItemCreation for RedFlower {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args);
        args.inventory.register_block(BlockIdState::new(properties.id, 0));

        properties.can_replace = false;
        properties.light_filter = 0;

        Self {
            properties: properties,
        }
    }
}
