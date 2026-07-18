use ash::vk;
use std::ffi::c_char;
use vk_mem::Alloc;
use super::vulkan_app::VulkanApp;


// core
pub const V_SYNC: bool = true;
pub const VALIDATION_LAYERS_ENABLED: bool = true;
pub const FRAMES_COUNT: usize = 2;
pub const SWAPCHAIN_IMAGES_COUNT: usize = 3;

// limits
pub const MAX_VERTEX_ATTRIBUTES_COUNT: usize = 10;
pub const MAX_VERTEX_BINDING_COUNT: usize = 2;
pub const MAX_DESCRIPTORS_BINDING_COUNT: usize = 2;
pub const MAX_PUSH_CONSTANT_SIZE: usize = 128;

// validate layers required by this application
pub const VALIDATE_LAYER_NAMES: [*const c_char; 2] = [
    c"VK_LAYER_KHRONOS_validation".as_ptr() as _,
    c"VK_LAYER_KHRONOS_synchronization2".as_ptr() as _,
];

// device extensions required by this application
pub const REQUIRED_DEVICE_EXTENSIONS: [*const c_char; 1] = [
    vk::KHR_SWAPCHAIN_NAME.as_ptr()
];

pub fn create_image_view(app: &VulkanApp, image: vk::Image, format: vk::Format,
                         aspect: vk::ImageAspectFlags) -> vk::ImageView {
    let mut create_info = vk::ImageViewCreateInfo::default();
    create_info.image = image;
    create_info.view_type = vk::ImageViewType::TYPE_2D;
    create_info.format = format;
    create_info.subresource_range.aspect_mask = aspect;
    create_info.subresource_range.base_mip_level = 0;
    create_info.subresource_range.level_count = 1;
    create_info.subresource_range.base_array_layer = 0;
    create_info.subresource_range.layer_count = 1;

    return unsafe {
        app.ash_device.create_image_view(&create_info, None).expect("Failed to create image views!")
    };
}

pub fn create_image(app: &VulkanApp, width: u32, height: u32, format: vk::Format,
                    usage: vk::ImageUsageFlags, dedicated_memory: bool) -> (vk::Image, vk_mem::Allocation) {
    let create_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .extent(vk::Extent3D{ width, height, depth: 1 })
        .mip_levels(1)
        .array_layers(1)
        .format(format)
        .tiling(vk::ImageTiling::OPTIMAL)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .samples(vk::SampleCountFlags::TYPE_1);

    let mut allocation_info = vk_mem::AllocationCreateInfo::default();
    if dedicated_memory {
        allocation_info.flags = vk_mem::AllocationCreateFlags::DEDICATED_MEMORY;
    }
    allocation_info.usage = vk_mem::MemoryUsage::Auto;
    allocation_info.preferred_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;

    return unsafe {
        app.vma_allocator.create_image(&create_info, &allocation_info).expect("Failed to create vma allocator!")
    };
}

pub fn create_buffer(app: &VulkanApp, size: u64, usage: vk::BufferUsageFlags,
                     allocation_info: &vk_mem::AllocationCreateInfo, concurrent: bool) -> (vk::Buffer, vk_mem::Allocation) {
    let families = [ app.families_indices_cache.transfer.unwrap(), app.families_indices_cache.graphics.unwrap() ];

    let mut buffer_info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    if concurrent && families[0] != families[1] {
        buffer_info.sharing_mode = vk::SharingMode::CONCURRENT;
        buffer_info.p_queue_family_indices = families.as_ptr();
        buffer_info.queue_family_index_count = families.len() as u32;
    }

    return unsafe {
        app.vma_allocator.create_buffer(&buffer_info, &allocation_info).expect("Failed to create buffer!")
    };
}

pub fn copy_data_to_staging_buffer(app: &VulkanApp, offset: usize, size: usize, data: *const u8, allocation: &mut vk_mem::Allocation,
                                   keep_maped: bool) -> *mut u8 {
    unsafe {
        let mapped_mem_ptr = app.vma_allocator.map_memory(allocation).expect("Failed to map memory!");

        // copy data to staging buffer
        std::ptr::copy_nonoverlapping(data, mapped_mem_ptr.byte_add(offset), size);

        if !keep_maped {
            app.vma_allocator.unmap_memory(allocation);
        }

        return if keep_maped { mapped_mem_ptr } else { std::ptr::null_mut() };
    }
}

