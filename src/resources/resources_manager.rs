use std::f32;
use std::ops::Add;
use std::{
    path::PathBuf,
    collections::HashMap,
    rc::Rc,
    cell::RefCell
};
use crate::math::{Matrix4, Vec3};
use crate::render::{Shader, Texture, Ubo};
use crate::resources::{BlockItemModel, FontInfo};


pub struct ResourceManager {
    shader_path: String,
    textures_path: String,
    models_path: String,
    assets_path: String,

    textures: HashMap<&'static str, Rc<Texture>>,
    shaders: HashMap<&'static str, Rc<RefCell<Shader>>>,
    ubos: HashMap<&'static str, Rc<Ubo>>,
    fonts: HashMap<&'static str, Rc<FontInfo>>,
    models: HashMap<String, Rc<BlockItemModel>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        let assets_path = std::env::current_dir().unwrap().display().to_string();

        Self {
            shader_path: format!(r"{assets_path}\assets\shaders"),
            textures_path: format!(r"{assets_path}\assets\textures"),
            models_path: format!(r"{assets_path}\assets\models"),
            assets_path,

            textures: HashMap::new(),
            shaders: HashMap::new(),
            ubos: HashMap::new(),
            fonts: HashMap::new(),
            models: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        let mut path = self.textures_path.clone();

        // read atlas and textures
        {
            path.push_str(r"\blocks");
            let images = get_filtered_files_in_directory(&path, "png");
            self.textures.insert("blocks", Rc::new(Texture::create_from_atlas(&images, 256, 256, gl::NEAREST)));
        }

        {
            path.clear();
            path.push_str(&self.textures_path);
            path.push_str(r"\ui");
            let images = get_filtered_files_in_directory(&path, "png");

            self.textures.insert("ui", Rc::new(Texture::create_from_atlas(&images, 256, 256, gl::NEAREST)));
        }

        {
            path.clear();
            path.push_str(&self.textures_path);
            path.push_str(r"\fonts");
            let images = get_filtered_files_in_directory(&path, "png");
            self.textures.insert("fonts", Rc::new(Texture::create_from_atlas(&images, 256, 256, gl::NEAREST)));
        }

        // load fonts
        {
            path.clear();
            path.push_str(&self.textures_path);
            path.push_str(r"\fonts\default_font.json");
            self.fonts.insert("default_font", Rc::new(FontInfo::create_from_file(&path, "default_font", self.textures["fonts"].clone())));
        }

        // load models

        self.models.clear();
        self.read_models();


        {
            let mut ubo = Ubo::new();
            ubo.add::<Matrix4>("uiProj");
            ubo.add::<f32>("uiPixelScale");
            ubo.add::<Matrix4>("camProj");
            ubo.add::<Matrix4>("camView");
            ubo.add::<Matrix4>("camViewProj");
            ubo.add::<Matrix4>("camViewNoTranslate");
            ubo.create(0);
            self.ubos.insert("globalData", Rc::new(ubo));
        }

        {
            let mut ubo = Ubo::new();
           	ubo.add::<Vec3>("skyColor");
           	ubo.add::<Vec3>("fogColor");
           	ubo.add::<Vec3>("lightColor");
           	ubo.add::<Vec3>("darknessColor");
           	ubo.add::<Vec3>("ambientColor");
           	ubo.add::<f32>("fogDistance");
           	ubo.add::<f32>("fogDensity");
           	ubo.add::<i32>("fogEnable");
            ubo.create(1);
            self.ubos.insert("worldData", Rc::new(ubo));
        }



        // read shaders
        self.read_shader("ui/sprites");
        self.read_shader("chunk");
        self.read_shader("ui/text");
        self.read_shader("skyDome");
    }

    pub fn get_ubo(&self, name: &str) -> Option<Rc<Ubo>> {
        if let Some(ubo) = self.ubos.get(name) {
            return Some(ubo.clone());
        }

        return None;
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

    pub fn get_font(&self, name: &str) -> Option<Rc<FontInfo>> {
        if let Some(font) = self.fonts.get(name) {
            return Some(font.clone());
        }

        return None;
    }

    pub fn get_model(&self, name: &str) -> Rc<BlockItemModel> {
        if let Some(model) = self.models.get(name) {
            return model.clone();
        }

        // is guaranteed that contains the 'error_404' texture coords
        return self.models.get("error_404").unwrap().clone();
    }

    fn read_shader(&mut self, name: &'static str) {
        let vert_path = format!(r"\{name}.vsh");
        let frag_path = format!(r"\{name}.fsh");

        let shader = Shader::create_from_disk(&self.shader_path, &vert_path, &frag_path);
        self.shaders.insert(name, Rc::new(RefCell::new(shader)));
    }

    fn read_models(&mut self) {
        let items_blocks_texture = self.get_texture("blocks").unwrap();


        let block_paths = get_filtered_files_in_directory(&format!(r"{}\blocks", self.models_path), "json");

        // load error model
        self.models.insert("error_404".to_string(), Rc::new(BlockItemModel::read_error_model(&items_blocks_texture)));

        for path in &block_paths {
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();

            let model = match BlockItemModel::new(&self.models_path, &path.to_str().unwrap(), &items_blocks_texture) {
                Ok(m) => m,
                Err(err) => {
                    println!("{err}");
                    BlockItemModel::read_error_model(&items_blocks_texture)
                }
            };

            self.models.insert(name, Rc::new(model));
        }

        let items_paths = get_filtered_files_in_directory(&format!(r"{}\items", self.models_path), "json");

        for path in &items_paths {
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();

            let model = match BlockItemModel::new(&self.models_path, &path.to_str().unwrap(), &items_blocks_texture) {
                Ok(m) => m,
                Err(err) => {
                    println!("{err}");
                    BlockItemModel::read_error_model(&items_blocks_texture)
                }
            };

            self.models.insert(name, Rc::new(model));
        }
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

pub fn get_filtered_files_in_directory(path: &str, filter: &str) -> Vec<PathBuf> {
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


pub fn gen_sphere(stacks: f32, slices: f32) -> (Vec<Vec3>, Vec<u32>) {
    let mut vertices: Vec<Vec3> = Vec::with_capacity(((stacks + 1.0) * (slices + 1.0)) as usize);
    let mut indices: Vec<u32> = Vec::with_capacity((stacks * slices * 6.0) as usize);


    for i in 0..=stacks as i32 {
        let theta = i as f32 / stacks * f32::consts::PI;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        for j in 0..=slices as i32 {
            let phi = j as f32 / slices * 2.0 * f32::consts::PI;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            vertices.push(Vec3 {
                x: sin_theta * cos_phi,
                y: cos_theta,
                z: sin_theta * sin_phi
            });
        }
    }

    for i in 0..stacks as u32 {
        for j in 0.. slices as u32 {
            let first = i * (slices as u32 + 1) + j;
            let second = first + slices as u32 + 1;

            indices.push(first);
            indices.push(second);
            indices.push(first + 1);

            indices.push(second);
            indices.push(second + 1);
            indices.push(first + 1);
        }
    }

    return (vertices, indices);
}
