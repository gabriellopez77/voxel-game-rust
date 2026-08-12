use crate::{render::chunks_renderer::ChunksRendererType, world::{blocks::{BlockFunctions, BlockProperties}, items::{ItemCreation, ItemCreationArgs}}};


pub struct WaterBlock {
    properties: Vec<BlockProperties>
}

impl BlockFunctions for WaterBlock {
    fn get_properties(&self, state: u8) -> &BlockProperties {
        &self.properties[state as usize]
    }
}

impl ItemCreation for WaterBlock {
    type ItemType = Self;

    fn new(args: &mut ItemCreationArgs) -> Self {
        let mut properties = BlockProperties::new(args, 0);
        args.inventory.register_item(properties.base_properties.clone());

        properties.can_replace = true;
        properties.is_transparent = true;
        properties.light_filter = 1;
        properties.renderer_type = ChunksRendererType::Alpha;
        properties.block_type = super::BlockTypes::Water;
        properties.collision_box = None;
        properties.selection_box = None;

        Self {
            properties: vec![properties],
        }
    }
}
