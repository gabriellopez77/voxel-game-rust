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
        self.quit_button.set_style(args.resources.ui_buttons_styles.get("red_button"));
        self.quit_button.set_size(145.0, 25.0);
        self.quit_button.text.set_font(args.resources.get_font("default"));
        self.quit_button.text.set_text("Leave the World");
        self.quit_button.text.set_color(Color3b::WHITE);
    }

    fn update(&mut self, dt: f32, args: &mut ScreenUpdateArgs) {
        if self.quit_button.update(args.mouse_pos) {
            args.game.add_event(GameEvents::LeaveToWorld);
        }
    }

    fn draw(&mut self, renderer: &mut UiRenderer) {
        self.quit_button.draw(renderer);
    }

    fn resize(&mut self, args: &ScreenResizeArgs) {
        self.quit_button.set_pos(
            args.screen_center.x - self.quit_button.get_size().x / 2.0,
            args.screen_size.y - self.quit_button.get_size().y - 20.0
        );
    }
}

impl PauseScreen {
    pub fn new() -> Self {
        Self {
            quit_button: Button::new(),
        }
    }
}
