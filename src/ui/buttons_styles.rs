use std::collections::HashMap;

use serde::Deserialize;

use crate::{render::Texture, resources::TexCoords};


pub struct ButtonsStyles {
    styles: HashMap<String, ButtonStyleInfo>,
}

impl ButtonsStyles {
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
        }
    }

    pub fn load_styles(&mut self, path: &str, ui_texture: &Texture) {
        let file_content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => panic!("Error reading file: {e}"),
        };

        let json_info: HashMap<String, JsonStyleInfo> = serde_json::from_str(&file_content).unwrap();

        for (name, info) in json_info {
            let tex_coords = ui_texture.get_coords(&name).denormalized(ui_texture.get_size());

            let style_info = ButtonStyleInfo {
                hover_tex: tex_coords.get_sub_tex(
                    info.hover_tex[0],
                    info.hover_tex[1],
                    info.hover_tex[2],
                    info.hover_tex[3],
                ).normalized(ui_texture.get_size()),
                hover_corner: info.hover_corner as u8,
                hover_corner_norm: info.hover_corner / ui_texture.get_size().x,

                default_tex: tex_coords.get_sub_tex(
                    info.default_tex[0],
                    info.default_tex[1],
                    info.default_tex[2],
                    info.default_tex[3],
                ).normalized(ui_texture.get_size()),
                default_corner: info.default_corner as u8,
                default_corner_norm: info.default_corner / ui_texture.get_size().x,

                pressed_tex: tex_coords.get_sub_tex(
                    info.pressed_tex[0],
                    info.pressed_tex[1],
                    info.pressed_tex[2],
                    info.pressed_tex[3],
                ).normalized(ui_texture.get_size()),
                pressed_corner: info.pressed_corner as u8,
                pressed_corner_norm: info.pressed_corner / ui_texture.get_size().x,
            };

            self.styles.insert(name, style_info);
        }
    }

    pub fn get(&self, name: &str) -> ButtonStyleInfo {
        return *self.styles.get(name).expect("Button style not exits!");
    }
}

#[derive(Clone, Copy)]
pub struct ButtonStyleInfo {
    pub hover_tex: TexCoords,
    pub hover_corner: u8,
    pub hover_corner_norm: f32,

    pub default_tex: TexCoords,
    pub default_corner: u8,
    pub default_corner_norm: f32,

    pub pressed_tex: TexCoords,
    pub pressed_corner: u8,
    pub pressed_corner_norm: f32,
}

#[derive(Deserialize)]
struct JsonStyleInfo {
    #[serde(rename = "hoverTex")]
    hover_tex: [f32; 4],
    #[serde(rename = "hoverCorner")]
    hover_corner: f32,

    #[serde(rename = "defaultTex")]
    default_tex: [f32; 4],
    #[serde(rename = "defaultCorner")]
    default_corner: f32,

    #[serde(rename = "pressedTex")]
    pressed_tex: [f32; 4],
    #[serde(rename = "pressedCorner")]
    pressed_corner: f32,
}
