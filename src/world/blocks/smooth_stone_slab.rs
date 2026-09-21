use crate::{
    game::Directions,
    world::{
        Aabb,
        blocks::{BlockBehaviors, BlockProperties, BlockTypes},
        items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct SmoothStoneSlab {
    properties: BlockProperties
}

impl BlockBehaviors for SmoothStoneSlab {
    fn get_properties(&self, state: u8) -> &BlockProperties { &self.properties }
    fn get_type(&self) -> BlockTypes { BlockTypes::Slab }
    fn causes_ambient_occlusion(&self) -> bool { return false }
    fn is_opaque(&self) -> bool { return false }
    fn should_render_face_twin(&self, dir: Directions) -> bool { return dir.is_vertical() }

    fn should_render_face(&self, dir: Directions, around: &dyn BlockBehaviors) -> bool {
        if dir != Directions::Up && around.is_opaque() {
            return false;
        }

        return true;
    }
}

impl ItemCreation for SmoothStoneSlab {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = false;
        properties.light_filter = 0;
        properties.collision_box = Some(Aabb::new(0.0, 0.0, 0.0, 1.0, 0.5, 1.0));
        properties.set_selection_box(0, 0, 0, 16, 8, 16);

        Self {
            properties: properties,
        }
    }
}
