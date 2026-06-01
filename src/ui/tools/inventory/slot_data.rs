use std::rc::Rc;
use crate::math::Vec2;
use crate::render::UiRenderer;
use crate::resources::FontInfo;
use crate::ui::tools::{Sprite, Text, UiElement};
use crate::world::player::EntityInventory;


pub struct SlotData {
    position: Vec2,
    size: Vec2,

    slot_index: i32,

    count_text: Text,
    icon: Sprite,

    last_count: i32,
    count_text_visible: bool,
    visible: bool,
}

impl UiElement for SlotData {
    fn get_pos(&self) -> Vec2 { self.position }
    fn get_size(&self) -> Vec2 { self.size }

    fn set_pos(&mut self, x: f32, y: f32) {
        self.position = Vec2::new(x, y);

        self.icon.set_posv(self.icon.get_center(self));
        self.count_text.set_posv(self.icon.get_final() - self.count_text.get_size());
    }

    fn set_size(&mut self, x: f32, y: f32) {
        self.size = Vec2::new(x, y);

        self.icon.set_size(16.0, 16.0);
    }
}

impl SlotData {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,

            slot_index: 0,

            count_text: Text::new(),
            icon: Sprite::new(),

            last_count: 0,
            count_text_visible: false,
            visible: false,
        }
    }

    pub fn start(&mut self, slot_index: i32, text_font: Rc<FontInfo>) {
        self.slot_index = slot_index;

        self.count_text.set_font(text_font);
    }

    pub fn update(&mut self, inventory: &dyn EntityInventory) {
        let slot = inventory.get_slot(self.slot_index);

        if slot.is_empty() {
            self.visible = false;
            return;
        }

        let item_count = slot.get_count();

        self.visible = true;
        self.count_text_visible = item_count > 1;


        // avoids update text chars mesh in every frame
        if self.last_count != item_count {
            self.count_text.set_text_i32(item_count);
            self.count_text.set_posv(self.icon.get_final());
        }

        self.icon.set_texture(slot.get_item().icon);

        self.last_count = item_count;
    }

    pub fn draw(&mut self, renderer: &mut UiRenderer) {
        if !self.visible { return }

        self.icon.draw(&mut renderer.icons);

        if self.count_text_visible {
            self.count_text.draw(&mut renderer.text);
        }

    }
}