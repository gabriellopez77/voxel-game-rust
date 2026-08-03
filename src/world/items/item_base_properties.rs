use std::rc::Rc;
use crate::{resources::{BlockItemMesh, ResourceManager, TexCoords}, world::player::PlayerInventory};


pub struct ItemCreationArgs<'a> {
    pub internal_name: &'static str,
    pub name: &'static str,
    pub parent_id: usize,
    pub resources: &'a ResourceManager,
    pub inventory: &'a mut PlayerInventory,
}

pub trait ItemCreation {
    type ItemType;

    fn new(args: &mut ItemCreationArgs) -> Self::ItemType;
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
    pub mesh: Rc<BlockItemMesh>,

    pub state: u8,
    pub parent_index: u32,
    pub base_type: ItemBaseType,
}

impl ItemBaseProperties {
    pub fn new(
        internal_name: &'static str,
        name: &'static str,
        mesh: Rc<BlockItemMesh>,
        parent_index: usize,
        state: u8,
        item_base_type: ItemBaseType
    ) -> Self {
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
            icon: mesh.particle_coords,

            state,
            parent_index: parent_index as u32,
            base_type: item_base_type,

            mesh,
        }
    }

    pub fn copy(&self,
        internal_name: &'static str,
        name: &'static str,
        mesh: Rc<BlockItemMesh>,
        parent_index: usize,
        state: u8,
        item_base_type: ItemBaseType
    ) -> Self {
        Self {
            id: self.id,
            internal_name,
            name,
            icon: mesh.icon_coords,
            mesh,

            state,
            parent_index: parent_index as u32,
            base_type: item_base_type,

        }
    }

    pub fn is_block(&self) -> bool { self.base_type == ItemBaseType::Block }
    pub fn is_item(&self) -> bool { self.base_type == ItemBaseType::Item }
}
