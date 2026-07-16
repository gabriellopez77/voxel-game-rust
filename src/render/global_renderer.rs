use std::{cell::RefCell, collections::HashMap, mem::offset_of, rc::Rc, usize};
use ash::{vk, vk::Handle};

use crate::{math::Vec3, render::{ChunkVertices, CloudsVertices, DescriptorSet, DrawInfo, GlobalUboData, GraphicsPipeline, Material, ParticlesVertices, PipelineLayout, PipelineSettings, SkyBodiesVertices, SpritesVertices, TextVertices, Ubo, VulkanApp, material::MaterialType, raw_buffer::BufferFlags, vertices_attributes::BuffersTypes, vkutl}, resources::{ResourceManager, ShadersCompiler}, utils::MutSafePtr};


pub struct GlobalRenderer {
    app: MutSafePtr<VulkanApp>,

    pub global_ubo: Ubo<GlobalUboData>,
    pub global_descriptor: DescriptorSet,
    pipelines: HashMap<&'static str, Rc<RefCell<GraphicsPipeline>>>,

    chunks_pipeline: Option<Rc<RefCell<GraphicsPipeline>>>,

    sky_draw_list: Vec<DrawInfo>,
    chunks_opaque_draw_list: Vec<DrawInfo>,
    chunks_alpha_draw_list: Vec<DrawInfo>,
    opaque_draw_list: Vec<DrawInfo>,
    alpha_draw_list: Vec<DrawInfo>,
    ui_draw_list: Vec<DrawInfo>,
    particle_draw_list: Vec<DrawInfo>,

    push_constant_list: Vec<(u8, [u8; vkutl::MAX_PUSH_CONSTANT_SIZE])>,

    frame_index: usize,

    pipeline_layouts: Vec<(vk::PipelineLayout, [vk::DescriptorSetLayout; vkutl::MAX_DESCRIPTORS_BINDING_COUNT])>,
}

impl GlobalRenderer {
    pub const WORLD_TEXTURE_IDX: u8 = 0;
    pub const UI_SPRITES_TEXTURE_IDX: u8 = 1;

    pub fn new(app: &mut VulkanApp) -> Self {
        Self {
            app: MutSafePtr::new(app),

            global_ubo: Ubo::new(),
            global_descriptor: DescriptorSet::new(),
            pipelines: HashMap::new(),

            chunks_pipeline: None,

            sky_draw_list: Vec::new(),
            chunks_opaque_draw_list: Vec::new(),
            chunks_alpha_draw_list: Vec::new(),
            opaque_draw_list: Vec::new(),
            alpha_draw_list: Vec::new(),
            ui_draw_list: Vec::new(),
            particle_draw_list: Vec::new(),

            push_constant_list: Vec::new(),

            frame_index: 0,

            pipeline_layouts: Vec::new(),
        }
    }

