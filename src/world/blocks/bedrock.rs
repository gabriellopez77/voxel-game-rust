use crate::{world::{blocks::{BlockBehaviors, BlockProperties}, items::{ItemCreation, ItemCreationArgs}}};


pub struct Bedrock {
    properties: BlockProperties
}

impl BlockBehaviors for Bedrock {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties
    }
}

impl ItemCreation for Bedrock {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = false;
        properties.is_transparent = false;
        properties.light_filter = 15;

        Self {
            properties: properties,
        }
    }
}
