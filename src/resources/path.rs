use std::sync::OnceLock;

pub static SHADER_PATH: OnceLock<String> = OnceLock::new();