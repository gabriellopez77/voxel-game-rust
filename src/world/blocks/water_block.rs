use crate::{render::chunk_renderer::RendererType, resources::ResourceManager, world::{blocks::{BlockFunctions, BlockProperties}, items::ItemCreation}};


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

    fn new(internal_name: &'static str, name: &'static str, id: usize, resources: &ResourceManager) -> Self {
        let mut properties = BlockProperties::new(internal_name, name, resources.get_model(internal_name), id, 0);
        properties.can_replaced = true;
        properties.is_transparent = true;
        properties.light_filter = 1;
        properties.renderer_type = RendererType::Alpha;
        properties.block_type = super::BlockTypes::Water;
        properties.collision_box = None;
        properties.selection_box = None;

        Self {
            properties: vec![properties],
        }
    }
}
