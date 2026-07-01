use crate::game::GameEvents;
use crate::math::Color3b;
use crate::render::UiRenderer;
use crate::ui::tools::{Button, UiElement};
use crate::ui::{ScreenBase, ScreenResizeArgs, ScreenStartArgs, ScreenUpdateArgs};


pub struct PauseScreen {
    quit_button: Button,
}

impl ScreenBase for PauseScreen {
    fn start(&mut self, args: &ScreenStartArgs) {
        self.quit_button.set_size(145.0, 25.0);
        self.quit_button.text.set_font(args.resources.get_font("default"));
        self.quit_button.text.set_text("Quit");
        self.quit_button.text.set_color(Color3b::new(76, 76, 76));
    }

    fn update(&mut self, dt: f32, args: &mut ScreenUpdateArgs) {
        if self.quit_button.update(args.mouse_pos) {
            args.game.add_event(GameEvents::QuitGame);
        }
    }

    fn draw(&mut self, renderer: &mut UiRenderer) {
        self.quit_button.draw(renderer);
    }

    fn resize(&mut self, args: &ScreenResizeArgs) {
        self.quit_button.set_posv(args.screen_center - self.quit_button.get_size() / 2.0);
    }
}

impl PauseScreen {
    pub fn new() -> Self {
        Self {
            quit_button: Button::new(),
        }
    }
}
