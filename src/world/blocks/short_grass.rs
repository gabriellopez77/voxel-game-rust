use crate::{world::{blocks::{BlockFunctions, BlockProperties}, items::{ItemCreation, ItemCreationArgs}}};


pub struct ShortGrass {
    properties: Vec<BlockProperties>
}

impl BlockFunctions for ShortGrass {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties[state as usize]
    }
}

impl ItemCreation for ShortGrass {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = true;
        properties.is_transparent = true;
        properties.light_filter = 0;
        properties.collision_box = None;

        Self {
            properties: vec![properties],
        }
    }
}
