use std::{
    path::PathBuf,
    collections::HashMap,
    rc::Rc,
    str::FromStr,
    f32,
};

use image::DynamicImage;
use crate::math::Vec3;
use crate::render::{Texture, VulkanApp};
use crate::resources::{BlockItemModel, FontInfo};
use crate::ui::ButtonsStyles;


pub struct ResourceManager {
    pub shader_path: String,
    pub textures_path: String,
    pub models_path: String,

    pub world_texture: Texture,
    pub ui_sprites_texture: Texture,
    pub ui_fonts_texture: Texture,
    pub sky_bodies_texture: Texture,

    fonts: HashMap<&'static str, Rc<FontInfo>>,
    models: HashMap<String, Rc<BlockItemModel>>,

    pub ui_buttons_styles: ButtonsStyles,
}

impl ResourceManager {
    pub fn new() -> Self {
        let project_path: &'static str = env!("CARGO_MANIFEST_DIR");

        Self {
            shader_path: format!(r"{project_path}\assets\shaders"),
            textures_path: format!(r"{project_path}\assets\textures"),
            models_path: format!(r"{project_path}\assets\models"),

            world_texture: Texture::new(),
            ui_sprites_texture: Texture::new(),
            ui_fonts_texture: Texture::new(),
            sky_bodies_texture: Texture::new(),


            fonts: HashMap::new(),
            models: HashMap::new(),

            ui_buttons_styles: ButtonsStyles::new(),
        }
    }

    pub fn start(&mut self, app: &mut VulkanApp) {
        // read atlas and textures
        {
            let images = get_files_in_directory(&format!("{}/blocks", self.textures_path), "png");
            self.world_texture = Texture::create_from_atlas(app, &images, 256, 256);
        }
        {
            let mut images = get_files_in_directory(&format!("{}/ui", self.textures_path), "png");
            let mut buttons_image = get_files_in_directory(&format!("{}/ui/buttons", self.textures_path), "png");
            images.append(&mut buttons_image);
            self.ui_sprites_texture = Texture::create_from_atlas(app, &images, 256, 256);
        }
        {
            let images = get_files_in_directory(&format!("{}/fonts", self.textures_path), "png");
            self.ui_fonts_texture = Texture::create_from_atlas(app, &images, 256, 256);
        }
        {
            let images = [
                PathBuf::from_str(&format!(r"{}\misc\moon.png", self.textures_path)).unwrap(),
                //PathBuf::from_str(&format!(r"{}\misc\stars.png", self.textures_path)).unwrap(),
                PathBuf::from_str(&format!(r"{}\misc\sun.png", self.textures_path)).unwrap(),
                PathBuf::from_str(&format!(r"{}\error_404.png", self.textures_path)).unwrap(),
            ];

            self.sky_bodies_texture = Texture::create_from_atlas(app, &images, 96, 96);
        }


        // load fonts
        {
            let path = format!("{}/fonts/default_font.json", self.textures_path);
            self.fonts.insert("default", Rc::new(FontInfo::create_from_file(&path, "default_font", &self.ui_fonts_texture)));
        }

        self.read_models();
        self.ui_buttons_styles.load_styles(&format!("{}/ui/buttons/buttons.json", self.textures_path), &self.ui_sprites_texture);
    }

    pub fn cleanup(&mut self, app: &mut VulkanApp) {
        self.world_texture.destroy(app);
        self.ui_sprites_texture.destroy(app);
        self.ui_fonts_texture.destroy(app);
        self.sky_bodies_texture.destroy(app);
    }

    pub fn get_font(&self, name: &str) -> Rc<FontInfo> {
        if let Some(font) = self.fonts.get(name) {
            return font.clone();
        }

        panic!("Resource not found: {}", name);
    }

    pub fn get_model(&self, name: &str) -> Rc<BlockItemModel> {
        if let Some(model) = self.models.get(name) {
            return model.clone();
        }

        // is guaranteed that contains the 'error_404' texture coords
        return self.models.get("error_404").unwrap().clone();
    }

    pub fn read_texture(&self, relative_path: &str) -> DynamicImage {
        image::open(format!(r"{}\{relative_path}", self.textures_path)).expect("Failed to load texture")
    }

    fn read_models(&mut self) {
        let block_paths = get_files_in_directory(&format!(r"{}\blocks", self.models_path), "json");

        // load error model
        self.models.insert("error_404".to_string(), Rc::new(BlockItemModel::read_error_model(&self.world_texture)));

        for path in &block_paths {
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();

            let model = match BlockItemModel::new(&self.models_path, &path.to_str().unwrap(), &self.world_texture) {
                Ok(m) => m,
                Err(err) => {
                    println!("{err}");
                    BlockItemModel::read_error_model(&self.world_texture)
                }
            };

            self.models.insert(name, Rc::new(model));
        }

        let items_paths = get_files_in_directory(&format!(r"{}\items", self.models_path), "json");

        for path in &items_paths {
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();

            let model = match BlockItemModel::new(&self.models_path, &path.to_str().unwrap(), &self.world_texture) {
                Ok(m) => m,
                Err(err) => {
                    println!("{err}");
                    BlockItemModel::read_error_model(&self.world_texture)
                }
            };

            self.models.insert(name, Rc::new(model));
        }
    }
}

pub fn get_files_in_directory(path: &str, extension: &str) -> Vec<PathBuf> {
    let dir = match std::fs::read_dir(path) {
        Ok(d) => d,
        Err(err) => panic!("Error to read Dir: '{path}': {}", err.to_string()),
    };

    let mut files_path: Vec<PathBuf> = vec!();

    for file in dir {
        let file_path = file.unwrap().path();
        let file_extension = file_path.extension();

        if file_extension.is_some()  && file_extension.unwrap() == extension {
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
