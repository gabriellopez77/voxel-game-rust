use crate::{
    world::{
        blocks::{BlockBehaviors, BlockProperties},
        items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct RedFlower {
    properties: BlockProperties
}

impl BlockBehaviors for RedFlower {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties
    }

    fn causes_ambient_occlusion(&self) -> bool { return false }

    fn is_opaque(&self) -> bool { return false }
}

impl ItemCreation for RedFlower {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = false;
        properties.light_filter = 0;
        properties.collision_box = None;
        properties.set_selection_box(5, 0, 5, 6, 10, 6);

        Self {
            properties: properties,
        }
    }
}
