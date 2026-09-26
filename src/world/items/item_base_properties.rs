use crate::{
    resources::{ResourceManager},
    world::{
        player::PlayerInventory
    }
};


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

#[derive(Clone)]
pub struct ItemBaseProperties {

}

impl ItemBaseProperties {
}
