use ash::vk;
use ash::vk::Handle;
use crate::math::Vec4;
use super::PipelineLayout;
use super::descriptor_set::DescriptorSet;
use super::vkutl;


pub struct PipelineSettings {
    pub vertex_shader_path: String,
    pub fragment_shader_path: String,

    //pub dynamic_states: Option<&'a [vk::DynamicState]>,

    pub cull_mode: vk::CullModeFlags,
    pub enable_blend: bool,
    pub enable_depth_test: bool,
    pub topology: vk::PrimitiveTopology,
    pub line_width: f32,

    bindings_info: [vk::VertexInputBindingDescription; vkutl::MAX_VERTEX_BINDING_COUNT],
    attributes_info: [vk::VertexInputAttributeDescription; vkutl::MAX_VERTEX_ATTRIBUTES_COUNT],

    pub pipeline_layout: PipelineLayout,

    current_location: u32,
    current_binding: u32,
    current_attributes_index: u32
}

impl PipelineSettings {
    pub fn new(name: &'static str) -> Self {
        let vert_full_path = format!("{name}.vsh");
        let frag_full_path = format!("{name}.fsh");

        Self {
            vertex_shader_path: vert_full_path,
            fragment_shader_path: frag_full_path,

            //dynamic_states: None,

            cull_mode: vk::CullModeFlags::FRONT,
            enable_blend: false,
            enable_depth_test: true,
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            line_width: 1.0,

            bindings_info: [vk::VertexInputBindingDescription::default(); vkutl::MAX_VERTEX_BINDING_COUNT],
            attributes_info: [vk::VertexInputAttributeDescription::default(); vkutl::MAX_VERTEX_ATTRIBUTES_COUNT],

            pipeline_layout: PipelineLayout::new(),

            current_location: 0,
            current_binding: 0,
            current_attributes_index: 0,
        }
    }



    pub fn vertex_info(&mut self, stride: usize, is_instance: bool) -> &mut Self {
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

    pub fn add_attrib(&mut self, format: vk::Format, offset: usize) -> &mut Self {
        let index = self.current_attributes_index;
        self.current_attributes_index += 1;

        debug_assert!(self.current_binding <= vkutl::MAX_VERTEX_ATTRIBUTES_COUNT as u32, "max attributes count is: {}", vkutl::MAX_VERTEX_ATTRIBUTES_COUNT);

        self.attributes_info[index as usize] = vk::VertexInputAttributeDescription{
            location: self.current_location,
            binding: self.current_binding - 1,
            format,
            offset: offset as u32
        };

        self.current_location += 1;
        self
    }

    pub fn add_attrib_matrix(&mut self, offset: usize) -> &mut Self {
        self.add_attrib(vk::Format::R32G32B32A32_SFLOAT, offset + 0 * size_of::<Vec4>());
        self.add_attrib(vk::Format::R32G32B32A32_SFLOAT, offset + 1 * size_of::<Vec4>());
        self.add_attrib(vk::Format::R32G32B32A32_SFLOAT, offset + 2 * size_of::<Vec4>());
        self.add_attrib(vk::Format::R32G32B32A32_SFLOAT, offset + 3 * size_of::<Vec4>());

        self
    }

    pub fn get_attributes(&self) -> &[vk::VertexInputAttributeDescription] {
        &self.attributes_info[0..self.current_attributes_index as usize]
    }

    pub fn get_bindings(&self) -> &[vk::VertexInputBindingDescription] {
        &self.bindings_info[0..self.current_binding as usize]
    }

    pub fn add_descriptor_set(&mut self, descriptor_set: &DescriptorSet) {
        debug_assert!(!descriptor_set.get_layout().is_null(), "descriptor set is null");

        self.pipeline_layout.add_descriptor(descriptor_set);
    }

    //pub fn set_dynamic_states(&mut self, states: &'a [vk::DynamicState]) {
    //    self.dynamic_states = Some(states);
    //}
}
