use std::{
    path::PathBuf,
    collections::HashMap,
    rc::Rc,
    cell::RefCell,
    mem::offset_of,
    str::FromStr,
    f32,
};
use ash::vk;

use image::DynamicImage;
use crate::math::{Matrix4, Vec3, Vec4};
use crate::render::{ChunkVertices, CloudsVertices, DescriptorSet, GlobalRenderer, GraphicsPipeline, PipelineSettings, SkyBodiesVertices, SpritesVertices, TextVertices, Texture, Ubo, VulkanApp};
use crate::render::raw_buffer::BufferFlags;
use crate::resources::{BlockItemModel, FontInfo, ShadersCompiler};


pub struct ResourceManager {
    shader_path: String,
    textures_path: String,
    models_path: String,

    pub global_ubo: Ubo,

    pub world_texture: Texture,
    pub ui_sprites_texture: Texture,
    pub ui_fonts_texture: Texture,
    pub sky_bodies_texture: Texture,

    pub global_descriptor: DescriptorSet,

    pipelines: HashMap<&'static str, Rc<RefCell<GraphicsPipeline>>>,
    fonts: HashMap<&'static str, Rc<FontInfo>>,
    models: HashMap<String, Rc<BlockItemModel>>,
}

impl ResourceManager {
    pub const WORLD_TEXTURE_IDX: u8 = 0;

    pub fn new() -> Self {
        let project_path: &'static str = env!("CARGO_MANIFEST_DIR");

        Self {
            shader_path: format!(r"{project_path}\assets\shaders"),
            textures_path: format!(r"{project_path}\assets\textures"),
            models_path: format!(r"{project_path}\assets\models"),

            global_ubo: Ubo::new(),

            world_texture: Texture::new(),
            ui_sprites_texture: Texture::new(),
            ui_fonts_texture: Texture::new(),
            sky_bodies_texture: Texture::new(),

            global_descriptor: DescriptorSet::new(),

            pipelines: HashMap::new(),
            fonts: HashMap::new(),
            models: HashMap::new(),
        }
    }

