use std::cell::RefCell;
use crate::game::Game;
use crate::inputs::Inputs;
use crate::math::Vec2;
use crate::render::UiRenderer;
use crate::resources::ResourceManager;
use crate::ui::screens::UiCommon;
use crate::ui::ui_manager::ScreensId;


pub trait ScreenBase {
    fn start(&mut self, args: &ScreenStartArgs);
    fn update(&mut self, args: &mut ScreenUpdateArgs);
    fn draw(&mut self, renderer: &mut UiRenderer);
    fn resize(&mut self, args: &ScreenResizeArgs);
}

pub struct ScreenInfo {
    pub screen_size: Vec2,
    pub screen_center: Vec2,
    pub started: bool,
    pub id: ScreensId,

    pub screen: Box<RefCell<dyn ScreenBase>>,
}

impl ScreenInfo {
    pub fn new(screen: Box<RefCell<dyn ScreenBase>>, id: ScreensId) -> Self {
        Self {
            screen_size: Vec2::ZERO,
            screen_center: Vec2::ZERO,
            started: false,
            id,

            screen,
        }
    }
}


pub struct ScreenStartArgs<'a> {
    pub resources: &'a ResourceManager,

    pub game: &'a Game,
}

pub struct ScreenUpdateArgs<'a> {
    pub dt: f32,

    pub screen_size: Vec2,
    pub screen_center: Vec2,

    pub mouse_pos: Vec2,

    pub game: &'a mut Game,
    pub inputs: &'a Inputs,
    pub ui_common: &'a mut UiCommon,
}

pub struct ScreenResizeArgs<'a> {
    pub screen_size: Vec2,
    pub screen_center: Vec2,

    pub game: &'a Game,
}
