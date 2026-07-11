use crate::game::Game;
use crate::inputs::Inputs;
use crate::math::Vec2;
use crate::render::UiRenderer;
use crate::ui::common::{FlyingItem, SlotHover};
use crate::ui::{ScreenResizeArgs, ScreenStartArgs};


pub struct UiCommonUpdateArgs<'a> {
    pub dt: f32,

    pub screen_size: Vec2,
    pub screen_center: Vec2,

    pub mouse_pos: Vec2,

    pub game: &'a mut Game,
    pub inputs: &'a Inputs,
}

pub struct UiCommon {
    pub slot_hover: SlotHover,
    pub flying_item: FlyingItem,
}

impl UiCommon {
    pub fn new() -> Self {
        Self {
            slot_hover: SlotHover::new(),
            flying_item: FlyingItem::new(),
        }
    }

    pub fn start(&mut self, args: &ScreenStartArgs) {
        self.slot_hover.start(args);
        self.flying_item.star(args);
    }

    pub fn update(&mut self, args: &mut UiCommonUpdateArgs) {
        self.flying_item.update(args);
    }

    pub fn draw(&mut self, renderer: &mut UiRenderer) {
        self.slot_hover.draw(renderer);
        self.flying_item.draw(renderer);
    }

    pub fn resize(&mut self, args: &ScreenResizeArgs) {

    }
}
