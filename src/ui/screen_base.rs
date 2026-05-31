use std::{cell::RefCell, rc::Rc};
use crate::game::Game;
use crate::math::Vec2;
use crate::render::UiRenderer;
use crate::resources::ResourceManager;


pub trait ScreenBase {
    fn start(&mut self, resource_manager: &ResourceManager, args: &ScreenUpdateArgs);
    fn update(&mut self, dt: f32, args: &ScreenUpdateArgs);
    fn draw(&mut self, renderer: &mut UiRenderer);
    fn resize(&mut self, args: &ScreenUpdateArgs);
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

pub struct ScreenUpdateArgs<'a> {
    pub screen_size: Vec2,
    pub screen_center: Vec2,
    
    pub game: &'a Game,
}