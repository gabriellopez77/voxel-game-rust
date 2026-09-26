use std::sync::Arc;

use crate::{
    render::chunks_renderer::ChunkRendererType,
    resources::ItemBlockModel,
    world::{
        blocks::block_states::{BlockStates, BlockStatesTypes},
        items::*
    }
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
    pub internal_name: &'static str,
    pub name: &'static str,
    pub id: u16,
    pub default_state: u8,
    model: Arc<ItemBlockModel>,

    states: Vec<BlockStates>,


    pub can_replace: bool,
    pub light_filter: u8,
    pub light_emission: u8,
    pub renderer_type: ChunkRendererType,

}

impl PartialEq for BlockProperties {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl BlockProperties {
    pub fn new(args: &ItemCreationArgs) -> Self {
        static mut CURRENT_ID: u16 = 0;

        // SAFETY: called only on the main thread
        let new_id = unsafe {
            let temp = CURRENT_ID;
            CURRENT_ID += 1;

            temp
        };


        Self {
            internal_name: args.internal_name,
            name: args.name,
            id: new_id,
            default_state: 0,
            model: args.resources.get_model(args.internal_name),

            states: Vec::new(),

            can_replace: false,
            light_filter: 0,
            light_emission: 0,
            renderer_type: ChunkRendererType::Opaque,
        }
    }

    pub fn get_model(&self) -> &ItemBlockModel { &self.model }

    pub fn add_states(&mut self, states: BlockStates) {
        self.states.push(states);
    }

    pub fn get_states_idx(&self, required_states: &[BlockStatesTypes]) -> Option<usize> {
        self.states.iter().position(|s| s.has(required_states))
    }

    pub fn get_states_from_idx(&self, idx: u8) -> &BlockStates {
        return &self.states[idx as usize];
    }
}
