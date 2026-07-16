use std::rc::Rc;
use crate::{math::{Color3b, Vec2, Vec2i16}, render::{TextVertices, UiRenderer}, ui::tools::UiElement};
use crate::resources::FontInfo;


enum TextTypes {
    String(String),
    Str(&'static str),
    None,
}

impl TextTypes {
    pub fn get(&self) -> &str {
        match self {
            TextTypes::String(value) => value.as_str(),
            TextTypes::Str(value) => value,
            TextTypes::None => panic!("text not set!")
        }
    }

    pub fn get_string_mut(&mut self) -> &mut String {
        match self {
            TextTypes::String(value) => value,
            _ => panic!("Invalid text type!")
        }
    }
}

pub struct Text {
    position: Vec2,
    size: Vec2,
    color: Color3b,

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
    }

    fn get_size(&self) -> Vec2 { self.size }
    fn set_size(&mut self, x: f32, y: f32) {}
}

impl Text {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            color: Color3b::WHITE,

            text: TextTypes::None,
            delay: 0.0,

            pos_modified: false,
            color_modified: false,
            shadow: false,

            buffer: Vec::new(),
            font_info: None,
        }
    }

    pub fn set_font(&mut self, font: Rc<FontInfo>) {
        self.font_info = Some(font);
    }

    pub fn enable_shadow(&mut self) {
        self.shadow = true;
    }

    pub fn set_color(&mut self, color: Color3b) {
        self.color = color;
        self.color_modified = true;
    }

    pub fn set_text(&mut self, text: &'static str) {
        self.text = TextTypes::Str(text);
        self.update_mesh();
    }

    pub fn set_text_string(&mut self, func: impl Fn(&mut String) -> Result<(), std::fmt::Error>) {
        if !matches!(self.text, TextTypes::String(_)) {
            self.text = TextTypes::String(String::new());
        }

        let text = self.text.get_string_mut();
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

    pub fn set_text_i32(&mut self, value: i32) {
        use std::fmt::Write;

        match self.text {
            TextTypes::String(ref mut string) => {
                string.clear();
                write!(string, "{value}").unwrap();
            }
            _ => self.text = TextTypes::String(value.to_string()),
        }

        self.update_mesh();
    }

    pub fn draw(&mut self, renderer: &mut UiRenderer) {
        match self.text {
            TextTypes::String(ref string) => if string.is_empty() { return }
            TextTypes::Str(value) => if value.is_empty() { return }
            _ => return
        }

        if self.pos_modified {
            self.pos_modified = false;

            let pos = Vec2i16::new(self.position.x as i16, self.position.y as i16);

            for i in 0..self.buffer.len() {
                self.buffer[i].position = pos;
            }
        }

        if self.color_modified {
            self.color_modified = false;

            for i in 0..self.buffer.len() {
                self.buffer[i].color = self.color;
            }
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

        for ch in self.text.get().chars() {
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
    }
}
