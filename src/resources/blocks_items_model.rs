use std::collections::HashMap;
use serde::Deserialize;
use crate::{math, render::BlockModelMesh, resources::TexCoords};
use crate::math::{Matrix4, Vec2, Vec3, Vec4};
use crate::render::Texture;


const SCALE: f32 = 1.0 / 16.0;
const TEXTURE_NORMALIZE_FACTOR: f32 = 16.0;

const ERROR_MODEL: &'static str = "
{
	\"isCompleteBlock\": true,
    \"textures\": {
    \"0\": \"blocks/error_404\"
    },
	\"elements\": [
		{
			\"from\": [0, 0, 0],
            \"to\": [16, 16, 16],
			\"faces\": {
				\"north\": {\"uv\": [0, 0, 16, 16], \"texture\": \"#0\", \"cullface\": \"north\"},
                \"east\": {\"uv\": [0, 0, 16, 16], \"texture\": \"#0\", \"cullface\": \"east\"},
                \"south\": {\"uv\": [0, 0, 16, 16], \"texture\": \"#0\", \"cullface\": \"south\"},
                \"west\": {\"uv\": [0, 0, 16, 16], \"texture\": \"#0\", \"cullface\": \"west\"},
                \"up\": {\"uv\": [0, 0, 16, 16], \"texture\": \"#0\", \"cullface\": \"up\"},
                \"down\": {\"uv\": [0, 0, 16, 16], \"texture\": \"#0\", \"cullface\": \"down\"}
			}
		}
	]
}
";

pub struct BlockItemModel {
    pub nothing_vertices: Vec<BlockModelMesh>,
    pub up_vertices: Vec<BlockModelMesh>,
    pub down_vertices: Vec<BlockModelMesh>,
    pub south_vertices: Vec<BlockModelMesh>,
    pub north_vertices: Vec<BlockModelMesh>,
    pub west_vertices: Vec<BlockModelMesh>,
    pub east_vertices: Vec<BlockModelMesh>,

    pub icon_coords: TexCoords,
    pub particle_coords: TexCoords,

    pub ambient_occlusion: bool,
}

impl BlockItemModel {
    pub fn new(models_path: &str, path: &str, texture: &Texture) -> Result<Self, String> {
        let file_content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(err) => {
                println!("Error reading model file: {err}");
                return Err(err.to_string());
            }
        };

        let mut instance = Self {
            nothing_vertices: Vec::new(),
            up_vertices: Vec::new(),
            down_vertices: Vec::new(),
            south_vertices: Vec::new(),
            north_vertices: Vec::new(),
            west_vertices: Vec::new(),
            east_vertices: Vec::new(),

            icon_coords: TexCoords::ZERO,
            particle_coords: TexCoords::ZERO,
            ambient_occlusion: false,
        };

