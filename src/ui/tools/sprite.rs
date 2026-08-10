use crate::math::{Color4b, Vec2, Vec2i16};
use crate::render::{SpritesVertices, Texture, UiRenderer};
use crate::resources::TexCoords;
use crate::ui::tools::ui_element::UiElement;


pub struct Sprite {
    position: Vec2,
    size: Vec2,
    uv: TexCoords,
    texture_idx: u8,

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
            texture_idx: u8::MAX,
        }
    }

    pub fn set_texture_from_coords(&mut self, texture_idx: u8, uv: TexCoords) {
        self.texture_idx = texture_idx;
        self.uv = uv;
    }

    pub fn set_texture(&mut self, tex: &Texture, name: &str) {
        self.texture_idx = tex.raw_texture.inxeding_idx as u8;
        self.uv = tex.get_coords(name);
    }

    pub fn draw(&self, renderer: &mut UiRenderer) {
        let pos = self.get_pos();
        let size = self.get_size();

        renderer.add_sprite(SpritesVertices{
            position: Vec2i16::new(pos.x as i16, pos.y as i16),
            size: Vec2i16::new(size.x as i16, size.y as i16),
            uv: self.uv,
            color: self.color,
            texture_idx: self.texture_idx,
        })
    }
}
