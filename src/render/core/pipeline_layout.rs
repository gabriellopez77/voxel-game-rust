use ash::vk::{self, Handle};

use super::{DescriptorSet, VulkanApp, vkutl};


pub struct PipelineLayout {
    layout: vk::PipelineLayout,

    pub descriptors_layout: [vk::DescriptorSetLayout; vkutl::MAX_DESCRIPTORS_BINDING_COUNT],
    descriptors_sets: [[vk::DescriptorSet; vkutl::MAX_DESCRIPTORS_BINDING_COUNT]; vkutl::FRAMES_COUNT],

    pub descriptors_count: u32,

    is_shared: bool,
}

impl Clone for PipelineLayout {
    fn clone(&self) -> PipelineLayout {
        PipelineLayout {
            layout: self.layout,

            descriptors_layout: self.descriptors_layout,
            descriptors_sets: self.descriptors_sets,

            descriptors_count: self.descriptors_count,

            is_shared: true,
        }
    }
}

impl PipelineLayout {
    pub fn new() -> Self {
        Self {
            layout: vk::PipelineLayout::null(),

            descriptors_layout: [vk::DescriptorSetLayout::null(); vkutl::MAX_DESCRIPTORS_BINDING_COUNT],
            descriptors_sets: [[vk::DescriptorSet::null(); vkutl::MAX_DESCRIPTORS_BINDING_COUNT]; vkutl::FRAMES_COUNT],

            descriptors_count: 0,

            is_shared: false,
        }
    }

    pub fn create(&mut self, app: &VulkanApp, shared: vk::PipelineLayout) {
        if !shared.is_null() {
            self.layout = shared;
            self.is_shared = true;
            return;
        }

        // every pipelines layouts support push constants
        let push_constant_range = [vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
            offset: 0,
            size: vkutl::MAX_PUSH_CONSTANT_SIZE as u32
        }];

        let create_info = &vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&self.descriptors_layout[0..self.descriptors_count as usize])
            .push_constant_ranges(&push_constant_range);

        self.layout = unsafe {
            app.ash_device.create_pipeline_layout(&create_info, None).expect("Failed to create pipeline layout!")
        };
    }

    pub fn destroy(&mut self, app: &mut VulkanApp) {
        if !self.is_shared {
            app.destroy_pipeline_layout(&mut self.layout);
        }
    }

    //pub fn is_same(&self, descriptor_layouts: &[vk::DescriptorSetLayout; vkutl::MAX_DESCRIPTORS_BINDING_COUNT]) -> bool {
    //    for i in 0..vkutl::MAX_DESCRIPTORS_BINDING_COUNT {
    //        if self.descriptors_layout[i] != descriptor_layouts[i] {
    //            return false
    //        }
    //    }

    //    return true;
    //}

    pub fn get_layout(&self) -> vk::PipelineLayout { self.layout }

    pub fn get_descriptors_sets(&self, frame_index: usize) -> &[vk::DescriptorSet; vkutl::MAX_DESCRIPTORS_BINDING_COUNT] {
        &self.descriptors_sets[frame_index]
    }

    pub fn add_descriptor(&mut self, descriptor: &DescriptorSet) -> &mut Self {
        let index = self.descriptors_count as usize;

        debug_assert!(index <= vkutl::MAX_DESCRIPTORS_BINDING_COUNT, "max descriptors count is: {}", vkutl::MAX_DESCRIPTORS_BINDING_COUNT);
        self.descriptors_layout[index] = descriptor.get_layout();

        for i in 0..vkutl::FRAMES_COUNT {
            self.descriptors_sets[i][index] = descriptor.descriptor_set[i];
        }

        self.descriptors_count += 1;

        self
    }
}
