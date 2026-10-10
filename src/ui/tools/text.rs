use std::rc::Rc;

use crate::{
    math::{Color3b, Color4b, Vec2, Vec2i16},
    render::{
        UiRenderer,
        vertices_data::TextVertices,
    },
    resources::FontInfo,
    ui::tools::{Sprite, UiElement},
};


enum TextTypes {
    String(String),
    Str(&'static str),
    None,
}

impl TextTypes {
    pub fn get_str(&self) -> Option<&str> {
        match self {
            TextTypes::String(value) => Some(value.as_str()),
            TextTypes::Str(value) => Some(value),
            TextTypes::None => None
        }
    }

    pub fn get_string_mut(&mut self) -> Option<&mut String> {
        match self {
            TextTypes::String(value) => Some(value),
            _ => None
        }
    }
}

pub enum TextInfos {
    Font(Rc<FontInfo>),
    Color(Color3b),
    Shadow(bool),
    Background(Color3b),
}

pub struct Text {
    position: Vec2,
    size: Vec2,
    color: Color3b,

    background: Option<Sprite>,

    text: TextTypes,

    delay: f32,
    pos_modified: bool,
    color_modified: bool,
    shadow: bool,

    buffer: Vec<TextVertices>,
    font_info: Option<Rc<FontInfo>>,
}

impl UiElement for Text {
    fn get_pos(&self) -> Vec2 { self.position }
    fn set_pos(&mut self, x: f32, y: f32) {
        if self.position == Vec2::new(x, y) { return }

        self.position = Vec2::new(x, y);
        self.pos_modified = true;

        if let Some(background) = &mut self.background {
            background.set_posv(self.position - 2.0);
        }
    }

    fn get_size(&self) -> Vec2 { self.size }
    fn set_size(&mut self, _: f32, _: f32) {}
}

impl Text {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            color: Color3b::WHITE,

            background: None,

            text: TextTypes::None,

            delay: 0.0,
            pos_modified: false,
            color_modified: false,
            shadow: false,

            buffer: Vec::new(),
            font_info: None,
        }
    }

    pub fn set_info(&mut self, info: TextInfos) {
        match info {
            TextInfos::Font(font) => self.font_info = Some(font),
            TextInfos::Color(color) => {
                self.color = color;
                self.color_modified = true;
            },
            TextInfos::Shadow(value) => self.shadow = value,
            TextInfos::Background(color) => {
                let mut background = Sprite::new();
                background.color = Color4b::from3(color, 128);
                background.set_posv(self.position);
                background.set_sizev(self.size);

                self.background = Some(background);
            }
        }
    }

    pub fn set_text(&mut self, text: &'static str) {
        self.text = TextTypes::Str(text);
        self.update_mesh();
    }

    pub fn clear_text(&mut self) {
        self.text = TextTypes::None;
    }

    pub fn set_text_string(&mut self, func: impl Fn(&mut String) -> Result<(), std::fmt::Error>) {
        if let None = self.text.get_string_mut() {
            self.text = TextTypes::String(String::new());
        }

        let text = self.text.get_string_mut().unwrap();
        text.clear();

        func(text).unwrap();
        self.update_mesh();
    }

    pub fn set_text_delayed(&mut self, dt: f32, delay: f32, func: impl Fn(&mut String) -> Result<(), std::fmt::Error>) {
        self.delay -= dt;

        if self.delay < 0.0 {
            self.delay = delay;

            self.set_text_string(func);
        }
    }

    pub fn draw(&mut self, renderer: &mut UiRenderer) {
        if let Some(str) = self.text.get_str() {
            if str.is_empty() { return }
        } else { return }

        if self.pos_modified {
            self.pos_modified = false;

            let pos = Vec2i16::new(self.position.x as i16, self.position.y as i16);

            for i in 0..self.buffer.len() {
                self.buffer[i].position = pos;

                if self.shadow && i % 2 == 0 {
                    self.buffer[i].position = pos + 1;
                }
            }
        }

        if self.color_modified {
            self.color_modified = false;

            for i in 0..self.buffer.len() {
                self.buffer[i].color = self.color;
            }
        }

        if let Some(background) = &mut self.background {
            background.draw(renderer);
        }

        // add text data to render buffer
        for item in &self.buffer {
            renderer.add_text(*item);
        }
    }

    fn update_mesh(&mut self) {
        let mut advance_x: i16 = 0;
        let mut advance_y: i16 = 8;
        let mut max_advance_x = 0;

        self.buffer.clear();
        self.color_modified = false;
        self.pos_modified = false;

        let font_info = self.font_info.as_ref().expect("Text font not set!");
        let pos = Vec2i16::new(self.get_pos().x as i16, self.get_pos().y as i16);

        let str = if let Some(str) = self.text.get_str() { str } else { return };

        for ch in str.chars() {
            // breakline
            if ch == '\n' {
                advance_x = 0;
                advance_y += 11;

                continue;
            }

            let char_info = font_info.get_info(ch);

            let mut text_vertices = TextVertices {
                position: pos,
                size: char_info.size,
                uv: char_info.uv,
                advance: Vec2i16::new(advance_x, advance_y - 8),
                color: self.color
            };

            // first, add to buffer the shadow character
            if self.shadow {
                text_vertices.position = pos + 1;
                text_vertices.color = self.color * Color3b::from1(64);
                self.buffer.push(text_vertices);

                text_vertices.position = pos;
                text_vertices.color = self.color;
            }

            self.buffer.push(text_vertices);

            advance_x += char_info.advance.x;
            max_advance_x = max_advance_x.max(advance_x);
        }

        self.size = Vec2::new((max_advance_x - 1) as f32, (advance_y - 1) as f32);


        if let Some(background) = &mut self.background {
            background.set_posv(self.position - 1.0);
            background.set_sizev(self.size + 2.0 + self.shadow as i8 as f32);
        }
    }
}
