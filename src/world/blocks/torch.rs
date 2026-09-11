use crate::{world::{blocks::{BlockFunctions, BlockProperties}, items::{ItemCreation, ItemCreationArgs}}};


pub struct Torch {
    properties: BlockProperties
}

impl BlockFunctions for Torch {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties
    }
}

impl ItemCreation for Torch {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = false;
        properties.is_transparent = true;
        properties.light_emission = 14;
        properties.light_filter = 0;
        properties.collision_box = None;
        properties.set_selection_box(6, 0, 6, 4, 11, 4);

        Self {
            properties: properties,
        }
    }
}
