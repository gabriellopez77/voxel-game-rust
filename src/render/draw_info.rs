use ash::vk;

use crate::render::core::vkutl;


pub struct DrawInfo {
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
    pub descriptors_sets: [vk::DescriptorSet; vkutl::MAX_DESCRIPTORS_BINDING_COUNT],
    pub descriptors_count: u32,

    pub buffers: [vk::Buffer; vkutl::MAX_VERTEX_BINDING_COUNT + 1],

    pub index_count: u32,
    pub instance_count: u32,

    pub push_constant_idx: i32,
}

impl Ord for DrawInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.pipeline != other.pipeline {
            return self.pipeline.cmp(&other.pipeline)
        }

        //if self.pipeline_layout != other.pipeline_layout {
        //}
        return self.pipeline_layout.cmp(&other.pipeline_layout)

        //if self.vertices_buffer[0] != other.vertices_buffer[0] ||
        //   self.vertices_buffer[1] != other.vertices_buffer[1] {
        //    return self.vertices_buffer.cmp(&other.vertices_buffer)
        //}

        //return self.index_buffer.cmp(&other.index_buffer)
    }
}

impl PartialOrd for DrawInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DrawInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pipeline == other.pipeline && self.buffers == other.buffers
    }
}

impl Eq for DrawInfo {}
