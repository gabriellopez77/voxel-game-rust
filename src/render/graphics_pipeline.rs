use ash::vk;
use crate::render::{GlobalRenderer, PipelineLayout};

use super::vulkan_app::VulkanApp;
use super::pipeline_settings::PipelineSettings;


pub struct GraphicsPipeline {
    pub pipeline_layout: PipelineLayout,
    pipeline: vk::Pipeline,
}

impl GraphicsPipeline {
    pub fn create(app: &VulkanApp, mut settings: PipelineSettings, global_render: &mut GlobalRenderer) -> Self {
        const DYNAMIC_STATES: [vk::DynamicState; 2] = [ vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR ];
        let shader_stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(settings.vertex_shader_module)
                .name(c"main"),

            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(settings.fragment_shader_module)
                .name(c"main")
        ];

        global_render.create_pipeline_layout(&mut settings.pipeline_layout);


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
            app.ash_device.destroy_shader_module(settings.vertex_shader_module, None);
            app.ash_device.destroy_shader_module(settings.fragment_shader_module, None);
        }

        Self {
            pipeline_layout: settings.pipeline_layout,
            pipeline,
        }
    }

    pub fn bind(&self, app: &VulkanApp) {
        let command_buffer = app.get_current_command_buffer();

        //println!("OPA");
        unsafe {
            // bind all descritpor sets used by this pipeline
            if self.pipeline_layout.use_descriptor_sets() {
            //let now = std::time::Instant::now();
                app.ash_device.cmd_bind_descriptor_sets(command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.pipeline_layout.get_layout(),
                    0,
                    &self.pipeline_layout.descriptors_sets[app.frame_index][0..self.pipeline_layout.descriptors_count as usize],
                    &[]
                );
            //println!("{}", now.elapsed().as_micros());
            }

            app.ash_device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
        }
    }

    pub fn get_pipeline(&self) -> vk::Pipeline {
        self.pipeline
    }
}
