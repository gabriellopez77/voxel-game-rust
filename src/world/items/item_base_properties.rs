use std::{cell::RefCell, rc::Rc};
use crate::resources::{BlockItemModel, ResourceManager, TexCoords};


pub trait ItemCreation {
    type ItemType;

    fn new(internal_name: &'static str, name: &'static str, index: usize, resources: &ResourceManager) -> Self::ItemType;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ItemBaseType {
    Item,
    Block
}

pub struct ItemBaseProperties {
    pub id: u16,
    pub internal_name: &'static str,
    pub name: &'static str,
    pub icon: TexCoords,
    pub model: Rc<BlockItemModel>,

    pub state: u8,
    pub parent_index: u32,
    pub base_type: ItemBaseType,
}

impl ItemBaseProperties {
    pub fn new(internal_name: &'static str, name: &'static str, model: Rc<BlockItemModel>,
               parent_index: usize, state: u8, item_base_type: ItemBaseType) -> Self {
        static mut CURRENT_ID: u16 = 0;

        let new_id;

        // SAFETY: called only on the main thread
        unsafe {
            new_id = CURRENT_ID;
            CURRENT_ID += 1;
        }

        Self {
            id: new_id,
            name,
            internal_name,
            icon: model.particle_tex_coords,

            state,
            parent_index: parent_index as u32,
            base_type: item_base_type,

            model,
        }
    }
    
    pub fn copy(&self, internal_name: &'static str, name: &'static str, model: Rc<BlockItemModel>,
                parent_index: usize, state: u8, item_base_type: ItemBaseType) -> Self {
        Self {
            id: self.id,
            internal_name,
            name,
            icon: model.icon_tex_coords,
            model,
            
            state,
            parent_index: parent_index as u32,
            base_type: item_base_type,
            
        }
    }

    pub fn is_block(&self) -> bool { self.base_type == ItemBaseType::Block }
    pub fn is_item(&self) -> bool { self.base_type == ItemBaseType::Item }
}
