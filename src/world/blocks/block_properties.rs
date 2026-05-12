use crate::resources::TextureCoords;
use crate::world::items::*;


pub trait BlockCreation {
    type BlockType;

    fn new(internal_name: &'static str, name: &'static str, id: usize) -> Self::BlockType;
}

pub trait BlockFunctions {
    fn get_properties(&self) -> &BlockProperties;

    fn get_base(&self) -> &ItemBaseProperties { &self.get_properties().base_properties }
}

pub struct BlockProperties {
    pub can_replaced: bool,
    pub is_transparent: bool,
    pub light_filter: u8,
    pub light_emission: u8,

    pub base_properties: ItemBaseProperties
}

impl BlockProperties {
    pub fn new(internal_name: &'static str, name: &'static str, index: usize) -> BlockProperties {
        Self {
            can_replaced: false,
            is_transparent: false,
            light_filter: 0,
            light_emission: 0,
            base_properties: ItemBaseProperties::new(internal_name, name, TextureCoords::ZERO, Some(index as u32), None)
        }
    }
}