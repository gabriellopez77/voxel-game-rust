use std::rc::Rc;
use std::sync::Arc;

use crate::render::chunks_renderer::ChunksRendererType;
use crate::resources::GenericModel;
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

    fn get_id_state(&self) -> BlockIdState {
        let base = self.get_base();

        return BlockIdState { id: base.id, state: base.state };
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct BlockIdState {
    pub id: u16,
    pub state: u8,
}

pub struct BlockProperties {
    pub can_replace: bool,
    pub is_transparent: bool,
    pub light_filter: u8,
    pub light_emission: u8,
    pub block_type: BlockTypes,
    pub renderer_type: ChunksRendererType,
    pub collision_box: Option<Aabb>,
    pub selection_box: Option<Aabb>,

    pub base_properties: Arc<ItemBaseProperties>,
}

impl PartialEq for BlockProperties {
    fn eq(&self, other: &Self) -> bool {
        self.base_properties.id == other.base_properties.id &&
        self.base_properties.state == other.base_properties.state
    }
}

impl PartialEq<BlockIdState> for BlockProperties {
    fn eq(&self, id_state: &BlockIdState) -> bool {
        self.base_properties.id == id_state.id &&
        self.base_properties.state == id_state.state
    }
}

impl BlockProperties {
    pub fn new(args: &ItemCreationArgs, state: u8) -> Self {
        Self {
            can_replace: false,
            is_transparent: false,
            light_filter: 0,
            light_emission: 0,
            block_type: BlockTypes::Default,
            renderer_type: ChunksRendererType::Opaque,
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

    pub fn copy(&self,
        internal_name: &'static str,
        name: &'static str,
        model: Rc<GenericModel>,
        index: usize,
        state: u8
    ) -> Self {
        Self {
            can_replace: self.can_replace,
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

    pub fn set_selection_box(&mut self, x: i32, y: i32, z: i32, sx: i32, sy: i32, sz: i32) {
        self.selection_box = Some(Aabb::new(
            x as f32 / 16.0,
            y as f32 / 16.0,
            z as f32 / 16.0,
            (x + sx) as f32 / 16.0,
            (y + sy) as f32 / 16.0,
            (z + sz) as f32 / 16.0,
        ));
    }
}
