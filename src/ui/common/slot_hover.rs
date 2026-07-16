use crate::{render::UiRenderer, ui::{ScreenStartArgs, tools::{Slice, UiElement}}};


pub struct SlotHover {
    background: Slice,
    visible: bool,
}

impl SlotHover {
    pub fn new() -> Self {
        Self {
            background: Slice::new(),
            visible: false,
        }
    }

    pub fn start(&mut self, args: &ScreenStartArgs) {
        self.background.set_texture(&args.resources.ui_sprites_texture, "inventory_slot_hover", 2);
    }

    pub fn draw(&mut self, renderer: &mut UiRenderer) {
        if self.visible {
            self.background.draw(renderer);
        }

        self.visible = false;
    }

    pub fn set(&mut self, slot: &dyn UiElement) {
        self.background.set_posv(slot.get_pos());
        self.background.set_sizev(slot.get_size());

        self.visible = true;
    }
}
