use crate::{
    world::{
        blocks::{BlockBehaviors, BlockProperties},
        items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct DefaultOpaqueCube {
    properties: BlockProperties
}

impl BlockBehaviors for DefaultOpaqueCube {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties
    }
}

impl ItemCreation for DefaultOpaqueCube {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = false;
        properties.light_filter = 15;

        Self {
            properties: properties,
        }
    }
}
