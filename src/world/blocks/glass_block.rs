use crate::{
    game::Directions, world::{
        blocks::{BlockBehaviors, BlockProperties},
        items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct GlassBlock {
    properties: BlockProperties
}

impl BlockBehaviors for GlassBlock {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties
    }

    fn is_opaque(&self) -> bool { return false }

    fn causes_ambient_occlusion(&self) -> bool {
        return false;
    }

    fn should_render_face_twin(&self, dir: Directions) -> bool {
        return false;
    }
}

impl ItemCreation for GlassBlock {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = false;
        properties.light_filter = 0;

        Self {
            properties: properties,
        }
    }
}