pub fn copy_buffer_async(app: &VulkanApp, src: vk::Buffer, dst: vk::Buffer, size: u64, src_offset: u64, dst_offset: u64) {
    let copy_region = vk::BufferCopy::default()
        .size(size)
        .src_offset(src_offset)
        .dst_offset(dst_offset);

    unsafe {
        app.ash_device.cmd_copy_buffer(app.get_current_transfer_command_buffer(), src, dst, &[copy_region]);
    }
}

pub unsafe fn begin_single_time_command(app: &VulkanApp, command_pool: vk::CommandPool) -> vk::CommandBuffer {
    let alloc_info = vk::CommandBufferAllocateInfo::default()
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1)
        .command_pool(command_pool);

    let command_buffer = unsafe {
        app.ash_device.allocate_command_buffers(&alloc_info).expect("Failed to allocate command buffer!")
    };

    let begin_info= vk::CommandBufferBeginInfo::default()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    unsafe { app.ash_device.begin_command_buffer(command_buffer[0], &begin_info).unwrap() };

    return command_buffer[0];
}

pub unsafe fn end_single_time_command(app: &VulkanApp, cmd: vk::CommandBuffer, cmd_pool: vk::CommandPool, queue: vk::Queue) {
    unsafe { app.ash_device.end_command_buffer(cmd).unwrap() };

    let command_buffer = [cmd];

    let submit_info = vk::SubmitInfo::default()
        .command_buffers(&command_buffer);

    unsafe {
        app.ash_device.queue_submit(queue, &[submit_info], vk::Fence::null()).expect("Failed to submit command buffer!");
        app.ash_device.queue_wait_idle(queue).unwrap();

        app.ash_device.free_command_buffers(cmd_pool, &command_buffer);
    }
}

pub fn transition_image_layout(app: &VulkanApp, image: vk::Image, old_layout: vk::ImageLayout, new_layout: vk::ImageLayout) {
    let mut barrier = vk::ImageMemoryBarrier::default()
        .old_layout(old_layout)
        .new_layout(new_layout)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(
            vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1
            }
        );

    let src_stage: vk::PipelineStageFlags;
    let dst_stage: vk::PipelineStageFlags;

    if old_layout == vk::ImageLayout::UNDEFINED && new_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL {
        barrier.src_access_mask = vk::AccessFlags::NONE;
        barrier.dst_access_mask = vk::AccessFlags::TRANSFER_WRITE;

        src_stage = vk::PipelineStageFlags::TOP_OF_PIPE;
        dst_stage = vk::PipelineStageFlags::TRANSFER;
    }
    else if old_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL && new_layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL {
        barrier.src_access_mask = vk::AccessFlags::TRANSFER_WRITE;
        barrier.dst_access_mask = vk::AccessFlags::SHADER_READ;

        src_stage = vk::PipelineStageFlags::TRANSFER;
        dst_stage = vk::PipelineStageFlags::FRAGMENT_SHADER;
    }
    else {
        panic!("Unsupported layout transition!")
    }

    unsafe {
        let command_buffer = begin_single_time_command(app, app.graphics_command_pool);

        app.ash_device.cmd_pipeline_barrier(
            command_buffer,
            src_stage, dst_stage,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[barrier]
        );

        end_single_time_command(app, command_buffer, app.graphics_command_pool, app.graphics_queue);
    };
}

pub fn copy_buffer_to_image(app: &VulkanApp, width: u32, height: u32, buffer: vk::Buffer, image: vk::Image) {
    let region = vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1
            }
        )
        .image_extent(
            vk::Extent3D {
                width,
                height,
                depth: 1
            }
        );

    unsafe {
        let command_buffer = begin_single_time_command(app, app.graphics_command_pool);

        app.ash_device.cmd_copy_buffer_to_image(
            command_buffer,
            buffer, image, vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[region]
        );

        end_single_time_command(app, command_buffer, app.graphics_command_pool, app.graphics_queue);
    }
}