    pub fn start(&mut self, app: &mut VulkanApp, global_renderer: &mut GlobalRenderer) {
        let mut path = self.textures_path.clone();

        // read atlas and textures
        {
            path.push_str(r"\blocks");
            let images = get_filtered_files_in_directory(&path, "png");
            self.world_texture = Texture::create_from_atlas(app, &images, 256, 256);
        }
        {
            path.clear();
            path.push_str(&self.textures_path);
            path.push_str(r"\ui");
            let images = get_filtered_files_in_directory(&path, "png");

            self.ui_sprites_texture = Texture::create_from_atlas(app, &images, 256, 256);
        }
        {
            path.clear();
            path.push_str(&self.textures_path);
            path.push_str(r"\fonts");
            let images = get_filtered_files_in_directory(&path, "png");
            self.ui_fonts_texture = Texture::create_from_atlas(app, &images, 256, 256);
        }
        {
            path.clear();
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
            path.clear();
            path.push_str(&self.textures_path);
            path.push_str(r"\fonts\default_font.json");
            self.fonts.insert("default", Rc::new(FontInfo::create_from_file(&path, "default_font", &self.ui_fonts_texture)));
        }

        // load models

        self.models.clear();
        self.read_models();


        self.global_ubo = Ubo::new();
        self.global_ubo.add::<Matrix4>("uiProj");
        self.global_ubo.add::<f32>("uiPixelScale");
        self.global_ubo.add::<Matrix4>("camProj");
        self.global_ubo.add::<Matrix4>("camView");
        self.global_ubo.add::<Matrix4>("camViewProj");
        self.global_ubo.add::<Matrix4>("camViewNoTranslate");
        self.global_ubo.add::<Vec3>("skyColor");
        self.global_ubo.add::<Vec3>("fogColor");
        self.global_ubo.add::<Vec3>("lightColor");
        self.global_ubo.add::<Vec3>("darknessColor");
        self.global_ubo.add::<Vec3>("ambientColor");
        self.global_ubo.add::<Vec3>("cloudsColor");
        self.global_ubo.add::<f32>("fogDistance");
        self.global_ubo.add::<f32>("fogDensity");
        self.global_ubo.add::<i32>("fogEnable");
        self.global_ubo.add::<f32>("renderDistance");
        self.global_ubo.create(app, BufferFlags::RAM | BufferFlags::DUPLICATE);


        let used_stages_flag = vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT;

        self.global_descriptor.add_indexing_textures(0, used_stages_flag, &mut [
                &mut self.world_texture.raw_texture,
                &mut self.ui_sprites_texture.raw_texture,
                &mut self.ui_fonts_texture.raw_texture,
                &mut self.sky_bodies_texture.raw_texture,
            ])
            .add_ubo(1, used_stages_flag, &self.global_ubo)
            .create(app);


        // create pipelines
        let mut shaders_compiler = ShadersCompiler::new();
        shaders_compiler.start(self.shader_path.clone());

        {
            let mut settings = PipelineSettings::new(app, &mut shaders_compiler, r"chunk");
            settings.enable_blend = true;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(size_of::<ChunkVertices>(), false)
                .attrib_info(0, vk::Format::R32G32B32_SFLOAT, offset_of!(ChunkVertices, vertices))
                .attrib_info(1, vk::Format::R32G32B32_SFLOAT, offset_of!(ChunkVertices, normal))
                .attrib_info(2, vk::Format::R32G32_SFLOAT, offset_of!(ChunkVertices, uv))
                .attrib_info(3, vk::Format::R8_UINT, offset_of!(ChunkVertices, flags));

            let pipeline = GraphicsPipeline::create(app, settings, global_renderer);

            self.pipelines.insert("chunks", Rc::new(RefCell::new(pipeline)));
        }
        {
            let mut settings = PipelineSettings::new(app, &mut shaders_compiler, r"ui\sprites");
            settings.enable_blend = true;
            settings.enable_depth_test = false;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.set_push_constant(vk::ShaderStageFlags::FRAGMENT, 128, 0);
            settings.vertex_info(4 * size_of::<f32>(), false)
                .attrib_info(0,  vk::Format::R32G32B32A32_SFLOAT, 0);
            settings.vertex_info(size_of::<SpritesVertices>(), true)
                .attrib_info(1, vk::Format::R16G16_SINT, offset_of!(SpritesVertices, position))
                .attrib_info(2, vk::Format::R16G16_SINT, offset_of!(SpritesVertices, size))
                .attrib_info(3, vk::Format::R32G32B32A32_SFLOAT, offset_of!(SpritesVertices, uv))
                .attrib_info(4, vk::Format::R8G8B8A8_UINT, offset_of!(SpritesVertices, color))
                .attrib_info(5, vk::Format::R8_UINT, offset_of!(SpritesVertices, texture_idx));

            let pipeline = GraphicsPipeline::create(app, settings, global_renderer);

            self.pipelines.insert("ui_sprites", Rc::new(RefCell::new(pipeline)));
        }
        {
            let mut settings = PipelineSettings::new(app, &mut shaders_compiler, r"ui\text");
            settings.enable_blend = true;
            settings.enable_depth_test = false;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.set_push_constant(vk::ShaderStageFlags::FRAGMENT, 4, 0);
            settings.vertex_info(4 * size_of::<f32>(), false)
                .attrib_info(0,  vk::Format::R32G32B32A32_SFLOAT, 0);
            settings.vertex_info(size_of::<TextVertices>(), true)
                .attrib_info(1, vk::Format::R16G16_SINT, offset_of!(TextVertices, position))
                .attrib_info(2, vk::Format::R8G8_UINT, offset_of!(TextVertices, size))
                .attrib_info(3, vk::Format::R32G32B32A32_SFLOAT, offset_of!(TextVertices, uv))
                .attrib_info(4, vk::Format::R16G16_SINT, offset_of!(TextVertices, advance))
                .attrib_info(5, vk::Format::R8G8B8_UINT, offset_of!(TextVertices, color));

            let pipeline = GraphicsPipeline::create(app, settings, global_renderer);

            self.pipelines.insert("ui_text", Rc::new(RefCell::new(pipeline)));
        }
        {
            let mut settings = PipelineSettings::new(app, &mut shaders_compiler, r"clouds");
            settings.enable_blend = true;
            settings.enable_depth_test = true;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(6 * size_of::<i8>(), false)
               .attrib_info(0, vk::Format::R8G8B8_SINT, 0)
               .attrib_info(1, vk::Format::R8G8B8_SINT, 3 * size_of::<i8>());
            settings.vertex_info(size_of::<CloudsVertices>(), true)
                .attrib_info(2, vk::Format::R32G32_SFLOAT, offset_of!(CloudsVertices, position))
                .attrib_info(3, vk::Format::R8_UINT, offset_of!(CloudsVertices, cullface));

            let pipeline = GraphicsPipeline::create(app, settings, global_renderer);

            self.pipelines.insert("clouds", Rc::new(RefCell::new(pipeline)));
        }
        {
            let mut settings = PipelineSettings::new(app, &mut shaders_compiler, r"skyDome");
            settings.enable_blend = false;
            settings.enable_depth_test = false;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(size_of::<Vec3>(), false)
                .attrib_info(0, vk::Format::R32G32B32_SFLOAT, 0);

            let pipeline = GraphicsPipeline::create(app, settings, global_renderer);

            self.pipelines.insert("skyDome", Rc::new(RefCell::new(pipeline)));
        }
        {
            let mut settings = PipelineSettings::new(app, &mut shaders_compiler, r"selectionBox");
            settings.enable_blend = true;
            settings.enable_depth_test = true;
            settings.set_push_constant(vk::ShaderStageFlags::VERTEX, size_of::<Vec3>(), 0);
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(6, false)
                .attrib_info(0, vk::Format::R8G8B8_SINT, 0);

            let pipeline = GraphicsPipeline::create(app, settings, global_renderer);

            self.pipelines.insert("selectionBox", Rc::new(RefCell::new(pipeline)));
        }
        {
            let mut settings = PipelineSettings::new(app, &mut shaders_compiler, r"skyBodies");
            settings.enable_blend = true;
            settings.enable_depth_test = false;
            settings.set_push_constant(vk::ShaderStageFlags::VERTEX, size_of::<Matrix4>() + size_of::<f32>(), 0);
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(5 * size_of::<f32>(), false)
                .attrib_info(0, vk::Format::R32G32B32_SFLOAT, 0)
                .attrib_info(1, vk::Format::R32G32_SFLOAT, 3 * size_of::<f32>());
            settings.vertex_info(size_of::<SkyBodiesVertices>(), true)
                .attrib_info(2, vk::Format::R32G32B32A32_SFLOAT, 0)
                .attrib_info(3, vk::Format::R32G32B32A32_SFLOAT, 1 * size_of::<Vec4>())
                .attrib_info(4, vk::Format::R32G32B32A32_SFLOAT, 2 * size_of::<Vec4>())
                .attrib_info(5, vk::Format::R32G32B32A32_SFLOAT, 3 * size_of::<Vec4>())
                .attrib_info(6, vk::Format::R32G32B32A32_SFLOAT, offset_of!(SkyBodiesVertices, uv))
                .attrib_info(7, vk::Format::R32G32B32A32_SFLOAT, offset_of!(SkyBodiesVertices, color));

            let pipeline = GraphicsPipeline::create(app, settings, global_renderer);

            self.pipelines.insert("skyBodies", Rc::new(RefCell::new(pipeline)));
        }
    }

