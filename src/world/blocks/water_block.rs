use crate::{
    game::Directions, render::chunks_renderer::ChunkRendererType, world::{
        Aabb, blocks::{BlockBehaviors, BlockIdState, BlockProperties, BlockTypes}, items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct WaterBlock {
    properties: BlockProperties
}

impl BlockBehaviors for WaterBlock {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }

    fn is_opaque(&self) -> bool { return false }

    fn get_type(&self) -> BlockTypes { BlockTypes::Fluid }

    fn causes_ambient_occlusion(&self) -> bool { return false }

    fn affected_by_ambient_occlusion(&self) -> bool { return true }

    fn should_render_face_twin(&self, dir: Directions) -> bool { return false }

    fn should_render_face(&self, dir: Directions, around: &dyn BlockBehaviors) -> bool {
        if dir != Directions::Up && around.is_opaque()  {
            return false;
        }

        return true;
    }

    fn get_collision_box(&self, state: u8) -> Option<Aabb> { None }
    fn get_selection_box(&self, state: u8) -> Option<Aabb> { None }
}

impl ItemCreation for WaterBlock {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args);
        args.inventory.register_block(BlockIdState::new(properties.id, 0));

        properties.can_replace = true;
        properties.light_filter = 1;
        properties.renderer_type = ChunkRendererType::Alpha;

        Self {
            properties: properties,
        }
    }
}
