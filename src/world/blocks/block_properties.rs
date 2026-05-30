use crate::render::chunk_renderer::RendererType;
use crate::resources::TexCoords;
use crate::world::items::*;


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BlockTypes {
    Default,
    Glass,
    Slab,
    Water,
    SnowLayer,
}

pub trait BlockFunctions {
    fn get_properties_mut(&mut self) -> &mut BlockProperties;
    fn get_properties(&self) -> &BlockProperties;

    fn get_base(&self) -> &ItemBaseProperties { &self.get_properties().base_properties }
    fn get_base_mut(&mut self) -> &mut ItemBaseProperties { &mut self.get_properties_mut().base_properties }
}


pub struct BlockProperties {
    pub can_replaced: bool,
    pub is_transparent: bool,
    pub light_filter: u8,
    pub light_emission: u8,
    pub block_type: BlockTypes,
    pub renderer_type: RendererType,

    pub base_properties: ItemBaseProperties
}

impl BlockProperties {
    pub fn new(internal_name: &'static str, name: &'static str, index: usize) -> BlockProperties {
        Self {
            can_replaced: false,
            is_transparent: false,
            light_filter: 0,
            light_emission: 0,
            block_type: BlockTypes::Default,
            renderer_type: RendererType::Opaque,
            base_properties: ItemBaseProperties::new(internal_name, name, TexCoords::DEFAULT, Some(index as u32), None)
        }
    }
}
