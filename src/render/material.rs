use std::{cell::RefCell, rc::Rc};

use ash::vk;

use crate::{math::Vec4, render::core::vkutl, utils::SafePtrMut};
use super::core::{VulkanApp, GraphicsPipeline};


#[derive(Clone, Copy)]
pub struct VertexAttribInfo {
    bindings_info: [vk::VertexInputBindingDescription; vkutl::MAX_VERTEX_BINDING_COUNT],
    attributes_info: [vk::VertexInputAttributeDescription; vkutl::MAX_VERTEX_ATTRIBUTES_COUNT],

    current_location: u32,
    current_binding: u32,
    current_attributes_index: u32
}

impl VertexAttribInfo {
    pub fn new() -> Self {
        Self {
            bindings_info: [vk::VertexInputBindingDescription::default(); vkutl::MAX_VERTEX_BINDING_COUNT],
            attributes_info: [vk::VertexInputAttributeDescription::default(); vkutl::MAX_VERTEX_ATTRIBUTES_COUNT],

            current_location: 0,
            current_binding: 0,
            current_attributes_index: 0,
        }
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

    pub pipeline: Rc<RefCell<GraphicsPipeline>>,

    material_type: MaterialType,

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
    pub fn new(app: SafePtrMut<VulkanApp>, pipeline: Rc<RefCell<GraphicsPipeline>>, material_type: MaterialType) -> Self {
        Self {
            app,

            pipeline: pipeline,

            material_type: material_type,

            vertex_attributes_info: VertexAttribInfo::new(),
            cull_mode: vk::CullModeFlags::FRONT,
            blend: false,
            depth_test: true,
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            line_width: 1.0,

            modified: true,
        }
    }

    pub fn destroy(&mut self) {

    }

    pub fn get_pipeline_info(&mut self) {

    }

    pub fn set_attributes_info(&mut self, info: VertexAttribInfo) {
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

    pub fn get_type(&self) -> MaterialType { self.material_type }
}
