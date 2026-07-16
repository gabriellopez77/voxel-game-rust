use crate::{render::UiRenderer, ui::{ScreenStartArgs, tools::{Slice, Text, UiElement}}, world::player::ItemStack};


pub struct SlotHoverInfo {
    background: Slice,
    item_name: Text,

    visible: bool,
}

impl SlotHoverInfo {
    pub fn new() -> Self {
        Self {
            background: Slice::new(),
            item_name: Text::new(),
            visible: false,
        }
    }

    pub fn start(&mut self, args: &ScreenStartArgs) {
        self.background.set_texture(&args.resources.ui_sprites_texture, "item_name_panel", 2);
        self.item_name.set_font(args.resources.get_font("default"));
    }

    pub fn draw(&mut self, renderer: &mut UiRenderer) {
        if !self.visible { return }

        self.background.draw(renderer);
        self.item_name.draw(renderer);

        self.visible = false;
    }

    pub fn set(&mut self, slot: &dyn UiElement, stack: &ItemStack) {
        if stack.is_empty() { return }

        self.item_name.set_text(stack.get_item().name);

        self.background.set_sizev(self.item_name.get_size() + 8.0);
        self.background.set_pos(
            self.background.get_centerx(slot),
            slot.get_pos().y - self.background.get_size().y
        );

        self.item_name.set_center(&self.background);

        self.visible = true;
    }
}
