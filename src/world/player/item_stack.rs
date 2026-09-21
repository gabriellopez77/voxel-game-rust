use std::sync::Arc;
use crate::world::items::ItemBaseProperties;


#[derive(Clone)]
pub struct ItemStack {
    item: Option<Arc<ItemBaseProperties>>,
    count: i32,
}

impl ItemStack {
    pub const MAX_STACK_COUNT: i32 = 64;
    pub const EMPTY: ItemStack = ItemStack { item: None, count: 0 };

    pub fn new(item: Arc<ItemBaseProperties>, count: i32) -> Self {
        Self {
            item: Some(item),
            count,
        }
    }

    pub fn is_full(&self) -> bool { self.count == Self::MAX_STACK_COUNT }
    pub fn is_empty(&self) -> bool { self.item.is_none() || self.count == 0 }
    pub fn get_count(&self) -> i32 { self.count }

    pub fn set(&mut self, item: Arc<ItemBaseProperties>, count: i32) {
        self.item = Some(item);
        self.count = count;
    }

    pub fn get_item(&self) -> Option<&Arc<ItemBaseProperties>> {
        self.item.as_ref()
    }

    pub fn is_same(&self, other: &ItemStack) -> bool {
        if let Some(ref this) = self.item && let Some(ref other) = other.item {
            return this.get_id_state().id ==  other.get_id_state().id
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
