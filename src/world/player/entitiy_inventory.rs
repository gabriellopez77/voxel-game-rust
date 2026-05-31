use crate::world::player::ItemStack;


pub const PLAYER_HOTBAR_SLOTS_COUNT: usize = 9;
pub const PLAYER_INVENTORY_SLOTS_COUNT: usize = 27;
pub const PLAYER_SLOTS_COUNT_TOTAL: usize = PLAYER_HOTBAR_SLOTS_COUNT + PLAYER_INVENTORY_SLOTS_COUNT;

pub trait EntityInventory {
    fn get_slot(&self, index: i32) -> &ItemStack;

    fn get_selected_hotbar_slot(&self) -> &ItemStack {
        self.get_slot(self.get_selected_hotbar_index())
    }
    
    fn get_selected_hotbar_index(&self) -> i32;

}