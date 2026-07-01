use crate::{inputs, math::{Color3b, Vec2}, render::UiRenderer, ui::tools::{Slice, Text, UiElement}};


pub struct Button {
    pos: Vec2,
    size: Vec2,

    background: Slice,
    pub text: Text,

    locked: bool,
    pressed: bool,
}

impl UiElement for Button {
    fn get_pos(&self) -> Vec2 { self.pos }
    fn get_size(&self) -> Vec2 { self.size }

    fn set_pos(&mut self, x: f32, y: f32) {
        self.pos = Vec2::new(x, y);

        self.background.set_pos(x, y);
        self.text.set_center(&self.background);
    }

    fn set_size(&mut self, x: f32, y: f32) {
        self.size = Vec2::new(x, y);

        self.background.set_size(x, y);
        self.text.set_center(&self.background);
    }
}

impl Button {
    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            size: Vec2::ZERO,

            background: Slice::new(),
            text: Text::new(),

            locked: false,
            pressed: false,
        }
    }

    pub fn set_texture(&mut self) {

    }

    pub fn update(&mut self, mouse_pos: Vec2) -> bool {
        if self.locked { return false }

        let hover = self.mouse_hover(mouse_pos);


        return inputs::mouse_button_pressed(inputs::MouseButton::Left) && hover;
    }

    pub fn draw(&mut self, renderer: &mut UiRenderer) {
        self.background.draw(renderer);
        self.text.draw(renderer);
    }

    pub fn lock(&mut self) {
        self.locked = true;
        self.background.color.a = 128;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
        self.background.color.a = 255;
    }
}
