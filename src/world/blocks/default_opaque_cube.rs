use crate::world::{
        blocks::{BlockBehaviors, BlockIdState, BlockProperties}, items::{ItemCreation, ItemCreationArgs}
    };


pub struct DefaultOpaqueCube {
    properties: BlockProperties
}

impl BlockBehaviors for DefaultOpaqueCube {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }
}

impl ItemCreation for DefaultOpaqueCube {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args);
        args.inventory.register_block(BlockIdState::new(properties.id, 0));

        properties.can_replace = false;
        properties.light_filter = 15;

        Self {
            properties: properties,
        }
    }
}
