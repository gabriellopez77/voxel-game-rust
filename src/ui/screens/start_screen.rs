use crate::game::GameEvents;
use crate::math::Color3b;
use crate::render::UiRenderer;
use crate::ui::tools::{Button, UiElement};
use crate::ui::{ScreenBase, ScreenResizeArgs, ScreenStartArgs, ScreenUpdateArgs};


pub struct StartScreen {
    start_button: Button,
    quit_button: Button,
}

impl ScreenBase for StartScreen {
    fn start(&mut self, args: &ScreenStartArgs) {
        self.start_button.set_size(145.0, 25.0);
        self.start_button.text.set_font(args.resources.get_font("default"));
        self.start_button.text.set_text("Start");
        self.start_button.text.set_color(Color3b::new(255, 255, 76));

        self.quit_button.set_size(145.0, 25.0);
        self.quit_button.text.set_font(args.resources.get_font("default"));
        self.quit_button.text.set_text("Quit");
        self.quit_button.text.set_color(Color3b::new(76, 76, 76));
    }

    fn update(&mut self, dt: f32, args: &mut ScreenUpdateArgs) {
        if self.start_button.update(args.mouse_pos) {
            args.game.add_event(GameEvents::EnterInWorld);
        }

        if self.quit_button.update(args.mouse_pos) {
            args.game.add_event(GameEvents::QuitGame);
        }
    }

    fn draw(&mut self, renderer: &mut UiRenderer) {
        self.start_button.draw(renderer);
        self.quit_button.draw(renderer);
    }

    fn resize(&mut self, args: &ScreenResizeArgs) {
        self.start_button.set_posv(args.screen_center - self.start_button.get_size() / 2.0);
        self.quit_button.set_pos(
            self.start_button.get_pos().x,
            self.start_button.get_finaly() + 3.0
        );
    }
}

impl StartScreen {
    pub fn new() -> Self {
        Self {
            start_button: Button::new(),
            quit_button: Button::new(),
        }
    }
}
