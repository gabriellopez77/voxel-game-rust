use std::rc::Rc;
use std::sync::Arc;

use crate::render::chunk_renderer::RendererType;
use crate::resources::{BlockItemModel, TexCoords};
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
    fn get_properties(&self, state: u8) -> &BlockProperties;

    fn get_base(&self) -> Arc<ItemBaseProperties> { self.get_properties(0).base_properties.clone() }
}


pub struct BlockProperties {
    pub can_replaced: bool,
    pub is_transparent: bool,
    pub light_filter: u8,
    pub light_emission: u8,
    pub block_type: BlockTypes,
    pub renderer_type: RendererType,
    
    pub base_properties: Arc<ItemBaseProperties>
}

impl BlockProperties {
    pub fn new(internal_name: &'static str, name: &'static str, model: Rc<BlockItemModel>, index: usize, state: u8) -> Self {
        Self {
            can_replaced: false,
            is_transparent: false,
            light_filter: 0,
            light_emission: 0,
            block_type: BlockTypes::Default,
            renderer_type: RendererType::Opaque,
            
            base_properties: Arc::new(ItemBaseProperties::new(
                internal_name,
                name,
                model,
                index,
                state,
                ItemBaseType::Block
            )),
        }
    }

    pub fn copy(&self, internal_name: &'static str, name: &'static str, model: Rc<BlockItemModel>, index: usize, state: u8) -> Self {
        Self {
            can_replaced: self.can_replaced,
            is_transparent: self.is_transparent,
            light_filter: self.light_filter,
            light_emission: self.light_emission,
            block_type: self.block_type,
            renderer_type: self.renderer_type,
            
            base_properties: Arc::new(self.base_properties.copy(internal_name, name, model, index, state, ItemBaseType::Block))
        }
    }
}
