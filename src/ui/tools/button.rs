use crate::{inputs::{self, InputActions, Inputs}, math::Vec2, render::UiRenderer, ui::{buttons_styles::ButtonStyleInfo, tools::{Slice, Text, UiElement}}};


pub struct Button {
    pos: Vec2,
    size: Vec2,

    background: Slice,
    pub text: Text,

    locked: bool,
    pressed: bool,

    style: Option<ButtonStyleInfo>,
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

            style: None,
        }
    }

    pub fn set_style(&mut self, style: ButtonStyleInfo) {
        self.style = Some(style);
    }

    pub fn update(&mut self, mouse_pos: Vec2, inputs: &Inputs) -> bool {
        self.pressed = false;
        if self.locked { return false }

        let hover = self.mouse_hover(mouse_pos);
        let action = inputs.get_mouse_action(inputs::MouseButton::Left);

        let style = self.style.as_ref().unwrap();

        if hover {
            if action == InputActions::Repeat {
                self.pressed = true;
                self.background.set_texture_from_coords(style.pressed_tex, style.pressed_corner, style.pressed_corner_norm);
            }
            else {
                self.background.set_texture_from_coords(style.hover_tex, style.hover_corner, style.hover_corner_norm);
            }
        }
        else {
            self.background.set_texture_from_coords(style.default_tex, style.default_corner, style.default_corner_norm);
        }

        return hover && action == InputActions::Release;
    }

    pub fn draw(&mut self, renderer: &mut UiRenderer) {
        self.background.draw(renderer);

        let temp_pos = self.text.get_pos();

        if self.pressed {
            self.text.set_posv(temp_pos + Vec2::new(0.0, 2.0));
        }

        self.text.draw(renderer);
        self.text.set_posv(temp_pos);
    }

    //pub fn lock(&mut self) {
    //    self.locked = true;
    //    self.background.color.a = 128;
    //}

    //pub fn unlock(&mut self) {
    //    self.locked = false;
    //    self.background.color.a = 255;
    //}
}
