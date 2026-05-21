use std::rc::Rc;
use crate::{math::{Color3b, Vec2, Vec2i16}, render::{self, SpritesRenderer, TextVertices}, ui::tools::UiElement};
use crate::resources::FontInfo;


pub struct Text {
    position: Vec2,
    size: Vec2,
    color: Color3b,

    text: String,

    pos_modified: bool,
    color_modified: bool,
    delay: f32,

    buffer: Vec<TextVertices>,
    font_info: Option<Rc<FontInfo>>,
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

            buffer: Vec::new(),
            font_info: None,
        }
    }

    pub fn set_font(&mut self, font: Rc<FontInfo>) {
        self.font_info = Some(font);
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.update_mesh();
    }
    
    pub fn set_color(&mut self, color: Color3b) {
        self.color = color;
        self.color_modified = true;
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

    fn update_mesh(&mut self) {
        let mut advance_x: i16 = 0;
        let mut advance_y: i16 = 8;
        let mut max_advance_x = advance_x;


        self.buffer.clear();
        self.color_modified = false;
        self.pos_modified = false;

        let font_info = self.font_info.as_ref().unwrap();

        for ch in self.text.chars() {
            // breakline
            if ch == '\n' {
                advance_x = 0;
                advance_y += 11;

                continue;
            }

            let char_info = font_info.get_info(ch);

            let pos = self.get_pos();
            let text_vertices = TextVertices{
                position: Vec2i16::new(pos.x as i16, pos.y as i16),
                size: char_info.size,
                uv: char_info.uv,
                advance: Vec2i16::new(advance_x, advance_y),
                color: self.color
            };

            self.buffer.push(text_vertices);
            advance_x += char_info.advance.x;
            max_advance_x = max_advance_x.max(advance_x);
        }

        self.set_size(max_advance_x as f32, advance_y as f32);
    }
}