use crate::{
    world::{
        blocks::{BlockBehaviors, BlockProperties},
        items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct ShortGrass {
    properties: BlockProperties
}

impl BlockBehaviors for ShortGrass {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties
    }
    fn is_opaque(&self) -> bool { return false }

    fn causes_ambient_occlusion(&self) -> bool {
        return false;
    }
}

impl ItemCreation for ShortGrass {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = true;
        properties.light_filter = 0;
        properties.collision_box = None;

        Self {
            properties: properties,
        }
    }
}
