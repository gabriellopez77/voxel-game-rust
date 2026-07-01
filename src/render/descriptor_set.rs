use ash::vk;
use crate::render::vulkan_app::GarbageType;
use super::{raw_texture::RawTexture, ubo::Ubo, vulkan_app::VulkanApp};
use super::vkutl;


enum LayoutInfos {
    Image(vk::Sampler, vk::ImageView),
    IndexingImages(Vec<(vk::Sampler, vk::ImageView)>),
    Ubo([vk::Buffer; vkutl::FRAMES_COUNT]),
}

struct LayoutInfo {
    layout_info: LayoutInfos,

    binding: u32,
    descriptor_type: vk::DescriptorType,
    shader_stage: vk::ShaderStageFlags,
}

pub struct DescriptorSet {
    layouts: Vec<LayoutInfo>,

    pub descriptor_set: [vk::DescriptorSet; vkutl::FRAMES_COUNT],
    descriptor_set_layout: vk::DescriptorSetLayout,
}

impl DescriptorSet {
    pub fn new() -> Self {
        Self {
            layouts: Vec::new(),

            descriptor_set: [vk::DescriptorSet::null(); vkutl::FRAMES_COUNT],
            descriptor_set_layout: vk::DescriptorSetLayout::null(),
        }
    }

    pub fn get_layout(&self) -> vk::DescriptorSetLayout {
        self.descriptor_set_layout
    }

    pub fn create(&mut self, app: &VulkanApp) {
        // configure and create descriptor set layout
        let mut layouts_bindings: Vec<vk::DescriptorSetLayoutBinding> = Vec::with_capacity(self.layouts.len());

        for layout in &self.layouts {
            let mut layout_binding = vk::DescriptorSetLayoutBinding::default();
            layout_binding.binding = layout.binding;
            layout_binding.descriptor_type = layout.descriptor_type;
            layout_binding.stage_flags = layout.shader_stage;

            if let LayoutInfos::IndexingImages(ref textures) = layout.layout_info {
                layout_binding.descriptor_count = textures.len() as u32;
            }
            else {
                layout_binding.descriptor_count = 1;
            }

            layouts_bindings.push(layout_binding);
        }

        let layout_info = vk::DescriptorSetLayoutCreateInfo::default()
            .bindings(&layouts_bindings);

        self.descriptor_set_layout = unsafe {
            app.ash_device.create_descriptor_set_layout(&layout_info, None).expect("Failed to create descriptor set layout!")
        };


        // create descriptor sets
        let set_layouts = [self.descriptor_set_layout; vkutl::FRAMES_COUNT];

        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(app.descriptor_pool)
            .set_layouts(&set_layouts);

        let descriptor_sets = unsafe {
            app.ash_device.allocate_descriptor_sets(&alloc_info).expect("Failed to allocate descriptor set!")
        };

        for i in 0..vkutl::FRAMES_COUNT {
            self.descriptor_set[i] = descriptor_sets[i];
        }



        // update descriptor set
        let mut descriptor_writes: Vec<vk::WriteDescriptorSet> = Vec::with_capacity(self.layouts.len() * vkutl::FRAMES_COUNT);
        let mut ubo_infos = vec![vk::DescriptorBufferInfo::default(); self.layouts.len()];
        let mut images_infos = vec![vk::DescriptorImageInfo::default(); self.layouts.len()];
        let mut indexing_images_info: Vec<Vec<vk::DescriptorImageInfo>> = vec![Vec::new(); self.layouts.len()];

        for i in 0..vkutl::FRAMES_COUNT {
            let mut ubos_index = 0;
            let mut images_index = 0;
            let mut indexing_images_index = 0;

            for info in &self.layouts {
                let mut writer = vk::WriteDescriptorSet::default()
                    .dst_set(self.descriptor_set[i])
                    .dst_binding(info.binding)
                    .descriptor_type(info.descriptor_type)
                    .descriptor_count(1);

                if let LayoutInfos::Ubo(buffers) = info.layout_info {
                    let buffer_info = vk::DescriptorBufferInfo::default()
                        .offset(0)
                        .buffer(buffers[i])
                        .range(vk::WHOLE_SIZE);

                    ubo_infos[ubos_index] = buffer_info;
                    writer.p_buffer_info = &ubo_infos[ubos_index];

                    ubos_index += 1;
                }
                else if let LayoutInfos::Image(sampler, image_view) = info.layout_info {
                    let image_info = vk::DescriptorImageInfo::default()
                        .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                        .image_view(image_view)
                        .sampler(sampler);

                    images_infos[images_index] = image_info;
                    writer.p_image_info = &images_infos[images_index];

                    images_index += 1;
                }
                else if let LayoutInfos::IndexingImages(ref images) = info.layout_info {
                    indexing_images_info[indexing_images_index] = images.iter().map(|info|
                        vk::DescriptorImageInfo::default()
                            .sampler(info.0)
                            .image_view(info.1)
                            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                    ).collect();


                    writer.p_image_info = indexing_images_info[indexing_images_index].as_ptr();
                    writer.descriptor_count = indexing_images_info[indexing_images_index].len() as u32;

                    indexing_images_index += 1;
                }

                descriptor_writes.push(writer);
            }

            unsafe { app.ash_device.update_descriptor_sets(&descriptor_writes, &[]) }

            descriptor_writes.clear();
        }
    }

    pub fn destroy(&mut self, app: &mut VulkanApp) {
        app.add_to_bargabe_list(GarbageType::DescriptorSetLayout(self.descriptor_set_layout));
    }

    pub fn add_indexing_textures(&mut self, binding: u32, stage: vk::ShaderStageFlags, textures: &mut [&mut RawTexture]) -> &mut Self {
        for i in 0..textures.len() {
            textures[i].inxeding_idx = i as u8;
        }

        let layout = LayoutInfo {
            layout_info: LayoutInfos::IndexingImages(textures.iter().map(|tex| (tex.sampler, tex.image_view)).collect()),

            binding,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            shader_stage: stage,
        };

        self.layouts.push(layout);

        self
    }

    pub fn add_texture(&mut self, binding: u32, stage: vk::ShaderStageFlags, texture: &RawTexture) -> &mut Self {
        let layout = LayoutInfo {
            layout_info: LayoutInfos::Image(texture.sampler, texture.image_view),

            binding,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            shader_stage: stage,
        };

        self.layouts.push(layout);

        self
    }

    pub fn add_ubo(&mut self, binding: u32, stage: vk::ShaderStageFlags, ubo: &Ubo) -> &mut Self {
        let layout = LayoutInfo {
            layout_info: LayoutInfos::Ubo(ubo.buffer.get_all_buffers()),

            binding,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            shader_stage: stage,
        };

        self.layouts.push(layout);

        self
    }
}
