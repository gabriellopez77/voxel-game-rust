use std::{path::PathBuf, collections::HashMap, rc::Rc};

use crate::render::{Texture, Shader};
use crate::resources::texture_atlas;

pub struct ResourceManager {
    pub shader_path: String,
    pub textures_path: String,

    textures: HashMap<&'static str, Rc<Texture>>,
    shaders: HashMap<&'static str, Rc<Shader>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        let assets_path = std::env::current_dir().unwrap().display().to_string();

        Self {
            shader_path: format!("{}{}", assets_path, r"\assets\shaders"),
            textures_path: format!("{}{}", assets_path, r"\assets\textures"),
            textures: HashMap::new(),
            shaders: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        {
            let atlas_files = get_files_in_directory_with_filter(&(self.textures_path), "png");
            let atlas = texture_atlas::create(&atlas_files, 256, 256);

            self.textures.insert("ui", Rc::new(Texture::create_from_atlas(atlas, 256, 256, gl::NEAREST)));
        }

        self.shaders.insert("ui", Rc::new(Shader::create_from_disk(&self.shader_path,r"\vert.glsl", r"\frag.glsl")));
    }

    pub fn get_shader(&self, name: &str) -> Option<Rc<Shader>> {
        if let Some(shader) = self.shaders.get(name) {
            return Some(shader.clone());
        }
        
        return None;
    }
    pub fn get_texture(&self, name: &str) -> Option<Rc<Texture>> {
        if let Some(texture) = self.textures.get(name) {
            return Some(texture.clone());
        }
        
        return None;
    }
}

pub fn get_files_in_directory(path: &String) -> Vec<PathBuf> {
    let dir = std::fs::read_dir(path).expect("Error to read Dir");
    let mut files_path: Vec<PathBuf> = vec!();


    for file in dir {
        files_path.push(file.unwrap().path());
    }

    return files_path;
}

pub fn get_files_in_directory_with_filter(path: &String, filter: &'static str) -> Vec<PathBuf> {
    let dir = std::fs::read_dir(path).expect("Error to read Dir");
    let mut files_path: Vec<PathBuf> = vec!();

    for file in dir {
        let file_path = file.unwrap().path();
        let file_extension = file_path.extension();

        if file_extension.is_some()  && file_extension.unwrap() == filter {
            files_path.push(file_path);
        }
    }

    return files_path;
}