use crate::world::{blocks::{BlockBehaviors, BlockIdState, block_registry}, player::player_inventory::ItemType};


#[derive(Clone)]
pub struct ItemStack {
    item: Option<ItemType>,
    count: i32,
}

impl ItemStack {
    pub const MAX_STACK_COUNT: i32 = 64;
    pub const EMPTY: ItemStack = ItemStack { item: None, count: 0 };

    pub fn new(item: ItemType, count: i32) -> Self {
        Self {
            item: Some(item),
            count,
        }
    }

    pub fn is_full(&self) -> bool { self.count == Self::MAX_STACK_COUNT }
    pub fn is_empty(&self) -> bool { self.item.is_none() || self.count == 0 }
    pub fn get_count(&self) -> i32 { self.count }

    pub fn set(&mut self, item: ItemType, count: i32) {
        self.item = Some(item);
        self.count = count;
    }

    pub fn get_as_block(&self) -> Option<(&'static dyn BlockBehaviors, BlockIdState)> {
        if let Some(item) = self.item {
            return match item {
                ItemType::Block(id_state) => Some((block_registry::get().get(id_state), id_state)),
                _ => None,
            }
        }

        return None;
    }

    pub fn get_item_type(&self) -> Option<ItemType> {
        self.item
    }

    pub fn is_same(&self, other: &ItemStack) -> bool {
        if let Some(ref this) = self.item && let Some(ref other) = other.item {
            return this ==  other
        }

        return false;
    }

    pub fn increment_from(&mut self, other: &mut ItemStack) {
        if !self.is_same(other) || self.is_full() { return }

        todo!();
    }

    pub fn swap(&mut self, other: &mut ItemStack) {
        let temp_item = self.item.take();
        let temp_count = self.count;

        self.item = other.item.take();
        self.count = other.count;

        other.item = temp_item;
        other.count = temp_count;
    }

    pub fn clear(&mut self) {
        self.count = 0;
        self.item = None;
    }
}
