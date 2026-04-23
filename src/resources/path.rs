use std::sync::OnceLock;

pub static SHADERS_PATH: OnceLock<String> = OnceLock::new();
pub static TEXTURES_PATH: OnceLock<String> = OnceLock::new();