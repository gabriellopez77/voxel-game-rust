use crate::{render::UiRenderer, ui::{ScreenStartArgs, screens::ui_common::UiCommonUpdateArgs, tools::{UiElement, inventory::SlotData}}, world::player::player_inventory::SlotType};

pub struct FlyingItem {
    item: SlotData,

    visible: bool,
}

impl FlyingItem {
    pub fn new() -> Self {
        Self {
            item: SlotData::new(),
            visible: false,
        }
    }

    pub fn star(&mut self, args: &ScreenStartArgs) {
        self.item.start(SlotType::FlyingItem, 0, args.resources.get_font("default"));
        self.item.set_size(16.0, 16.0);
    }

    pub fn update(&mut self, args: &mut UiCommonUpdateArgs) {
        self.item.set_posv(args.mouse_pos - (self.item.get_size() / 2.0));
        self.item.update(&args.game.world.player.inventory);
    }

    pub fn draw(&mut self, renderer: &mut UiRenderer) {
        self.item.draw(renderer);
    }
}
