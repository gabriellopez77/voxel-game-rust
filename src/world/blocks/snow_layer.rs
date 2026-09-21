use crate::{
    game::Directions,
    world::{
        blocks::{BlockBehaviors, BlockProperties, BlockTypes},
        items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct SnowLayer {
    properties: BlockProperties
}

impl BlockBehaviors for SnowLayer {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties
    }

    fn is_opaque(&self) -> bool { return false }

    fn get_type(&self) -> BlockTypes { BlockTypes::SnowLayer }

    fn causes_ambient_occlusion(&self) -> bool {
        return false;
    }

    fn should_render_face_twin(&self, dir: Directions) -> bool {
        return dir.is_vertical()
    }

    fn should_render_face(&self, dir: Directions, around: &dyn BlockBehaviors) -> bool {
        if dir != Directions::Up && around.is_opaque()  {
            return false;
        }

        return true;
    }
}

impl ItemCreation for SnowLayer {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = true;
        properties.light_filter = 0;
        properties.collision_box = None;
        properties.set_selection_box(0, 0, 0, 16, 2, 16);

        Self {
            properties: properties,
        }
    }
}
