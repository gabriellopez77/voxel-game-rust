pub mod vec2;
pub mod vec2i;
pub mod vec2u8;
pub mod vec2i16;

pub mod vec3;
pub mod vec3i;

pub mod vec4;
pub mod vec4i16;

pub mod color3b;
pub mod color4b;

pub mod matrix4;

pub mod math;

pub mod key_frame;


pub use {
    math::*,

    vec2::Vec2,
    vec2i::Vec2i,
    vec2u8::Vec2u8,
    vec2i16::Vec2i16,

    vec3::Vec3,
    vec3i::Vec3i,

    vec4::Vec4,
    vec4i16::Vec4i16,

    color4b::Color4b,
    color3b::Color3b,

    matrix4::Matrix4,

    key_frame::KeyFrame,
};
