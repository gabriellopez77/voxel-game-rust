use crate::{
    world::{
        blocks::{BlockBehaviors, BlockProperties},
        items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct WhiteOakLeaves {
    properties: BlockProperties
}

impl BlockBehaviors for WhiteOakLeaves {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties
    }

    fn is_opaque(&self) -> bool { return false }
}

impl ItemCreation for WhiteOakLeaves {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = false;
        properties.light_filter = 1;

        Self {
            properties: properties,
        }
    }
}
