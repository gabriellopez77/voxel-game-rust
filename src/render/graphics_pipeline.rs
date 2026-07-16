use ash::vk;
use crate::render::PipelineLayout;
use crate::resources::ShadersCompiler;

use super::vulkan_app::VulkanApp;
use super::pipeline_settings::PipelineSettings;


pub struct GraphicsPipeline {
    pub pipeline_layout: PipelineLayout,
    pipeline: vk::Pipeline,
}

impl GraphicsPipeline {
    pub fn create(app: &VulkanApp, compiler: &mut ShadersCompiler, settings: PipelineSettings) -> Self {
        const DYNAMIC_STATES: [vk::DynamicState; 2] = [ vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR ];

        let vertex_module = Self::compile_shader(app, compiler, shaderc::ShaderKind::Vertex, &settings.vertex_shader_path);
        let fragment_module = Self::compile_shader(app, compiler, shaderc::ShaderKind::Fragment, &settings.fragment_shader_path);

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


        let bindings = settings.get_bindings();
        let attributes = settings.get_attributes();

        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&attributes);

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(settings.topology)
            .primitive_restart_enable(false);

        let mut viewport_state = vk::PipelineViewportStateCreateInfo::default();
        viewport_state.viewport_count = 1;
        viewport_state.p_viewports = std::ptr::null();   // viewport is dynamic
        viewport_state.scissor_count = 1;
        viewport_state.p_scissors = std::ptr::null();   // scissor is dynamic

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(settings.cull_mode)
            .front_face(vk::FrontFace::CLOCKWISE);

        let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let color_blend_attachment = [
            vk::PipelineColorBlendAttachmentState::default()
                .color_write_mask(vk::ColorComponentFlags::RGBA)
                .blend_enable(settings.enable_blend)
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
            .depth_test_enable(settings.enable_depth_test)
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
                .layout(settings.pipeline_layout.get_layout())
                .render_pass(app.render_pass)
                .subpass(0)
                .base_pipeline_handle(vk::Pipeline::null())
                .base_pipeline_index(-1)
        ];

        let pipeline = unsafe {
            app.ash_device.create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_info, None)
                .expect("Failed to create graphics pipeline!")[0]
        };

        // destroy shaders modules
        unsafe {
            app.ash_device.destroy_shader_module(vertex_module, None);
            app.ash_device.destroy_shader_module(fragment_module, None);
        }

        Self {
            pipeline_layout: settings.pipeline_layout,
            pipeline,
        }
    }

    pub fn get_pipeline(&self) -> vk::Pipeline {
        self.pipeline
    }

    fn compile_shader(app: &VulkanApp, compiler: &mut ShadersCompiler, kind: shaderc::ShaderKind, path: &str) -> vk::ShaderModule {
        let binary_code = compiler.compile(path, kind);

        let mut module_info = vk::ShaderModuleCreateInfo::default();
        module_info.p_code = binary_code.as_ptr() as _;
        module_info.code_size = binary_code.len() as _;

        return unsafe {
            app.ash_device.create_shader_module(&module_info, None)
                .expect("Failed to create shader module!")
        };
    }
}
