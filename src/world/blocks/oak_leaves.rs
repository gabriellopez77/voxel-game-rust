use crate::world::{
        blocks::{BlockBehaviors, BlockIdState, BlockProperties}, items::{ItemCreation, ItemCreationArgs}
    };


pub struct OakLeaves {
    properties: BlockProperties
}

impl BlockBehaviors for OakLeaves {
    fn get_properties(&self) -> &BlockProperties { &self.properties }

    fn is_opaque(&self) -> bool { false }
}

impl ItemCreation for OakLeaves {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args);
        args.inventory.register_block(BlockIdState::new(properties.id, 0));

        properties.can_replace = false;
        properties.light_filter = 1;

        Self {
            properties: properties,
        }
    }
}
