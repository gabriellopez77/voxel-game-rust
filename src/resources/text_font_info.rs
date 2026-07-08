use std::collections::HashMap;
use serde::Deserialize;
use crate::math::{Vec2i16, Vec2u8};
use crate::render::Texture;
use crate::resources::TexCoords;


pub struct FontInfo {
    characters_info: HashMap<char, CharacterInfo>,

    unknown_character_info: CharacterInfo,
}

impl FontInfo {
    pub fn create_from_file(path: &str, font_name: &str, fonts_atlas: &Texture) -> Self {
        let file_content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => panic!("Error reading file: {e}")
        };

        let json_info: JsonFontInfo = serde_json::from_str(&file_content).unwrap();


        let atlas_size = fonts_atlas.get_size();
        let font_tex_coords = fonts_atlas.get_coords(font_name).denormalized(atlas_size);


        let unknown_char_coords = fonts_atlas.get_coords("error_404");
        let unknown_char_size = unknown_char_coords.denormalized(atlas_size).get_size();

        let unknown_char_info = CharacterInfo {
            uv: unknown_char_coords,
            advance: Vec2i16::from1(8),
            size: Vec2u8::new(unknown_char_size.x as u8, unknown_char_size.y as u8)
        };


        let mut chars_info: HashMap<char, CharacterInfo> = HashMap::with_capacity(json_info.characters.len());

        for (ch, info) in &json_info.characters {
            // no supports no ascii characters
            if !ch.is_ascii() { continue }

            let advance = match info.advance {
                Some(advance) => Vec2i16::new(advance[0], advance[1]),
                None => Vec2i16::new(json_info.default_info.advance[0], json_info.default_info.advance[1]),
            };

            let uv = font_tex_coords.get_sub_tex(
                info.uv[0] as f32,
                info.uv[1] as f32,
                info.uv[2] as f32,
                info.uv[3] as f32
            );
            let ch_size = uv.get_size();


            let ch_info = CharacterInfo {
                uv: uv.normalized(atlas_size),
                advance,
                size: Vec2u8::new(ch_size.x as u8, ch_size.y as u8)
            };

            if ch.is_ascii_digit() {
                chars_info.insert(*ch, ch_info);
            }
            else {
                let case_sensitive = match info.case_sensitive {
                    Some(case_sensitive) => case_sensitive,
                    None => json_info.default_info.case_sensitive
                };

                if case_sensitive {
                    chars_info.insert(ch.to_ascii_lowercase(), ch_info);
                    chars_info.insert(ch.to_ascii_uppercase(), ch_info);
                }
                else {
                    chars_info.insert(*ch, ch_info);
                }
            }
        }


        return FontInfo{ characters_info: chars_info, unknown_character_info: unknown_char_info };
    }

    pub fn get_info(&self, ch: char) -> &CharacterInfo {
        if let Some(info) = self.characters_info.get(&ch) { return info }

        return &self.unknown_character_info;
    }
}

#[derive(Copy, Clone)]
pub struct CharacterInfo {
    pub uv: TexCoords,
    pub size: Vec2u8,
    pub advance: Vec2i16,
}


#[derive(Deserialize, Copy, Clone)]
struct DefaultCharacterInfo {
    advance: [i16; 2],

    #[serde(rename = "caseSensitive")]
    case_sensitive: bool,
}

#[derive(Deserialize, Copy, Clone)]
struct JsonCharacterInfo {
    uv: [i32; 4],
    advance: Option<[i16; 2]>,

    #[serde(rename = "caseSensitive")]
    case_sensitive: Option<bool>,
}

#[derive(Deserialize)]
struct JsonFontInfo {
    #[serde(rename = "defaultInfo")]
    default_info: DefaultCharacterInfo,

    characters: HashMap<char, JsonCharacterInfo>,
}
