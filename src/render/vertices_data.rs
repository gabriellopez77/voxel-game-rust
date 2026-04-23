use crate::math::*;


pub const SPRITES_VERTICES: [f32; 16] = [
    1.0, 1.0,  1.0, 0.0, // top right
    1.0, 0.0,  1.0, 1.0, // bottom right
    0.0, 0.0,  0.0, 1.0, // bottom left
    0.0, 1.0,  0.0, 0.0, // top left
];

pub const SPRITES_INDICES: [u32; 6] = [ 0, 1, 2, 2, 3, 0 ];

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpritesVertices {
    pub position: Vec2i16,
    pub size: Vec2i16,
    pub uv: Vec4,
    pub color: Color4b,
}

impl Default for SpritesVertices {
    fn default() -> Self {
        Self {
            position: Vec2i16::ZERO,
            size: Vec2i16::ZERO,
            uv: Vec4::ZERO,
            color: Color4b::ZERO,
        }
    }
}