use crate::math::{Color4b, Vec2, Vec2i16, Vec4};
use crate::render::{SpritesRenderer, SpritesVertices, sprites_renderer};
use crate::resources::TexCoords;
use crate::ui::tools::ui_element::UiElement;


pub struct Sprite {
    position: Vec2,
    size: Vec2,
    uv: TexCoords,

    pub color: Color4b,
}

impl UiElement for Sprite {
    fn get_pos(&self) -> Vec2 { self.position }
    fn set_pos(&mut self, x: f32, y: f32) { self.position = Vec2{ x, y } }

    fn get_size(&self) -> Vec2 { self.size }
    fn set_size(&mut self, x: f32, y: f32) { self.size = Vec2{ x, y } }
}

impl Sprite {
    pub const fn new() -> Self {
        Self {
            color: Color4b {r: 255, g: 255, b: 255, a: 255},
            position: Vec2::ZERO,
            size: Vec2::ZERO,
            uv: TexCoords::ZERO,
        }
    }

    pub fn set_texture(&mut self, uv: TexCoords) { self.uv = uv }

    pub fn draw(&self, renderer: &mut SpritesRenderer<SpritesVertices>) {
        if renderer.buffer_len() >= sprites_renderer::MAX_SPRITES { return }

        let pos = self.get_pos();
        let size = self.get_size();

        renderer.add_element(SpritesVertices{
            position: Vec2i16::new(pos.x as i16, pos.y as i16),
            size: Vec2i16::new(size.x as i16, size.y as i16),
            uv: self.uv,
            color: self.color,
        })
    }
}