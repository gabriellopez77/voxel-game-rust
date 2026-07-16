use crate::{render::UiRenderer, ui::{ScreenStartArgs, screens::ui_common::UiCommonUpdateArgs, tools::{UiElement, inventory::SlotData}}, world::player::player_inventory::SlotType};

pub struct FlyingItem {
    item: SlotData,
}

impl FlyingItem {
    pub fn new() -> Self {
        Self {
            item: SlotData::new(),
        }
    }

    pub fn start(&mut self, args: &ScreenStartArgs) {
        self.item.start(SlotType::FlyingItem, 0, args.resources.get_font("default"));
        self.item.set_size(16.0, 16.0);
    }

    pub fn update(&mut self, args: &mut UiCommonUpdateArgs) {
        self.item.update(&args.game.world.player.inventory);
        self.item.set_posv(args.mouse_pos - (self.item.get_size() / 2.0));
    }

    pub fn draw(&mut self, renderer: &mut UiRenderer) {
        self.item.draw(renderer);
    }
}
