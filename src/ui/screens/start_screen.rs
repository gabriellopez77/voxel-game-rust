use crate::math::{Vec2, Color4b};
use crate::render::{SpritesRenderer};
use crate::ui::screen_base::ScreenBase;
use crate::ui::tools::sprites::Sprite;
use crate::ui::tools::ui_element::UiElement;


pub struct StartScreen {
    screen_size: Vec2,
    screen_center: Vec2,
    started: bool,

    teste: Sprite,
}

impl ScreenBase for StartScreen {
    fn start(&mut self) {
        self.teste.color = Color4b { r: 123, g: 53, b: 98, a: 255 };
        self.teste.set_size(100.0, 100.0);
    }

    fn update(&mut self, dt: f32) {

    }

    fn draw(&mut self, renderer: &mut SpritesRenderer) {
        self.teste.draw(renderer);
    }

    fn resize(&mut self, width: f32, height: f32) {
        println!("Rsize: {}, {}", width, height);
    }

    fn change_logic(&mut self, width: f32, height: f32) {
        let new_screen_size = Vec2{ x: width, y: height };

        if !self.started {
            self.started = true;
            self.screen_size = new_screen_size;
            self.screen_center = new_screen_size / 2.0;
            self.start();

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

            teste: Sprite::new()
        }
    }
}