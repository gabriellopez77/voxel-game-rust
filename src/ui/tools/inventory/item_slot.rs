use std::rc::Rc;
use crate::inputs::{self, Inputs};
use crate::math::Vec2;
use crate::render::{Texture, UiRenderer};
use crate::resources::FontInfo;
use crate::ui::screens::UiCommon;
use crate::ui::tools::Slice;
use crate::ui::tools::{UiElement, inventory::SlotData};
use crate::world::player::PlayerInventory;
use crate::world::player::player_inventory::SlotType;


pub struct ItemSlot {
    position: Vec2,
    size: Vec2,

    slot_data: SlotData,
    background: Slice,
}

impl UiElement for ItemSlot {
    fn get_pos(&self) -> Vec2 { self.position }
    fn get_size(&self) -> Vec2 { self.size }

    fn set_pos(&mut self, x: f32, y: f32) {
        self.position = Vec2::new(x, y);

        self.background.set_pos(x, y);
        self.slot_data.set_posv(self.slot_data.get_center(&self.background));
    }

    fn set_size(&mut self, x: f32, y: f32) {
        self.size = Vec2::new(x, y);

        self.background.set_size(x, y);
        self.slot_data.set_size(x, y);
    }
}

impl ItemSlot {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,

            slot_data: SlotData::new(),
            background: Slice::new(),
        }
    }

    pub fn start(&mut self, slot_type: SlotType, slot_index: usize, tex: &Texture, name: &'static str, text_font: Rc<FontInfo>) {
        self.background.set_texture(tex, name, 2);
        self.slot_data.start(slot_type, slot_index as i32, text_font);
    }

    pub fn update(&mut self, inventory: &mut PlayerInventory, mouse_pos: Vec2, inputs: &Inputs, ui_common: &mut UiCommon) {
        if self.mouse_hover(mouse_pos) {
            ui_common.slot_hover.set(self);

            if inputs.mouse_pressed(inputs::MouseButton::Left) {
                inventory.process_left_click(self.slot_data.slot_index, self.slot_data.slot_type);
            }

            ui_common.slot_hover_info.set(self, inventory.get_slot(self.slot_data.slot_index, self.slot_data.slot_type));
        }

        self.slot_data.update(inventory);
    }

    pub fn draw(&mut self, renderer: &mut UiRenderer) {
        self.background.draw(renderer);
        self.slot_data.draw(renderer);
    }
}
