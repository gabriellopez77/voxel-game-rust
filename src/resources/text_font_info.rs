use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::rc::Rc;
use serde::Deserialize;
use crate::math::{Vec2, Vec2i16};
use crate::render::Texture;
use crate::resources::{CharacterInfo, TextureCoords};


pub struct FontInfo {
    characters_info: HashMap<char, CharacterInfo>,
    unknown_character_info: CharacterInfo,
}

impl FontInfo {
    pub fn create_from_file(path: &str, fonts_atlas: Rc<Texture>) -> Self {
        let file_content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => { panic!("Error reading file: {}", e)}
        };

        let json_info: JsonFontInfo = serde_json::from_str(&file_content).unwrap();

        let atlas_size = fonts_atlas.get_size();

        let unknown_char_coords = fonts_atlas.get_coords("error_404");
        let unknown_char_size = unknown_char_coords.get_size(atlas_size);

        let unknown_char_info = CharacterInfo::new(
            Vec2i16::from1(8),
            unknown_char_coords,
            unknown_char_size.x as i16, unknown_char_size.y as i16
        );



        let mut chars_info: HashMap<char, CharacterInfo> = HashMap::with_capacity(json_info.characters.len());

        for (ch, info) in &json_info.characters {
            // no supports no ascii characters
            if !ch.is_ascii() { continue }

            let advance = match info.advance {
                Some(advance) => Vec2i16::new(advance[0] as i16, advance[1] as i16),
                None => Vec2i16::from1(8),
            };

            let uv = TextureCoords::newi(
                info.uv[0],
                info.uv[1],
                info.uv[0] + info.uv[2],
                info.uv[1] + info.uv[3]
            );
            let ch_size = uv.get_size(atlas_size);


            let ch_info = CharacterInfo::new(
                advance,
                uv.normalized(atlas_size),
                ch_size.x as i16, ch_size.y as i16
            );

            if ch.is_ascii_digit() {
                chars_info.insert(*ch, ch_info);
            }
            else {
                if info.case_sensitive.unwrap_or_default() {
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
}

#[derive(Deserialize, Copy, Clone)]
struct DefaultCharacterInfo {
    advance: [i32; 2],

    #[serde(rename = "caseSensitive")]
    case_sensitive: bool,
}

#[derive(Deserialize, Copy, Clone)]
struct JsonCharacterInfo {
    character: char,
    uv: [i32; 4],
    advance: Option<[i32; 2]>,

    #[serde(rename = "caseSensitive")]
    case_sensitive: Option<bool>,
}

#[derive(Deserialize)]
struct JsonFontInfo {
    #[serde(rename = "defaultInfo")]
    default_info: DefaultCharacterInfo,

    characters: HashMap<char, JsonCharacterInfo>,
}