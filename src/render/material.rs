use ash::vk::{self, Handle};

use crate::{math::Vec4, render::{GlobalRenderer, core::{PipelineLayout, vkutl}}, resources::ShadersCompiler, utils::SafePtrMut};
use super::core::VulkanApp;


#[derive(Clone, Copy, Default)]
pub struct VertexAttribInfo {
    bindings_info: [vk::VertexInputBindingDescription; vkutl::MAX_VERTEX_BINDING_COUNT],
    attributes_info: [vk::VertexInputAttributeDescription; vkutl::MAX_VERTEX_ATTRIBUTES_COUNT],

    current_location: u32,
    current_binding: u32,
    current_attributes_index: u32
}

impl VertexAttribInfo {
    pub fn get_attributes(&self) -> &[vk::VertexInputAttributeDescription] {
        &self.attributes_info[0..self.current_attributes_index as usize]
    }

    pub fn get_bindings(&self) -> &[vk::VertexInputBindingDescription] {
        &self.bindings_info[0..self.current_binding as usize]
    }

    pub fn add_vertex(&mut self, stride: usize, is_instance: bool) -> &mut Self {
        let binding = self.current_binding;
        self.current_binding += 1;

        debug_assert!(self.current_binding <= vkutl::MAX_VERTEX_BINDING_COUNT as u32, "max binding count is: {}", vkutl::MAX_VERTEX_BINDING_COUNT);

        self.bindings_info[binding as usize] = vk::VertexInputBindingDescription {
            binding: binding,
            stride: stride as u32,
            input_rate: if is_instance { vk::VertexInputRate::INSTANCE } else { vk::VertexInputRate::VERTEX },
        };

        self
    }

    pub fn add_attribute(&mut self, format: vk::Format, offset: usize) -> &mut Self {
        let index = self.current_attributes_index;
        self.current_attributes_index += 1;

        debug_assert!(self.current_binding <= vkutl::MAX_VERTEX_ATTRIBUTES_COUNT as u32, "max attributes count is: {}", vkutl::MAX_VERTEX_ATTRIBUTES_COUNT);

        self.attributes_info[index as usize] = vk::VertexInputAttributeDescription {
            location: self.current_location,
            binding: self.current_binding - 1,
            format,
            offset: offset as u32
        };

        self.current_location += 1;

        self
    }

    pub fn add_attribute_matrix(&mut self, offset: usize) -> &mut Self {
        self.add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset + 0 * size_of::<Vec4>());
        self.add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset + 1 * size_of::<Vec4>());
        self.add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset + 2 * size_of::<Vec4>());
        self.add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset + 3 * size_of::<Vec4>());

        self
    }

    pub fn add_attribute_array_vec4(&mut self, offset: usize, array_len: usize) -> &mut Self {
        for i in 0..array_len {
            self.add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset + i * size_of::<Vec4>());
        }

        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MaterialType {
    Sky,
    ChunksOpaque,
    ChunksAlpha,
    Opaque,
    Alpha,
    Particle,
    FirstPerson,
    Ui,
}

pub struct Material {
    app: SafePtrMut<VulkanApp>,

    pipeline: vk::Pipeline,
    pipeline_layout: PipelineLayout,

    material_type: MaterialType,

    vertex_shader_name: String,
    fragment_shader_name: String,

    vertex_attributes_info: VertexAttribInfo,
    cull_mode: vk::CullModeFlags,
    blend: bool,
    depth_test: bool,
    topology: vk::PrimitiveTopology,
    line_width: f32,

    modified: bool,
}

unsafe impl Send for Material {}

impl Material {
    pub fn new(app: SafePtrMut<VulkanApp>, shader_name: &'static str, material_type: MaterialType) -> Self {
        Self {
            app,

            pipeline: vk::Pipeline::null(),
            pipeline_layout: PipelineLayout::new(),

            material_type: material_type,

            vertex_shader_name: format!("{shader_name}.vsh"),
            fragment_shader_name: format!("{shader_name}.fsh"),

            vertex_attributes_info: VertexAttribInfo::default(),
            cull_mode: vk::CullModeFlags::FRONT,
            blend: false,
            depth_test: true,
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            line_width: 1.0,

            modified: true,
        }
    }

    pub fn get_type(&self) -> MaterialType { self.material_type }

    pub fn destroy(&mut self) {
        self.destroy_pipeline();
    }

    pub fn get_draw_info(&mut self,
        global_renderer: &mut GlobalRenderer,
        frame_index: usize
    ) -> Option<(
        vk::Pipeline,
        vk::PipelineLayout,
        [vk::DescriptorSet; vkutl::MAX_DESCRIPTORS_BINDING_COUNT],
        u32
    )> {
        if self.modified {
            self.destroy_pipeline();
            self.create_pipeline(global_renderer);

            self.modified = false;
        }

        if self.pipeline.is_null() {
            return None;
        }

        return Some((
            self.pipeline,
            self.pipeline_layout.get_layout(),
            self.pipeline_layout.get_descriptors_sets(frame_index).clone(),
            self.pipeline_layout.descriptors_count,
        ));
    }

