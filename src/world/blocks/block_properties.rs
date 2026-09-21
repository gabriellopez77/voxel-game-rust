use std::sync::Arc;

use crate::{
    render::chunks_renderer::ChunkRendererType,
    world::{Aabb, items::*}
};


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BlockTypes {
    Default,
    Glass,
    Slab,
    Fluid,
    SnowLayer,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct BlockIdState {
    pub id: u16,
    pub state: u8,
}

impl BlockIdState {
    pub const AIR: Self = Self { id: 0, state: 0 };

    pub fn new(id: u16, state: u8) -> Self {
        Self {
            id,
            state
        }
    }
}

pub struct BlockProperties {
    pub can_replace: bool,
    pub light_filter: u8,
    pub light_emission: u8,
    pub renderer_type: ChunkRendererType,
    pub collision_box: Option<Aabb>,
    pub selection_box: Option<Aabb>,

    pub base_properties: Arc<ItemBaseProperties>,
}

impl PartialEq for BlockProperties {
    fn eq(&self, other: &Self) -> bool {
        self.base_properties.get_id_state() == other.base_properties.get_id_state()
    }
}

impl PartialEq<BlockIdState> for BlockProperties{
    fn eq(&self, id_state: &BlockIdState) -> bool {
        self.base_properties.get_id_state() == *id_state
    }
}

impl BlockProperties {
    pub fn new(args: &ItemCreationArgs, state: u8) -> Self {
        Self {
            can_replace: false,
            light_filter: 0,
            light_emission: 0,
            renderer_type: ChunkRendererType::Opaque,
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
