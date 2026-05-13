use std::{
    path::PathBuf,
    collections::HashMap,
    rc::Rc,
    cell::RefCell
};
use crate::render::{Texture, Shader};


pub struct ResourceManager {
    shader_path: String,
    textures_path: String,
    //assets_path: String,

    textures: HashMap<&'static str, Rc<Texture>>,
    //fonts: HashMap<&'static str, Rc<Texture>>,
    shaders: HashMap<&'static str, Rc<RefCell<Shader>>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        let assets_path = std::env::current_dir().unwrap().display().to_string();

        Self {
            shader_path: format!(r"{assets_path}\assets\shaders"),
            textures_path: format!(r"{assets_path}\assets\textures"),
            //assets_path,

            textures: HashMap::new(),
            shaders: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        let mut path = self.textures_path.clone();

        // read atlas and textures
        {
            path.push_str(r"\blocks");
            let images = get_files_in_directory_with_filter(&path, "png");
            self.textures.insert("blocks", Rc::new(Texture::create_from_atlas(&images, 256, 256, gl::NEAREST)));
        }

        {
            path.clear();
            path.push_str(&self.textures_path);
            path.push_str(r"\ui");
            let images = get_files_in_directory_with_filter(&path, "png");

            self.textures.insert("ui", Rc::new(Texture::create_from_atlas(&images, 256, 256, gl::NEAREST)));
        }

        {
            path.clear();
            path.push_str(&self.textures_path);
            path.push_str(r"\fonts");
            let images = get_files_in_directory_with_filter(&path, "png");
            self.textures.insert("fonts", Rc::new(Texture::create_from_atlas(&images, 256, 256, gl::NEAREST)));
        }

        // read shaders
        self.read_shader("ui/sprites");
        self.read_shader("chunk");
        self.read_shader("ui/text");
    }

    pub fn get_shader(&self, name: &str) -> Option<Rc<RefCell<Shader>>> {
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

    fn read_shader(&mut self, name: &'static str) {
        let vert_path = format!(r"\{name}.vsh");
        let frag_path = format!(r"\{name}.fsh");

        let shader = Shader::create_from_disk(&self.shader_path, &vert_path, &frag_path);
        self.shaders.insert(name, Rc::new(RefCell::new(shader)));
    }
}

pub fn get_files_in_directory(path: &str) -> Vec<PathBuf> {
    let dir = std::fs::read_dir(path).expect("Error to read Dir");
    let mut files_path: Vec<PathBuf> = vec!();


    for file in dir {
        files_path.push(file.unwrap().path());
    }

    return files_path;
}

pub fn get_files_in_directory_with_filter(path: &str, filter: &str) -> Vec<PathBuf> {
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