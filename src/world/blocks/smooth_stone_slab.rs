use crate::{
    game::Directions, world::{
        Aabb, blocks::{BlockBehaviors, BlockIdState, BlockProperties, BlockTypes}, items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct SmoothStoneSlab {
    properties: BlockProperties
}

impl BlockBehaviors for SmoothStoneSlab {
    fn get_properties(&self) -> &BlockProperties { &self.properties }
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

    fn get_collision_box(&self, state: u8) -> Option<Aabb> { Some(Aabb::new(0.0, 0.0, 0.0, 1.0, 0.5, 1.0)) }
    fn get_selection_box(&self, state: u8) -> Option<Aabb> { Some(Aabb::new_cube(0, 0, 0, 16, 8, 16)) }
}

impl ItemCreation for SmoothStoneSlab {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args);
        args.inventory.register_block(BlockIdState::new(properties.id, 0));

        properties.can_replace = false;
        properties.light_filter = 0;

        Self {
            properties: properties,
        }
    }
}
