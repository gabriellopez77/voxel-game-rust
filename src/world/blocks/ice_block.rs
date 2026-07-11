use crate::{world::{blocks::{BlockFunctions, BlockProperties}, items::{ItemCreation, ItemCreationArgs}}};


pub struct IceBlock {
    properties: Vec<BlockProperties>
}

impl BlockFunctions for IceBlock {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties[state as usize]
    }
}

impl ItemCreation for IceBlock {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replaced = false;
        properties.is_transparent = false;
        properties.light_filter = 15;

        Self {
            properties: vec![properties],
        }
    }
}