        match instance.read(models_path, &file_content, texture) {
            Ok(()) => return Ok(instance),
            Err(err) => Err(err)
        }
    }

    pub fn read_error_model(texture: &Texture) -> Self {
        let mut instance = Self {
            nothing_vertices: Vec::new(),
            up_vertices: Vec::new(),
            down_vertices: Vec::new(),
            south_vertices: Vec::new(),
            north_vertices: Vec::new(),
            west_vertices: Vec::new(),
            east_vertices: Vec::new(),

            icon_coords: TexCoords::ZERO,
            particle_coords: TexCoords::ZERO,
            ambient_occlusion: false,
        };

        match instance.read("", &ERROR_MODEL, texture) {
            Ok(()) => return instance,
            Err(err) => panic!("error to load error model: {}", err.to_string())
        }
    }

    fn read(&mut self, models_path: &str, content: &str, texture: &Texture) -> Result<(), String> {
        let model_info: ModelInfo = match serde_json::from_str(content) {
            Ok(info) => info,
            Err(err) => {
                println!("Error parsing model file: {err}");
                return Err(err.to_string());
            }
        };

        let mut parent_info: Option<ModelInfo> = None;

        let mut used_textures: HashMap<String, TexCoords> = HashMap::new();

        // read texture if model have it
        if let Some(ref textures_info) = model_info.textures {
            used_textures = HashMap::with_capacity(textures_info.len());
            self.read_textures(&mut used_textures, &textures_info, texture);
        }

        // read parent model
        if let Some(parent_name) = &model_info.parent {
            let full_parent_path = format!(r"{models_path}\{parent_name}.json");

            let parent_file_content = match std::fs::read_to_string(&full_parent_path) {
                Ok(content) => content,
                Err(err) => {
                    println!("Error reading model file: {err}");
                    return Err(err.to_string());
                }
            };

            parent_info = match serde_json::from_str::<ModelInfo>(&parent_file_content) {
                Ok(info) => Some(info),
                Err(err) => {
                    println!("Error parsing model file: {err} path: {full_parent_path}");
                    return Err(err.to_string());
                }
            };

            if let Some(ref elements) = parent_info.as_ref().unwrap().elements {
                self.create_mesh(&elements, &used_textures, texture.get_size());
            }
        }
        else {
            if let Some(ref elements) = model_info.elements {
                self.create_mesh(&elements, &used_textures, texture.get_size());
            }
        }

        let mut ambient_occlusion = true;

        if let Some(ref parent) = parent_info && let Some(value) = parent.ambient_occlusion {
            ambient_occlusion = value;
        }

        if let Some(value) = model_info.ambient_occlusion {
            ambient_occlusion = value;
        }

        self.ambient_occlusion = ambient_occlusion;

        return Ok(());

    }

    fn create_mesh(&mut self, elements_info: &Vec<ElementInfo>,
                   used_textures: &HashMap<String, TexCoords>, texture_size: Vec2) {
        for element in elements_info {
            let from = Vec3::new(element.from[0], element.from[1], element.from[2]) * SCALE;
            let to = Vec3::new(element.to[0], element.to[1], element.to[2]) * SCALE;

            let shade = element.shade.unwrap_or_else(|| true);

            self.create_cube(texture_size,
                used_textures,
                &element.faces,
                &element.rotation,
                from, to, shade
            );
        }
    }

    fn create_cube(&mut self, texture_size: Vec2, used_textures: &HashMap<String, TexCoords>,
                    faces_info: &HashMap<String, FaceInfo>, rotate_info: &Option<RotateInfo>,
                    from: Vec3, to: Vec3, shade: bool) {
        let size = to - from;

        let mut rotate_matrix = Matrix4::IDENTITY;
        let mut origin = Vec3::ZERO;
        let mut angle = 0.0;

        if let Some(info) = rotate_info && info.angle != 0.0 {
            angle = info.angle;

            origin = Vec3::new(info.origin[0],info.origin[1],info.origin[2]) * SCALE;

            match info.axis {
                'x' => rotate_matrix.rotate(angle, 1.0, 0.0, 0.0),
                'y' => rotate_matrix.rotate(angle, 0.0, 1.0, 0.0),
                _ => rotate_matrix.rotate(angle, 0.0, 0.0, 1.0),
            }
        }

        if let Some(face) = faces_info.get("up") && size.x != 0.0 && size.z != 0.0 {
            let vertices = self.get_vertices(&face.cullface);

            let mut vert1 = Vec3::new(0.0, 1.0, 1.0) * size + from;
            let mut vert2 = Vec3::new(1.0, 1.0, 1.0) * size + from;
            let mut vert3 = Vec3::new(1.0, 1.0, 0.0) * size + from;
            let mut vert4 = Vec3::new(0.0, 1.0, 0.0) * size + from;

            let normal1 = Vec3::new(0.0, 1.0, 0.0);
            let normal2 = Vec3::new(0.0, 1.0, 0.0);
            let normal3 = Vec3::new(0.0, 1.0, 0.0);
            let normal4 = Vec3::new(0.0, 1.0, 0.0);

            let (tex1, tex2, tex3, tex4) = Self::get_tex_coords(used_textures, &face, texture_size);

            if angle != 0.0 {
                Self::rotate_face(&mut vert1, &mut vert2, &mut vert3, &mut vert4, origin, &rotate_matrix)
            }

            vertices.push(BlockModelMesh { vertices: vert1, uv: tex1, normal: normal1, shade });
            vertices.push(BlockModelMesh { vertices: vert2, uv: tex2, normal: normal2, shade });
            vertices.push(BlockModelMesh { vertices: vert3, uv: tex3, normal: normal3, shade });
            vertices.push(BlockModelMesh { vertices: vert4, uv: tex4, normal: normal4, shade });
        }

        if let Some(face) = faces_info.get("down") && size.x != 0.0 && size.z != 0.0 {
            let vertices = self.get_vertices(&face.cullface);

            let mut vert1 = Vec3::new(1.0, 0.0, 1.0) * size + from;
            let mut vert2 = Vec3::new(0.0, 0.0, 1.0) * size + from;
            let mut vert3 = Vec3::new(0.0, 0.0, 0.0) * size + from;
            let mut vert4 = Vec3::new(1.0, 0.0, 0.0) * size + from;

            let normal1 = Vec3::new(0.0, -1.0, 0.0);
            let normal2 = Vec3::new(0.0, -1.0, 0.0);
            let normal3 = Vec3::new(0.0, -1.0, 0.0);
            let normal4 = Vec3::new(0.0, -1.0, 0.0);

            let (tex1, tex2, tex3, tex4) = Self::get_tex_coords(used_textures, &face, texture_size);

            if angle != 0.0 {
                Self::rotate_face(&mut vert1, &mut vert2, &mut vert3, &mut vert4, origin, &rotate_matrix)
            }

            vertices.push(BlockModelMesh { vertices: vert1, uv: tex1, normal: normal1, shade });
            vertices.push(BlockModelMesh { vertices: vert2, uv: tex2, normal: normal2, shade });
            vertices.push(BlockModelMesh { vertices: vert3, uv: tex3, normal: normal3, shade });
            vertices.push(BlockModelMesh { vertices: vert4, uv: tex4, normal: normal4, shade });
        }

        if let Some(face) = faces_info.get("north") && size.y != 0.0 && size.x != 0.0 {
            let vertices = self.get_vertices(&face.cullface);

            let mut vert1 = Vec3::new(1.0, 1.0, 0.0) * size + from;
            let mut vert2 = Vec3::new(1.0, 0.0, 0.0) * size + from;
            let mut vert3 = Vec3::new(0.0, 0.0, 0.0) * size + from;
            let mut vert4 = Vec3::new(0.0, 1.0, 0.0) * size + from;

            let normal1 = Vec3::new(0.0, 0.0, -1.0);
            let normal2 = Vec3::new(0.0, 0.0, -1.0);
            let normal3 = Vec3::new(0.0, 0.0, -1.0);
            let normal4 = Vec3::new(0.0, 0.0, -1.0);

            let (tex1, tex2, tex3, tex4) = Self::get_tex_coords(used_textures, &face, texture_size);

            if angle != 0.0 {
                Self::rotate_face(&mut vert1, &mut vert2, &mut vert3, &mut vert4, origin, &rotate_matrix)
            }

            vertices.push(BlockModelMesh { vertices: vert1, uv: tex1, normal: normal1, shade });
            vertices.push(BlockModelMesh { vertices: vert2, uv: tex2, normal: normal2, shade });
            vertices.push(BlockModelMesh { vertices: vert3, uv: tex3, normal: normal3, shade });
            vertices.push(BlockModelMesh { vertices: vert4, uv: tex4, normal: normal4, shade });
        }

        if let Some(face) = faces_info.get("south") && size.y != 0.0 && size.x != 0.0 {
            let vertices = self.get_vertices(&face.cullface);

            let mut vert1 = Vec3::new(0.0, 1.0, 1.0) * size + from;
            let mut vert2 = Vec3::new(0.0, 0.0, 1.0) * size + from;
            let mut vert3 = Vec3::new(1.0, 0.0, 1.0) * size + from;
            let mut vert4 = Vec3::new(1.0, 1.0, 1.0) * size + from;

            let normal1 = Vec3::new(0.0, 0.0, 1.0);
            let normal2 = Vec3::new(0.0, 0.0, 1.0);
            let normal3 = Vec3::new(0.0, 0.0, 1.0);
            let normal4 = Vec3::new(0.0, 0.0, 1.0);

            let (tex1, tex2, tex3, tex4) = Self::get_tex_coords(used_textures, &face, texture_size);

            if angle != 0.0 {
                Self::rotate_face(&mut vert1, &mut vert2, &mut vert3, &mut vert4, origin, &rotate_matrix)
            }

            vertices.push(BlockModelMesh { vertices: vert1, uv: tex1, normal: normal1, shade });
            vertices.push(BlockModelMesh { vertices: vert2, uv: tex2, normal: normal2, shade });
            vertices.push(BlockModelMesh { vertices: vert3, uv: tex3, normal: normal3, shade });
            vertices.push(BlockModelMesh { vertices: vert4, uv: tex4, normal: normal4, shade });
        }

        if let Some(face) = faces_info.get("west") && size.y != 0.0 && size.z != 0.0 {
            let vertices = self.get_vertices(&face.cullface);

            let mut vert1 = Vec3::new(0.0, 1.0, 0.0) * size + from;
            let mut vert2 = Vec3::new(0.0, 0.0, 0.0) * size + from;
            let mut vert3 = Vec3::new(0.0, 0.0, 1.0) * size + from;
            let mut vert4 = Vec3::new(0.0, 1.0, 1.0) * size + from;

            let normal1 = Vec3::new(-1.0, 0.0, 0.0);
            let normal2 = Vec3::new(-1.0, 0.0, 0.0);
            let normal3 = Vec3::new(-1.0, 0.0, 0.0);
            let normal4 = Vec3::new(-1.0, 0.0, 0.0);

            let (tex1, tex2, tex3, tex4) = Self::get_tex_coords(used_textures, &face, texture_size);

            if angle != 0.0 {
                Self::rotate_face(&mut vert1, &mut vert2, &mut vert3, &mut vert4, origin, &rotate_matrix)
            }
            vertices.push(BlockModelMesh { vertices: vert1, uv: tex1, normal: normal1, shade });
            vertices.push(BlockModelMesh { vertices: vert2, uv: tex2, normal: normal2, shade });
            vertices.push(BlockModelMesh { vertices: vert3, uv: tex3, normal: normal3, shade });
            vertices.push(BlockModelMesh { vertices: vert4, uv: tex4, normal: normal4, shade });
        }

        if let Some(face) = faces_info.get("east") && size.y != 0.0 && size.z != 0.0 {
            let vertices = self.get_vertices(&face.cullface);

            let mut vert1 = Vec3::new(1.0, 1.0, 1.0) * size + from;
            let mut vert2 = Vec3::new(1.0, 0.0, 1.0) * size + from;
            let mut vert3 = Vec3::new(1.0, 0.0, 0.0) * size + from;
            let mut vert4 = Vec3::new(1.0, 1.0, 0.0) * size + from;

            let normal1 = Vec3::new(1.0, 0.0, 0.0);
            let normal2 = Vec3::new(1.0, 0.0, 0.0);
            let normal3 = Vec3::new(1.0, 0.0, 0.0);
            let normal4 = Vec3::new(1.0, 0.0, 0.0);

            let (tex1, tex2, tex3, tex4) = Self::get_tex_coords(used_textures, &face, texture_size);

            if angle != 0.0 {
                Self::rotate_face(&mut vert1, &mut vert2, &mut vert3, &mut vert4, origin, &rotate_matrix)
            }

            vertices.push(BlockModelMesh { vertices: vert1, uv: tex1, normal: normal1, shade });
            vertices.push(BlockModelMesh { vertices: vert2, uv: tex2, normal: normal2, shade });
            vertices.push(BlockModelMesh { vertices: vert3, uv: tex3, normal: normal3, shade });
            vertices.push(BlockModelMesh { vertices: vert4, uv: tex4, normal: normal4, shade });
        }
    }

    fn get_vertices(&mut self, face: &Option<String>) -> &mut Vec<BlockModelMesh> {
        match face {
            Some(f) => {
                match f.as_str() {
                    "up" => &mut self.up_vertices,
                    "down" => &mut self.down_vertices,
                    "north" => &mut self.north_vertices,
                    "south" => &mut self.south_vertices,
                    "west" => &mut self.west_vertices,
                    "east" => &mut self.east_vertices,
                    _ => &mut self.nothing_vertices,
                }
            }
            None => &mut self.nothing_vertices,
        }

    }

    fn get_tex_coords(used_textures: &HashMap<String, TexCoords>, face_info: &FaceInfo,
                      texture_size: Vec2) -> (Vec2, Vec2, Vec2, Vec2) {
        let tex_coords = match used_textures.get(face_info.texture.as_str()) {
            Some(x) => x,
            None => used_textures.get("#missing").unwrap()
        }.denormalized(texture_size);


        let tex_size = tex_coords.get_size();

        let mut tex_quad_start = Vec2::new(
            math::lerp(0.0, tex_size.x, face_info.uv[0] as f32 / TEXTURE_NORMALIZE_FACTOR),
            math::lerp(0.0, tex_size.y, face_info.uv[1] as f32 / TEXTURE_NORMALIZE_FACTOR)
        );

        let tex_quad_size = Vec2::new(
            math::lerp(0.0, tex_size.x, face_info.uv[2] as f32 / TEXTURE_NORMALIZE_FACTOR) - tex_quad_start.x,
            math::lerp(0.0, tex_size.y, face_info.uv[3] as f32 / TEXTURE_NORMALIZE_FACTOR) - tex_quad_start.y,
        );

        tex_quad_start += Vec2::new(tex_coords.minx, tex_coords.miny);

        return (
            (Vec2::new(0.0, 0.0) * tex_quad_size + tex_quad_start) / texture_size,
            (Vec2::new(0.0, 1.0) * tex_quad_size + tex_quad_start) / texture_size,
            (Vec2::new(1.0, 1.0) * tex_quad_size + tex_quad_start) / texture_size,
            (Vec2::new(1.0, 0.0) * tex_quad_size + tex_quad_start) / texture_size
        );
    }

    fn rotate_face(vert1: &mut Vec3, vert2: &mut Vec3, vert3: &mut Vec3, vert4: &mut Vec3,
                    origin: Vec3, rotate_matrix: &Matrix4) {
        *vert1 -= origin;
        *vert2 -= origin;
        *vert3 -= origin;
        *vert4 -= origin;

        *vert1 = Vec3::from4(Vec4::from3(*vert1, 1.0) * *rotate_matrix);
        *vert2 = Vec3::from4(Vec4::from3(*vert2, 1.0) * *rotate_matrix);
        *vert3 = Vec3::from4(Vec4::from3(*vert3, 1.0) * *rotate_matrix);
        *vert4 = Vec3::from4(Vec4::from3(*vert4, 1.0) * *rotate_matrix);

        *vert1 += origin;
        *vert2 += origin;
        *vert3 += origin;
        *vert4 += origin;
    }

    fn read_textures(&mut self, used_textures: &mut HashMap<String, TexCoords>,
                     textures_info: &HashMap<String, String>, texture: &Texture) {
        fn remove_unnecessary_path(path: &String) -> String {
            if path.starts_with("blocks/") {
                return path.replace("blocks/", "")
            }
            else if path.starts_with("items/") {
                return path.replace("items/", "")
            }

            panic!("invalid model texture path: {path}");
        }

        // add missing (error texture)
        used_textures.insert("#missing".into(), texture.get_coords("error_404"));

        // load error particle texture
        self.particle_coords = texture.get_coords("error_404");

        for (tex_alias, tex_path) in textures_info {
            let coords = texture.get_coords(&remove_unnecessary_path(&tex_path));

            // load particle texture
            if tex_alias == "particle" {
                self.particle_coords = coords;
            }
            else if tex_alias == "$side" {
                used_textures.insert("#north".into(), coords);
                used_textures.insert("#south".into(), coords);
                used_textures.insert("#west".into(), coords);
                used_textures.insert("#east".into(), coords);
            }
            else if tex_alias == "$all" {
                used_textures.insert("#up".into(), coords);
                used_textures.insert("#down".into(), coords);
                used_textures.insert("#north".into(), coords);
                used_textures.insert("#south".into(), coords);
                used_textures.insert("#west".into(), coords);
                used_textures.insert("#east".into(), coords);
            }
            else {
                used_textures.insert(format!("#{tex_alias}"), coords);
            }
        }
    }
}

#[derive(Deserialize)]
struct ModelInfo {
    #[serde(rename = "itemIcon")]
    item_icon: Option<String>,
    parent: Option<String>,
    textures: Option<HashMap<String, String>>,

    #[serde(rename = "ambientOcclusion")]
    ambient_occlusion: Option<bool>,

    elements: Option<Vec<ElementInfo>>
}

#[derive(Deserialize)]
struct ElementInfo {
    from: [f32; 3],
    to: [f32; 3],
    shade: Option<bool>,
    rotation: Option<RotateInfo>,
    faces: HashMap<String, FaceInfo>
}

#[derive(Deserialize)]
struct RotateInfo {
    angle: f32,
    axis: char,
    origin: [f32; 3],
}

#[derive(Deserialize)]
struct FaceInfo {
    uv: [f32; 4],
    texture: String,
    cullface: Option<String>,
}
