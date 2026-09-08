use crate::{world::{blocks::{BlockFunctions, BlockProperties}, items::{ItemCreation, ItemCreationArgs}}};


pub struct GlassBlock {
    properties: BlockProperties
}

impl BlockFunctions for GlassBlock {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties
    }
}

impl ItemCreation for GlassBlock {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = false;
        properties.is_transparent = true;
        properties.light_filter = 0;

        Self {
            properties: properties,
        }
    }
}