    pub fn set_attributes_info(&mut self, info: VertexAttribInfo)  {
        self.vertex_attributes_info = info;
        self.modified = true;
    }

    pub fn set_cull_mode(&mut self, mode: vk::CullModeFlags) {
        self.cull_mode = mode;
        self.modified = true;
    }

    pub fn set_blend(&mut self, value: bool) {
        self.blend = value;
        self.modified = true;
    }

    pub fn set_depth_test(&mut self, value: bool) {
        self.depth_test = value;
        self.modified = true;
    }

    pub fn set_topology(&mut self, topology: vk::PrimitiveTopology) {
        self.topology = topology;
        self.modified = true;
    }

    pub fn set_line_width(&mut self, width: f32) {
        self.line_width = width;
        self.modified = true;
    }

    fn create_pipeline(&mut self, global_renderer: &mut GlobalRenderer) {
        self.pipeline_layout = global_renderer.global_pipeline_layout.clone();

        const DYNAMIC_STATES: [vk::DynamicState; 2] = [ vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR ];

        let vertex_module = self.compile_shader(&mut global_renderer.shaders_compiler, shaderc::ShaderKind::Vertex, &self.vertex_shader_name);
        let fragment_module = self.compile_shader(&mut global_renderer.shaders_compiler, shaderc::ShaderKind::Fragment, &self.fragment_shader_name);

        let shader_stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vertex_module)
                .name(c"main"),

            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(fragment_module)
                .name(c"main")
        ];


        let bindings = self.vertex_attributes_info.get_bindings();
        let attributes = self.vertex_attributes_info.get_attributes();

        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&attributes);

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(self.topology)
            .primitive_restart_enable(false);

        let mut viewport_state = vk::PipelineViewportStateCreateInfo::default();
        viewport_state.viewport_count = 1;
        viewport_state.p_viewports = std::ptr::null();   // viewport is dynamic
        viewport_state.scissor_count = 1;
        viewport_state.p_scissors = std::ptr::null();   // scissor is dynamic

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(self.line_width)
            .cull_mode(self.cull_mode)
            .front_face(vk::FrontFace::CLOCKWISE);

        let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let color_blend_attachment = [
            vk::PipelineColorBlendAttachmentState::default()
                .color_write_mask(vk::ColorComponentFlags::RGBA)
                .blend_enable(self.blend)
                .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
                .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                .color_blend_op(vk::BlendOp::ADD)
                .src_alpha_blend_factor(vk::BlendFactor::ONE)
                .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
                .alpha_blend_op(vk::BlendOp::ADD)
        ];

        let color_blending = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op(vk::LogicOp::COPY)
            .attachments(&color_blend_attachment);

        let dynamic_state = vk::PipelineDynamicStateCreateInfo::default()
            .dynamic_states(&DYNAMIC_STATES);

        let depth_stencil = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(self.depth_test)
            .depth_write_enable(true)
            .depth_compare_op(vk::CompareOp::LESS)
            .depth_bounds_test_enable(false)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0);

        let pipeline_info = [
            vk::GraphicsPipelineCreateInfo::default()
                .stages(&shader_stages)
                .vertex_input_state(&vertex_input_info)
                .input_assembly_state(&input_assembly)
                .viewport_state(&viewport_state)
                .rasterization_state(&rasterizer)
                .multisample_state(&multisampling)
                .depth_stencil_state(&depth_stencil)
                .color_blend_state(&color_blending)
                .dynamic_state(&dynamic_state)
                .layout(self.pipeline_layout.get_layout())
                .render_pass(self.app.render_pass)
                .subpass(0)
                .base_pipeline_handle(vk::Pipeline::null())
                .base_pipeline_index(-1)
        ];

        self.pipeline = unsafe {
            match self.app.ash_device.create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_info, None) {
                Ok(pipeline) => pipeline[0],
                Err(err) => {
                    println!("Failed to create graphics pipeline: {}", err.1);

                    vk::Pipeline::null()
                }
            }
        };

        // destroy shaders modules
        unsafe {
            self.app.ash_device.destroy_shader_module(vertex_module, None);
            self.app.ash_device.destroy_shader_module(fragment_module, None);
        }
    }

    fn destroy_pipeline(&mut self) {
        self.pipeline_layout.destroy(&mut self.app);
        self.app.destroy_graphics_pipeline(&mut self.pipeline);
    }

    fn compile_shader(&self,
        shaders_compiler: &mut ShadersCompiler,
        kind: shaderc::ShaderKind,
        path: &str
    ) -> vk::ShaderModule {
        let binary_code = shaders_compiler.compile(path, kind);

        let mut module_info = vk::ShaderModuleCreateInfo::default();
        module_info.p_code = binary_code.as_ptr() as _;
        module_info.code_size = binary_code.len() as _;

        return unsafe {
            self.app.ash_device.create_shader_module(&module_info, None).expect("Failed to create shader module!")
        };
    }
}
