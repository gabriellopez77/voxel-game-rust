use std::{rc::Rc, cell::RefCell};

use crate::math::{Color3b, Color4b, Vec2};
use crate::render::{SpritesRenderer, SpritesVertices, TextVertices};
use crate::resources::{ResourceManager, TexCoords};
use crate::ui::{ScreenBase, tools::{Sprite, UiElement}};
use crate::ui::tools::Text;


pub struct StartScreen {
    screen_size: Vec2,
    screen_center: Vec2,
    started: bool,

    text: Text,
}

impl ScreenBase for StartScreen {
    fn start(&mut self, resource_manager: Rc<RefCell<ResourceManager>>) {
        self.text.set_font(resource_manager.borrow().get_font("default_font").unwrap());
        self.text.set_text("Hello, World".to_string());
        self.text.set_color(Color3b::WHITE);
    }

    fn update(&mut self, dt: f32) {

    }

    fn draw(&mut self, sprite_renderer: &mut SpritesRenderer<SpritesVertices>, text_renderer: &mut SpritesRenderer<TextVertices>) {
        self.text.draw(text_renderer)
    }

    fn resize(&mut self, width: f32, height: f32) {

    }

    fn change_logic(&mut self, width: f32, height: f32, resource_manager: Rc<RefCell<ResourceManager>>) {
        let new_screen_size = Vec2{ x: width, y: height };

        if !self.started {
            self.started = true;
            self.screen_size = new_screen_size;
            self.screen_center = new_screen_size / 2.0;
            self.start(resource_manager.clone());

            // not resize if screen size in zero
            if new_screen_size != Vec2::ZERO {
                self.resize(width, height);
            }
        }

        if self.screen_size != new_screen_size {
            self.screen_size = new_screen_size;
            self.screen_center = new_screen_size / 2.0;
            self.resize(width, height);
        }
    }
}

impl StartScreen {
    pub fn new() -> Self {
        Self {
            screen_size: Vec2::ZERO,
            screen_center: Vec2::ZERO,
            started: false,

            text: Text::new()
        }
    }
}