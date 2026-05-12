use crate::{math::{Color3b, Vec2, Vec2i16}, render::{self, SpritesRenderer, TextVertices}, ui::tools::UiElement};

pub struct Text {
    position: Vec2,
    size: Vec2,
    color: Color3b,

    text: String,

    pos_modified: bool,
    color_modified: bool,
    delay: f32,

    buffer: Vec<TextVertices>,
}

impl UiElement for Text {
    fn get_pos(&self) -> Vec2 { self.position }
    fn set_pos(&mut self, x: f32, y: f32) {
        
    }

    fn get_size(&self) -> Vec2 { self.size }
    fn set_size(&mut self, x: f32, y: f32) {
        
    }
}

impl Text {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            color: Color3b::ZERO,

            text: String::new(),

            pos_modified: false,
            color_modified: false,
            delay: 0.0,

            buffer: Vec::new()
        }
    }

    pub fn draw(&mut self, renderer: &mut SpritesRenderer<TextVertices>) {
        if self.text.len() == 0 { return }

        if self.pos_modified {
            self.pos_modified = false;

            let pos = Vec2i16::new(self.position.x as i16, self.position.y as i16);

            for i in 0..self.buffer.len() as usize {
                self.buffer[i].position = pos;
            }
        }

        if self.color_modified {
            self.color_modified = false;

            for i in 0..self.buffer.len() as usize {
                self.buffer[i].color = self.color;
            }
        }

        // add text data to render buffer
        for item in &self.buffer {
            renderer.add_element(*item);
        }
    }
}