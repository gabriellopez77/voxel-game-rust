use std::{cell::RefCell, rc::Rc};
use crate::game::Game;
use crate::math::Vec2;
use crate::render::UiRenderer;
use crate::resources::ResourceManager;


pub trait ScreenBase {
    fn start(&mut self, args: &ScreenStartArgs);
    fn update(&mut self, dt: f32, args: &mut ScreenUpdateArgs);
    fn draw(&mut self, renderer: &mut UiRenderer);
    fn resize(&mut self, args: &ScreenResizeArgs);
}

pub struct ScreenInfo {
    pub screen_size: Vec2,
    pub screen_center: Vec2,
    pub started: bool,

    pub screen: Rc<RefCell<dyn ScreenBase>>,
}

impl ScreenInfo {
    pub fn new(screen: Rc<RefCell<dyn ScreenBase>>) -> Self {
        Self {
            screen_size: Vec2::ZERO,
            screen_center: Vec2::ZERO,
            started: false,

            screen,
        }
    }
}


pub struct ScreenStartArgs<'a> {
    pub screen_size: Vec2,
    pub screen_center: Vec2,

    pub resources: &'a ResourceManager,

    pub game: &'a Game,
}

pub struct ScreenUpdateArgs<'a> {
    pub screen_size: Vec2,
    pub screen_center: Vec2,

    pub mouse_pos: Vec2,

    pub game: &'a mut Game,
}

pub struct ScreenResizeArgs<'a> {
    pub screen_size: Vec2,
    pub screen_center: Vec2,

    pub game: &'a Game,
}