    pub fn cleanup(&mut self, app: &mut VulkanApp) {
        self.world_texture.destroy(app);
        self.ui_sprites_texture.destroy(app);
        self.ui_fonts_texture.destroy(app);
        self.sky_bodies_texture.destroy(app);

        self.global_descriptor.destroy(app);
        self.global_ubo.destroy(app);
    }

    pub fn get_pipeline(&self, name: &str) -> Rc<RefCell<GraphicsPipeline>> {
        if let Some(pipeline) = self.pipelines.get(name) {
            return pipeline.clone()
        }

        panic!("Resource not found: {}", name);
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
        let block_paths = get_filtered_files_in_directory(&format!(r"{}\blocks", self.models_path), "json");

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

        let items_paths = get_filtered_files_in_directory(&format!(r"{}\items", self.models_path), "json");

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

pub fn get_files_in_directory(path: &str) -> Vec<PathBuf> {
    let dir = std::fs::read_dir(path).expect("Error to read Dir");
    let mut files_path: Vec<PathBuf> = vec!();


    for file in dir {
        let path = match file {
            Ok(f) => f.path(),
            Err(err) => panic!("Error to read Dir: {}", err.to_string()),
        };
        files_path.push(path);
    }

    return files_path;
}

pub fn get_filtered_files_in_directory(path: &str, filter: &str) -> Vec<PathBuf> {
    let dir = match std::fs::read_dir(path) {
        Ok(d) => d,
        Err(err) => panic!("Error to read Dir: '{path}': {}", err.to_string()),
    };

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
