use crate::{
    game::Directions, world::{
        Aabb, blocks::{BlockBehaviors, BlockIdState, BlockProperties, BlockTypes}, items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct SnowLayer {
    properties: BlockProperties
}

impl BlockBehaviors for SnowLayer {
    fn get_properties(&self) -> &BlockProperties {
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

    fn get_collision_box(&self, state: u8) -> Option<Aabb> { return None }
    fn get_selection_box(&self, state: u8) -> Option<Aabb> { Some(Aabb::new_cube(0, 0, 0, 16, 2, 16)) }
}

impl ItemCreation for SnowLayer {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args);
        args.inventory.register_block(BlockIdState::new(properties.id, 0));

        properties.can_replace = true;
        properties.light_filter = 0;

        Self {
            properties: properties,
        }
    }
}
