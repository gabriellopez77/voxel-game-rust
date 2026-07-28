use std::rc::Rc;
use std::sync::Arc;

use crate::render::chunk_renderer::RendererType;
use crate::resources::{BlockItemModel};
use crate::world::{Aabb, items::*};


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

    fn get_id_state(&self) -> (u16, u8) {
        let base = self.get_base();

        return (base.id, base.state);
    }
}


pub struct BlockProperties {
    pub can_replaced: bool,
    pub is_transparent: bool,
    pub light_filter: u8,
    pub light_emission: u8,
    pub block_type: BlockTypes,
    pub renderer_type: RendererType,
    pub collision_box: Option<Aabb>,
    pub selection_box: Option<Aabb>,

    pub base_properties: Arc<ItemBaseProperties>,
}

impl BlockProperties {
    pub fn new(args: &ItemCreationArgs, state: u8) -> Self {
        Self {
            can_replaced: false,
            is_transparent: false,
            light_filter: 0,
            light_emission: 0,
            block_type: BlockTypes::Default,
            renderer_type: RendererType::Opaque,
            collision_box: Some(Aabb::CUBE),
            selection_box: Some(Aabb::CUBE),

            base_properties: Arc::new(ItemBaseProperties::new(
                args.internal_name,
                args.name,
                args.resources.get_model(args.internal_name),
                args.parent_id,
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
            collision_box: self.collision_box,
            selection_box: self.selection_box,

            base_properties: Arc::new(self.base_properties.copy(internal_name, name, model, index, state, ItemBaseType::Block))
        }
    }
}
