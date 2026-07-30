use crate::{world::{blocks::{BlockFunctions, BlockProperties}, items::{ItemCreation, ItemCreationArgs}}};


pub struct RedFlower {
    properties: Vec<BlockProperties>
}

impl BlockFunctions for RedFlower {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties[state as usize]
    }
}

impl ItemCreation for RedFlower {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = false;
        properties.is_transparent = true;
        properties.light_filter = 0;
        properties.collision_box = None;
        properties.set_selection_box(5, 0, 5, 6, 10, 6);

        Self {
            properties: vec![properties],
        }
    }
}
