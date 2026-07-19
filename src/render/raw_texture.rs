use ash::vk;
use super::{vkutl, vulkan_app::VulkanApp};


pub struct RawTexture {
    image: vk::Image,
    image_allocation: vk_mem::Allocation,

    pub image_view: vk::ImageView,
    pub sampler: vk::Sampler,

    pub inxeding_idx: u8,
}

impl RawTexture {
    pub fn new() -> Self {
        Self {
            image: vk::Image::null(),
            image_allocation: unsafe { std::mem::zeroed() },

            image_view: vk::ImageView::null(),
            sampler: vk::Sampler::null(),

            inxeding_idx: 0,
        }
    }

    pub fn create(&mut self, app: &mut VulkanApp,  width: u32, height: u32, data: &[u8], filter: vk::Filter, repeat_mode: vk::SamplerAddressMode) {
        let mut allocation_info = vk_mem::AllocationCreateInfo::default();
        allocation_info.usage = vk_mem::MemoryUsage::Auto;
        allocation_info.preferred_flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
        allocation_info.flags = vk_mem::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE | vk_mem::AllocationCreateFlags::MAPPED;

        let (mut staging_buffer, mut staging_allocation) = vkutl::create_buffer(
            app,
            data.len() as _,
            vk::BufferUsageFlags::TRANSFER_SRC,
            &allocation_info, false
        );

        unsafe {
            let mapped_mem_ptr = app.vma_allocator.map_memory(&mut staging_allocation).expect("Failed to map memory!");

            // copy data to staging buffer
            std::ptr::copy_nonoverlapping(data.as_ptr() as _, mapped_mem_ptr, data.len());

            app.vma_allocator.unmap_memory(&mut staging_allocation);
        }

        // create image in vram
        (self.image, self.image_allocation) = vkutl::create_image(
            app,
            width, height,
            vk::Format::R8G8B8A8_UNORM,
            vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            false
        );


        unsafe {
            vkutl::transition_image_layout(app, self.image, vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL);

            vkutl::copy_buffer_to_image(app, width, height, staging_buffer, self.image);

            vkutl::transition_image_layout(app, self.image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        }

        self.image_view = vkutl::create_image_view(app, self.image, vk::Format::R8G8B8A8_UNORM, vk::ImageAspectFlags::COLOR);

        self.create_sampler(app, filter, repeat_mode);

        // destroy staging buffer
        app.destroy_buffer(&mut staging_buffer, &mut staging_allocation, &mut std::ptr::null_mut());
    }

    pub fn destroy(&mut self, app: &mut VulkanApp) {
        app.destroy_image(&mut self.image, &mut self.image_allocation, &mut self.image_view, &mut self.sampler);
    }

    fn create_sampler(&mut self, app: &VulkanApp, filter: vk::Filter, repeat_mode: vk::SamplerAddressMode) {
        let sampler_info = vk::SamplerCreateInfo::default()
            .mag_filter(filter)
            .min_filter(filter)
            .address_mode_u(repeat_mode)
            .address_mode_v(repeat_mode)
            .address_mode_w(repeat_mode)
            .anisotropy_enable(false)
            .max_anisotropy(1.0)
            .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(vk::CompareOp::ALWAYS)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR);


        self.sampler = unsafe {
            app.ash_device.create_sampler(&sampler_info, None).expect("Failed to create texture sampler!")
        };
    }
}
