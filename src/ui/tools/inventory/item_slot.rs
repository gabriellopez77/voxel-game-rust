use std::rc::Rc;
use crate::math::Vec2;
use crate::render::{Texture, UiRenderer};
use crate::resources::FontInfo;
use crate::ui::tools::{Sprite, UiElement, inventory::SlotData};
use crate::world::player::EntityInventory;


pub struct ItemSlot {
    position: Vec2,
    size: Vec2,

    slot_data: SlotData,
    background: Sprite,
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
            background: Sprite::new(),
        }
    }

    pub fn start(&mut self, slot_index: i32, tex: &Texture, name: &'static str, text_font: Rc<FontInfo>) {
        self.background.set_texture(tex, name);
        self.slot_data.start(slot_index, text_font);
    }

    pub fn update(&mut self, inventory: &dyn EntityInventory) {
        self.slot_data.update(inventory);
    }

    pub fn draw(&mut self, renderer: &mut UiRenderer) {
        self.background.draw(renderer);
        self.slot_data.draw(renderer);
    }
}
