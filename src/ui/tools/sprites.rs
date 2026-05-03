use crate::math::{Color4b, Vec2, Vec2i16, Vec4};
use crate::render::sprites_renderer::MAX_SPRITES;
use crate::render::{SpritesRenderer, SpritesVertices};
use crate::ui::tools::ui_element::UiElement;


pub struct Sprite {
    position: Vec2,
    size: Vec2,
    uv: Vec4,

    pub color: Color4b,
}

impl UiElement for Sprite {
    fn get_pos(&self) -> Vec2 { self.position }
    fn get_size(&self) -> Vec2 { self.size }

    fn set_pos(&mut self, x: f32, y: f32) { self.position = Vec2{ x, y } }
    fn set_size(&mut self, x: f32, y: f32) { self.size = Vec2{ x, y } }
}

impl Sprite {
    pub const fn new() -> Self {
        Self {
            color: Color4b::ZERO,
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            uv: Vec4::ZERO,
        }
    }

    pub fn set_texture(&mut self, uv: Vec4) { self.uv = uv }

    pub fn draw(&self, renderer: &mut SpritesRenderer) {
        if renderer.buffer.len() as usize >= MAX_SPRITES { return }

        renderer.buffer.add(SpritesVertices{
            position: Vec2i16{x: self.get_pos().x as i16, y: self.get_pos().y as i16},
            size: Vec2i16{x: self.get_size().x as i16, y: self.get_size().y as i16},
            uv: self.uv,
            color: self.color,
        })
    }
}