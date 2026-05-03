use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::game::Game;
use crate::math::{self, Vec2};
use crate::render::{SpritesRenderer, Ubo};
use crate::ui::screen_base::ScreenBase;
use crate::ui::screens::start_screen::StartScreen;

pub struct ScreenManager {
    sprites_renderer: SpritesRenderer,
    sprites_ubo: Ubo,

    pixel_scale: i32,
    screen_size: Vec2,

    current_screen: Option<Rc<RefCell<dyn ScreenBase>>>,
    screens: HashMap<TypeId, Rc<RefCell<dyn ScreenBase>>>,
}

impl ScreenManager {
    pub fn new() -> Self {
        Self {
            sprites_renderer: SpritesRenderer::new(),
            sprites_ubo: Ubo::new(),

            pixel_scale: 3,
            screen_size: Vec2::ZERO,

            screens: HashMap::new(),
            current_screen: None,
        }
    }

    pub fn start(&mut self) {
        self.sprites_renderer.start();

        self.sprites_ubo.add::<math::Matrix4>("projection");
        self.sprites_ubo.create(0);

        self.screens.insert(TypeId::of::<StartScreen>(), Rc::new(RefCell::new(StartScreen::new())));

        self.change::<StartScreen>();
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.screen_size = Vec2 { x: width, y: height };

        let projection = math::Matrix4::orthographic(0.0, width, height, 0.0);
        self.sprites_ubo.update("projection", projection.as_ptr() as *const ());

        self.current_screen.as_ref().unwrap().borrow_mut().resize(width, height);
    }

    pub fn update(&mut self, dt: f32) {
        self.current_screen.as_ref().unwrap().borrow_mut().update(dt);
    }

    pub fn draw(&mut self) {
        self.current_screen.as_ref().unwrap().borrow_mut().draw(&mut self.sprites_renderer);

        self.sprites_renderer.draw()
    }

    pub fn change<T: ScreenBase>(&mut self) {
        let new_screen_id = TypeId::of::<StartScreen>();
        let new_screen = self.screens.get(&new_screen_id).unwrap().clone();

        new_screen.borrow_mut().change_logic(self.screen_size.x, self.screen_size.y);
        self.current_screen = Some(new_screen);
    }
}