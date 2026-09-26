use crate::world::{
        Aabb, blocks::{BlockBehaviors, BlockIdState, BlockProperties}, items::{ItemCreation, ItemCreationArgs}
    };


pub struct ShortGrass {
    properties: BlockProperties
}

impl BlockBehaviors for ShortGrass {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }
    fn is_opaque(&self) -> bool { return false }

    fn causes_ambient_occlusion(&self) -> bool {
        return false;
    }

    fn get_collision_box(&self, state: u8) -> Option<Aabb> { None }
}

impl ItemCreation for ShortGrass {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args);
        args.inventory.register_block(BlockIdState::new(properties.id, 0));

        properties.can_replace = true;
        properties.light_filter = 0;

        Self {
            properties: properties,
        }
    }
}
