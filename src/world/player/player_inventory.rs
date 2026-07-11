use std::{array, sync::Arc};

use crate::world::{items::ItemBaseProperties, player::ItemStack};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SlotType {
    Inventory,
    Creative,
    FlyingItem,
}

pub struct PlayerInventory {
    creative_inventory: Vec<ItemStack>,
    inventory: [ItemStack; Self::SLOTS_COUNT_TOTAL],

    flying_item: ItemStack,

    selected_hotbar_slot: i32,
}

impl PlayerInventory {
    pub const HOTBAR_SLOTS_COUNT: usize = 9;
    pub const INVENTORY_SLOTS_COUNT: usize = 27;
    pub const SLOTS_COUNT_TOTAL: usize = Self::HOTBAR_SLOTS_COUNT + Self::INVENTORY_SLOTS_COUNT;

    pub fn new() -> Self {
        Self {
            creative_inventory: Vec::new(),
            inventory: array::from_fn(|_| ItemStack::EMPTY),

            flying_item: ItemStack::EMPTY,

            selected_hotbar_slot: 0,
        }
    }

    pub fn process_hotbar_scroll(&mut self, scroll: i32) {
        self.selected_hotbar_slot -= scroll;

        if self.selected_hotbar_slot < 0 {
            self.selected_hotbar_slot = (Self::HOTBAR_SLOTS_COUNT - 1) as i32;
        }
        else if self.selected_hotbar_slot >= Self::HOTBAR_SLOTS_COUNT as i32 {
            self.selected_hotbar_slot = 0;
        }
    }

    pub fn process_left_click(&mut self, slot_index: i32, slot_type: SlotType) {
        debug_assert!(slot_type != SlotType::FlyingItem, "invalid slot type");

        if slot_type == SlotType::Creative {
            if !self.flying_item.is_empty() {
                self.flying_item.clear();
            }
            else {
                self.flying_item = self.get_slot(slot_index, slot_type).clone();
            }
        }
        else {
            self.inventory[slot_index as usize].swap(&mut self.flying_item);
        }
    }

    pub fn clear_flying_item(&mut self) {
        self.flying_item.clear();
    }

    /// used for register item in creative inventory
    pub fn register_item(&mut self, item: Arc<ItemBaseProperties>) {
        self.creative_inventory.push(ItemStack::new(item, 1));
    }


    pub fn get_slot(&self, index: i32, slot_type: SlotType) -> &ItemStack {
        match slot_type {
            SlotType::Inventory => &self.inventory[index as usize],
            SlotType::Creative => &self.creative_inventory[index as usize],
            SlotType::FlyingItem => &self.flying_item,
        }
    }

    pub fn get_creative_items_count(&self) -> usize {
        self.creative_inventory.len()
    }

    pub fn get_selected_hotbar_index(&self) -> i32 {
        self.selected_hotbar_slot
    }

    pub fn get_selected_hotbar_slot(&self) -> &ItemStack {
        &self.inventory[self.selected_hotbar_slot as usize]
    }

    pub fn get_flying_item(&self) -> &ItemStack {
        &self.flying_item
    }
}