    pub fn start(&mut self, resources: &mut ResourceManager) {
        let add_pipeline = |app: &mut VulkanApp, shader_compiler: &mut ShadersCompiler, settings: PipelineSettings| -> Rc<RefCell<GraphicsPipeline>> {
            Rc::new(RefCell::new(GraphicsPipeline::create(app, shader_compiler, settings)))
        };

        self.global_ubo.create(&mut self.app, BufferFlags::RAM | BufferFlags::DUPLICATE);


        let used_stages_flag = vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT;

        self.global_descriptor.add_indexing_textures(0, used_stages_flag, &mut [
                &mut resources.world_texture.raw_texture,
                &mut resources.ui_sprites_texture.raw_texture,
                &mut resources.ui_fonts_texture.raw_texture,
                &mut resources.sky_bodies_texture.raw_texture,
            ])
            .add_ubo(1, used_stages_flag, self.global_ubo.buffer.get_all_buffers())
            .create(&self.app);


        // create pipelines
        let mut shaders_compiler = ShadersCompiler::new();
        shaders_compiler.start(resources.shader_path.clone());

        {
            let mut settings = PipelineSettings::new(r"chunk");
            settings.enable_blend = true;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(size_of::<ChunkVertices>(), false)
                .add_attrib(vk::Format::R32G32B32_SFLOAT, offset_of!(ChunkVertices, vertices))
                .add_attrib(vk::Format::R32G32B32_SFLOAT, offset_of!(ChunkVertices, normal))
                .add_attrib(vk::Format::R32G32_SFLOAT, offset_of!(ChunkVertices, uv))
                .add_attrib(vk::Format::R8_UINT, offset_of!(ChunkVertices, flags));

            self.create_pipeline_layout(&mut settings.pipeline_layout);
            self.pipelines.insert("chunks", add_pipeline(&mut self.app, &mut shaders_compiler, settings));
        }
        {
            let mut settings = PipelineSettings::new(r"ui\sprites");
            settings.enable_blend = true;
            settings.enable_depth_test = false;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(4 * size_of::<f32>(), false)
                .add_attrib( vk::Format::R32G32B32A32_SFLOAT, 0);
            settings.vertex_info(size_of::<SpritesVertices>(), true)
                .add_attrib(vk::Format::R16G16_SINT, offset_of!(SpritesVertices, position))
                .add_attrib(vk::Format::R16G16_SINT, offset_of!(SpritesVertices, size))
                .add_attrib(vk::Format::R32G32B32A32_SFLOAT, offset_of!(SpritesVertices, uv))
                .add_attrib(vk::Format::R8G8B8A8_UINT, offset_of!(SpritesVertices, color))
                .add_attrib(vk::Format::R8_UINT, offset_of!(SpritesVertices, texture_idx));

            self.create_pipeline_layout(&mut settings.pipeline_layout);
            self.pipelines.insert("ui_sprites", add_pipeline(&mut self.app, &mut shaders_compiler, settings));
        }
        {
            let mut settings = PipelineSettings::new(r"ui\text");
            settings.enable_blend = true;
            settings.enable_depth_test = false;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(4 * size_of::<f32>(), false)
                .add_attrib( vk::Format::R32G32B32A32_SFLOAT, 0);
            settings.vertex_info(size_of::<TextVertices>(), true)
                .add_attrib(vk::Format::R16G16_SINT, offset_of!(TextVertices, position))
                .add_attrib(vk::Format::R8G8_UINT, offset_of!(TextVertices, size))
                .add_attrib(vk::Format::R32G32B32A32_SFLOAT, offset_of!(TextVertices, uv))
                .add_attrib(vk::Format::R16G16_SINT, offset_of!(TextVertices, advance))
                .add_attrib(vk::Format::R8G8B8_UINT, offset_of!(TextVertices, color));

            self.create_pipeline_layout(&mut settings.pipeline_layout);
            self.pipelines.insert("ui_text", add_pipeline(&mut self.app, &mut shaders_compiler, settings));
        }
        {
            let mut settings = PipelineSettings::new(r"clouds");
            settings.enable_blend = true;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(6 * size_of::<i8>(), false)
               .add_attrib(vk::Format::R8G8B8_SINT, 0)
               .add_attrib(vk::Format::R8G8B8_SINT, 3 * size_of::<i8>());
            settings.vertex_info(size_of::<CloudsVertices>(), true)
                .add_attrib(vk::Format::R32G32_SFLOAT, offset_of!(CloudsVertices, position))
                .add_attrib(vk::Format::R8_UINT, offset_of!(CloudsVertices, cullface));

            self.create_pipeline_layout(&mut settings.pipeline_layout);
            self.pipelines.insert("clouds", add_pipeline(&mut self.app, &mut shaders_compiler, settings));
        }
        {
            let mut settings = PipelineSettings::new(r"skyDome");
            settings.enable_depth_test = false;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(size_of::<Vec3>(), false)
                .add_attrib(vk::Format::R32G32B32_SFLOAT, 0);

            self.create_pipeline_layout(&mut settings.pipeline_layout);
            self.pipelines.insert("skyDome", add_pipeline(&mut self.app, &mut shaders_compiler, settings));
        }
        {
            let mut settings = PipelineSettings::new(r"selectionBox");
            settings.enable_blend = true;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(6, false)
                .add_attrib(vk::Format::R8G8B8_SINT, 0);

            self.create_pipeline_layout(&mut settings.pipeline_layout);
            self.pipelines.insert("selectionBox", add_pipeline(&mut self.app, &mut shaders_compiler, settings));
        }
        {
            let mut settings = PipelineSettings::new(r"skyBodies");
            settings.enable_blend = true;
            settings.enable_depth_test = false;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(5 * size_of::<f32>(), false)
                .add_attrib(vk::Format::R32G32B32_SFLOAT, 0)
                .add_attrib(vk::Format::R32G32_SFLOAT, 3 * size_of::<f32>());
            settings.vertex_info(size_of::<SkyBodiesVertices>(), true)
                .add_attrib_matrix(offset_of!(SkyBodiesVertices, matrix))
                .add_attrib(vk::Format::R32G32B32A32_SFLOAT, offset_of!(SkyBodiesVertices, uv))
                .add_attrib(vk::Format::R32G32B32A32_SFLOAT, offset_of!(SkyBodiesVertices, color));

            self.create_pipeline_layout(&mut settings.pipeline_layout);
            self.pipelines.insert("skyBodies", add_pipeline(&mut self.app, &mut shaders_compiler, settings));
        }
        {
            let mut settings = PipelineSettings::new(r"particles");
            settings.enable_blend = true;
            settings.add_descriptor_set(&self.global_descriptor);
            settings.vertex_info(5 * size_of::<f32>(), false)
                .add_attrib(vk::Format::R32G32B32_SFLOAT, 0)
                .add_attrib(vk::Format::R32G32_SFLOAT, 3 * size_of::<f32>());
            settings.vertex_info(size_of::<ParticlesVertices>(), true)
                .add_attrib(vk::Format::R32G32B32A32_SFLOAT, offset_of!(ParticlesVertices, position))
                .add_attrib(vk::Format::R32G32B32A32_SFLOAT, offset_of!(ParticlesVertices, scale))
                .add_attrib(vk::Format::R32G32B32A32_SFLOAT, offset_of!(ParticlesVertices, rotation))
                .add_attrib(vk::Format::R32G32B32A32_SFLOAT, offset_of!(ParticlesVertices, uv))
                .add_attrib(vk::Format::R8_UINT, offset_of!(ParticlesVertices, texture_idx));

            self.create_pipeline_layout(&mut settings.pipeline_layout);
            self.pipelines.insert("particles", add_pipeline(&mut self.app, &mut shaders_compiler, settings));
        }

        self.chunks_pipeline = Some(self.pipelines.get("chunks").unwrap().clone());
    }


