use std::{cell::RefCell, rc::Rc, usize};

use ash::vk::{self, Handle};

use crate::{render::{DrawInfo, GraphicsPipeline, Material, PipelineLayout, VulkanApp, material::MaterialType, vertices_attributes::BuffersTypes, vkutl}, resources::ResourceManager, utils::{NullSafePtr, MutSafePtr}};


pub struct GlobalRenderer {
    app: MutSafePtr<VulkanApp>,
    resources_manager: NullSafePtr<ResourceManager>,

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
    pub fn new(app: &mut VulkanApp) -> Self {
        Self {
            app: MutSafePtr::from(app),
            resources_manager: NullSafePtr::null(),

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

    pub fn start(&mut self, resources_manager: &ResourceManager) {
        self.chunks_pipeline = Some(resources_manager.get_pipeline("chunks"));
        self.resources_manager = NullSafePtr::from(resources_manager);
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
        Material::new(&mut self.app, self.resources_manager.get_pipeline(pipeline_name), material_type)
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
        self.sky_draw_list.sort();
        self.opaque_draw_list.sort();
        self.alpha_draw_list.sort();
        self.ui_draw_list.sort();
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
