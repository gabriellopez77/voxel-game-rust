use ash::vk;
use std::ffi::c_char;
use vk_mem::Alloc;
use super::vulkan_app::VulkanApp;


// core
pub const V_SYNC: bool = false;
pub const VALIDATION_LAYERS_ENABLED: bool = false;
pub const FRAMES_COUNT: usize = 2;
pub const SWAPCHAIN_IMAGES_COUNT: usize = 3;

// limits
pub const MAX_VERTEX_ATTRIBUTES_COUNT: usize = 10;
pub const MAX_VERTEX_BINDING_COUNT: usize = 2;
pub const MAX_BUFFERS_REQUIRED_TO_DRAW_COUNT: usize = 3;
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

pub fn null_allocation() -> vk_mem::Allocation { unsafe { std::mem::zeroed() } }

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

pub fn create_buffer(app: &VulkanApp, size: usize, usage: vk::BufferUsageFlags, alloc_info: &vk_mem::AllocationCreateInfo,
                     concurrent: bool) -> (vk::Buffer, vk_mem::Allocation) {
    let families = [ app.transfer_queue_index, app.graphics_queue_index ];

    let mut buffer_info = vk::BufferCreateInfo::default()
        .size(size as u64)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    if concurrent && families[0] != families[1] {
        buffer_info.sharing_mode = vk::SharingMode::CONCURRENT;
        buffer_info.p_queue_family_indices = families.as_ptr();
        buffer_info.queue_family_index_count = families.len() as u32;
    }

    return unsafe {
        app.vma_allocator.create_buffer(&buffer_info, &alloc_info).expect("Failed to create buffer!")
    };
}

pub fn copy_buffer_async(app: &VulkanApp, src: vk::Buffer, dst: vk::Buffer, size: usize, src_offset: usize, dst_offset: usize) {
    let copy_region = vk::BufferCopy::default()
        .size(size as u64)
        .src_offset(src_offset as u64)
        .dst_offset(dst_offset as u64);

    unsafe {
        app.ash_device.cmd_copy_buffer(app.get_transfer_cmd(), src, dst, &[copy_region]);
    }
}

pub fn transition_image_layout(app: &VulkanApp, image: vk::Image, old_layout: vk::ImageLayout, new_layout: vk::ImageLayout) {
    let mut barrier = vk::ImageMemoryBarrier::default()
        .old_layout(old_layout)
        .new_layout(new_layout)
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
    let cmd: vk::CommandBuffer;

    if old_layout == vk::ImageLayout::UNDEFINED && new_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL {
        barrier.src_access_mask = vk::AccessFlags::NONE;
        barrier.dst_access_mask = vk::AccessFlags::TRANSFER_WRITE;

        src_stage = vk::PipelineStageFlags::TOP_OF_PIPE;
        dst_stage = vk::PipelineStageFlags::TRANSFER;

        barrier.src_queue_family_index = vk::QUEUE_FAMILY_IGNORED;
        barrier.dst_queue_family_index = vk::QUEUE_FAMILY_IGNORED;

        cmd = app.get_transfer_cmd();
    }
    else if old_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL && new_layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL {
        barrier.src_access_mask = vk::AccessFlags::TRANSFER_WRITE;
        barrier.dst_access_mask = vk::AccessFlags::SHADER_READ;

        src_stage = vk::PipelineStageFlags::TRANSFER;
        dst_stage = vk::PipelineStageFlags::FRAGMENT_SHADER;

        barrier.src_queue_family_index = app.transfer_queue_index;
        barrier.dst_queue_family_index = app.graphics_queue_index;

        cmd = app.get_graphics_cmd();
    }
    else {
        panic!("Unsupported layout transition!")
    }

    unsafe {
        app.ash_device.cmd_pipeline_barrier(cmd,
            src_stage, dst_stage,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[barrier]
        );
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
        .image_extent(vk::Extent3D { width, height, depth: 1 });

    unsafe {
        app.ash_device.cmd_copy_buffer_to_image(
            app.get_transfer_cmd(),
            buffer, image, vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[region]
        );

        let release_barrier = vk::ImageMemoryBarrier::default()
            .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .src_queue_family_index(app.transfer_queue_index)
            .dst_queue_family_index(app.graphics_queue_index)
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

        app.ash_device.cmd_pipeline_barrier(app.get_transfer_cmd(),
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[release_barrier]
        );

    }
}