    pub fn cleanup(&mut self) {
        self.global_descriptor.destroy(&mut self.app);
        self.global_ubo.destroy(&mut self.app);
    }

    pub fn create_pipeline_layout(&mut self, layout: &mut PipelineLayout) {
        let shared_layout = match self.pipeline_layouts.iter().find(|x| layout.is_same(&x.1)) {
            Some(layout) => layout.0,
            None => vk::PipelineLayout::null()
        };

        layout.create(&self.app, shared_layout);

        // save shared layout
        if shared_layout.is_null() {
            self.pipeline_layouts.push((layout.get_layout(), layout.descriptors_layout));
        }
    }

    pub fn create_material(&mut self, pipeline_name: &'static str, material_type: MaterialType) -> Material {
        Material::new(&mut self.app, self.pipelines.get(pipeline_name).unwrap().clone(), material_type)
    }

    pub fn create_chunk_material(&mut self, material_type: MaterialType) -> Material {
        Material::new(&mut self.app, self.chunks_pipeline.as_ref().unwrap().clone(), material_type)
    }

    pub fn draw_obj_instanced(&mut self, material: &Material, instance_count: usize) {
        // item is not suitable to draw
        if instance_count == 0 || material.get_triangles_count() == 0 { return }

        self.prepare_draw_info(material, instance_count, material.get_push_constant_info());
    }

    pub fn draw_obj_instanced_with_buffer<T>(&mut self, material: &mut Material, instance_data: &mut Vec<T>) {
        // item is not suitable to draw
        if instance_data.len() == 0 || material.get_triangles_count() == 0 { return }

        material.update_instance_data(&instance_data);
        self.prepare_draw_info(material, instance_data.len(), material.get_push_constant_info());

        instance_data.clear();
    }

    pub fn draw_obj(&mut self, material: &Material) {
        // item is not suitable to draw
        if material.get_triangles_count() == 0 { return }

        self.prepare_draw_info(material, 1, material.get_push_constant_info());
    }

    fn prepare_draw_info(&mut self, material: &Material, instance_count: usize,
                             push_constant_info: (u8, &[u8; vkutl::MAX_PUSH_CONSTANT_SIZE])) {
        let mut draw_info = material.create_draw_info(self.frame_index);

        draw_info.instance_count = instance_count as u32;

        if push_constant_info.0 != 0 {
            draw_info.push_constant_idx = self.push_constant_list.len() as i32;

            self.push_constant_list.push((push_constant_info.0, *push_constant_info.1))
        }

        match material.get_type() {
            MaterialType::ChunksOpaque => self.chunks_opaque_draw_list.push(draw_info),
            MaterialType::ChunksAlpha => self.chunks_alpha_draw_list.push(draw_info),
            MaterialType::Opaque => self.opaque_draw_list.push(draw_info),
            MaterialType::Alpha => self.alpha_draw_list.push(draw_info),
            MaterialType::Sky => self.sky_draw_list.push(draw_info),
            MaterialType::Ui => self.ui_draw_list.push(draw_info),
            MaterialType::Particle => self.particle_draw_list.push(draw_info),
        }
    }

