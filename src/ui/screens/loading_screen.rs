use crate::math::Color4b;
use crate::render::UiRenderer;
use crate::ui::tools::{Sprite, Text, UiElement};
use crate::ui::{ScreenBase, ScreenResizeArgs, ScreenStartArgs, ScreenUpdateArgs};


pub struct LoadingScreen {
    background: Sprite,
    loading_text: Text,
}

impl ScreenBase for LoadingScreen {
    fn start(&mut self, args: &ScreenStartArgs) {
        self.background.color = Color4b::new(0, 94, 97, 255);

        self.loading_text.set_font(args.resources.get_font("default"));
        self.loading_text.set_text("Loading World...");

    }

    fn update(&mut self, dt: f32, args: &mut ScreenUpdateArgs) {

    }

    fn draw(&mut self, renderer: &mut UiRenderer) {
        self.background.draw(renderer);
        self.loading_text.draw(renderer);
    }

    fn resize(&mut self, args: &ScreenResizeArgs) {
        self.background.set_sizev(args.screen_size + 1.0);

        self.loading_text.set_posv(args.screen_center - self.loading_text.get_size() / 2.0);
    }
}

impl LoadingScreen {
    pub fn new() -> Self {
        Self {
            background: Sprite::new(),
            loading_text: Text::new(),
        }
    }
}
