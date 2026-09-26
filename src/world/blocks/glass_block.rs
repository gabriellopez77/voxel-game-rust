use crate::{
    game::Directions, world::{
        blocks::{BlockBehaviors, BlockIdState, BlockProperties, BlockTypes}, items::{ItemCreation, ItemCreationArgs}
    }
};


pub struct GlassBlock {
    properties: BlockProperties
}

impl BlockBehaviors for GlassBlock {
    fn get_properties(&self) -> &BlockProperties {
        &self.properties
    }

    fn get_type(&self) -> BlockTypes { BlockTypes::Glass }

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
        let mut properties = BlockProperties::new(args);
        args.inventory.register_block(BlockIdState::new(properties.id, 0));

        properties.can_replace = false;
        properties.light_filter = 0;

        Self {
            properties: properties,
        }
    }
}