    pub fn begin(&mut self) {
        self.sky_draw_list.clear();
        self.chunks_opaque_draw_list.clear();
        self.chunks_alpha_draw_list.clear();
        self.opaque_draw_list.clear();
        self.alpha_draw_list.clear();
        self.ui_draw_list.clear();
        self.particle_draw_list.clear();
        self.push_constant_list.clear();

        self.frame_index = self.app.frame_index;
    }

    pub fn end(&mut self) {
        //let now = std::time::Instant::now();
        self.opaque_draw_list.sort();
        self.alpha_draw_list.sort();
        //println!("{}", now.elapsed().as_micros());
        //let now = std::time::Instant::now();
        self.render(&self.sky_draw_list);
        self.render(&self.chunks_opaque_draw_list);
        self.render(&self.opaque_draw_list);
        self.render(&self.chunks_alpha_draw_list);
        self.render(&self.alpha_draw_list);
        self.render(&self.particle_draw_list);
        self.render(&self.ui_draw_list);
        //println!("{}", now.elapsed().as_micros());
    }

    fn render(&self, draw_list: &Vec<DrawInfo>) {
        if draw_list.is_empty() { return }

        let mut current_pipeline = vk::Pipeline::null();
        let mut current_pipeline_layout = vk::PipelineLayout::null();
        let mut current_descriptor_sets = [vk::DescriptorSet::null(); vkutl::MAX_DESCRIPTORS_BINDING_COUNT];

        let vulkan_app = &*self.app;

        let command_buffer = vulkan_app.get_current_command_buffer();


        for draw_info in draw_list {
            if current_pipeline != draw_info.pipeline {
                current_pipeline = draw_info.pipeline;

                unsafe {
                    vulkan_app.ash_device.cmd_bind_pipeline(command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        current_pipeline
                    );
                }
            }

            if current_pipeline_layout != draw_info.pipeline_layout {
                current_pipeline_layout = draw_info.pipeline_layout;

                let mut first_set = usize::MAX;

                for i in 0..draw_info.descriptors_count as usize {
                    if draw_info.descriptors_sets[i] != current_descriptor_sets[i] {
                        current_descriptor_sets[i] = draw_info.descriptors_sets[i];

                        if first_set == usize::MAX {
                            first_set = i;
                        }
                    }
                }

                unsafe {
                    vulkan_app.ash_device.cmd_bind_descriptor_sets(command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        current_pipeline_layout,
                        first_set as u32,
                        &draw_info.descriptors_sets[first_set..draw_info.descriptors_count as usize],
                        &[]
                    );
                }
            }

            if draw_info.push_constant_idx != -1 {
                let (push_size, push_data) = &self.push_constant_list[draw_info.push_constant_idx as usize];

                unsafe {
                    vulkan_app.ash_device.cmd_push_constants(command_buffer,
                        current_pipeline_layout,
                        vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                        0,
                        &push_data[0..*push_size as usize]
                    );
                }
            }


            const OFFSETS: [u64; vkutl::MAX_VERTEX_BINDING_COUNT] = [0; vkutl::MAX_VERTEX_BINDING_COUNT];

            unsafe {
                let count = if draw_info.vertices_buffer[BuffersTypes::Instance as usize].is_null() { 1 } else { 2 };

                vulkan_app.ash_device.cmd_bind_vertex_buffers(command_buffer,
                    0,
                    &draw_info.vertices_buffer[0..count],
                    &OFFSETS[0..count],
                );

                vulkan_app.ash_device.cmd_bind_index_buffer(command_buffer,
                    draw_info.index_buffer,
                    0,
                    vk::IndexType::UINT32
                );

                //let d = vk::DrawIndexedIndirectCommand {
                //    index_count: draw_info.index_count,
                //    instance_count: 1,
                //    first_index: 0, // range.start in index buffer
                //    vertex_offset: 0, // range.start in vertices buffer
                //    first_instance: 0,
                //};

                vulkan_app.ash_device.cmd_draw_indexed(command_buffer,
                    draw_info.index_count,
                    draw_info.instance_count,
                    0, 0, 0
                );
            }
        }
    }
}
