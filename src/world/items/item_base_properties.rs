use std::{cell::RefCell, rc::Rc};
use crate::resources::{BlockItemModel, ResourceManager, TexCoords};


pub trait ItemCreation {
    type ItemType;

    fn new(internal_name: &'static str, name: &'static str, id: usize) -> Self::ItemType;
}

pub struct ItemBaseProperties {
    pub id: u16,
    pub name: &'static str,
    pub internal_name: &'static str,
    pub icon: TexCoords,

    pub block_index: Option<u32>,
    pub item_index: Option<u32>,

    model: Option<Rc<BlockItemModel>>
}

impl ItemBaseProperties {
    pub fn new(internal_name: &'static str, name: &'static str, icon: TexCoords, block_index: Option<u32>, item_index: Option<u32>) -> Self {
        static mut CURRENT_ID: u16 = 0;

        let new_id;

        unsafe {
            new_id = CURRENT_ID;
            CURRENT_ID += 1;
        }

        Self {
            id: new_id,
            name,
            internal_name,
            icon,

            block_index,
            item_index,

            model: None,
        }
    }

    pub fn get_model(&self) -> Rc<BlockItemModel> { self.model.as_ref().unwrap().clone() }

    pub fn load_model(&mut self, resources_manager: &Rc<RefCell<ResourceManager>>) {
        self.model = Some(resources_manager.borrow().get_model(self.internal_name));
    }
}
