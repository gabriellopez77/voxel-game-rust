use crate::math::Vec2i16;
use crate::resources::TextureCoords;

#[derive(Copy, Clone)]
pub struct CharacterInfo {
    advance: Vec2i16,

    uv: TextureCoords,

    size: Vec2i16,
}

impl CharacterInfo {
    pub fn new(advance: Vec2i16, uv: TextureCoords, width: i16, height: i16) -> Self {
        Self {
            advance,
            uv,
            size: Vec2i16::new(width, height)
        }
    }

    pub fn get_size(&self) -> Vec2i16 { self.size }
}