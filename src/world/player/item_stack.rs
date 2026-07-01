use std::sync::Arc;
use crate::world::items::ItemBaseProperties;


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
    pub fn is_empty(&self) -> bool { self.count == 0 }
    pub fn get_count(&self) -> i32 { self.count }

    pub fn set(&mut self, item: Arc<ItemBaseProperties>, count: i32) {
        self.item = Some(item);
        self.count = count;
    }

    pub fn get_item(&self) -> &Arc<ItemBaseProperties> {
        if let Some(item) = &self.item {
            return item;
        }

        panic!("ItemStack is none");
    }

    pub fn is_same(&self, other: &ItemStack) -> bool {
        if self.is_empty() || other.is_empty() { return false; }

        return self.item.as_ref().unwrap().id == other.item.as_ref().unwrap().id;
    }

    pub fn increment_from(&mut self, other: &mut ItemStack) {
        if !self.is_same(other) || self.is_full() { return }

        todo!();
    }

    pub fn swap(&mut self, other: &mut ItemStack) {
        let temp_item = self.item.take();
        let temp_count = other.count;

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
