use crate::{world::{blocks::{BlockFunctions, BlockProperties, BlockTypes}, items::{ItemCreation, ItemCreationArgs}}};


pub struct SnowLayer {
    properties: Vec<BlockProperties>
}

impl BlockFunctions for SnowLayer {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties[state as usize]
    }
}

impl ItemCreation for SnowLayer {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replaced = true;
        properties.is_transparent = true;
        properties.light_filter = 0;
        properties.block_type = BlockTypes::SnowLayer;
        properties.collision_box = None;

        Self {
            properties: vec![properties],
        }
    }
}
