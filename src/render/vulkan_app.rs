use std::collections::HashSet;
use std::ffi::CStr;
use ash::{vk, khr::surface};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use super::swapchain_info::SwapChainInfo;
use super::vkutl;

//struct BufferDuplacteUpdateInfo {
//    buffers: [vk::Buffer; vkutl::FRAMES_COUNT],
//    can_free: [bool; vkutl::FRAMES_COUNT],
//    need_update: [bool; vkutl::FRAMES_COUNT],
//    ranges: [RangeInfo; vkutl::FRAMES_COUNT],
//}
//
//impl BufferDuplacteUpdateInfo {
//    pub fn get_info(&self, frame: usize) -> (vk::Buffer, bool, bool) {
//        (self.buffers[frame], self.can_free[frame], self.need_update[frame])
//    }
//}

#[derive(Clone, Copy)]
enum GarbageType {
    Buffer(vk::Buffer, vk_mem::Allocation, bool),
    Image(vk::Image, vk_mem::Allocation, vk::ImageView, vk::Sampler),
    DescriptorSetLayout(vk::DescriptorSetLayout),
}

pub struct QueueFamilyIndices {
    pub graphics: Option<u32>,
    pub present: Option<u32>,
    pub transfer: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn new() -> Self {
        Self {
            graphics: None,
            present: None,
            transfer: None,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.graphics.is_some() && self.present.is_some() && self.transfer.is_some()
    }
}

pub struct VulkanApp {
    pub ash_entry: ash::Entry,
    pub ash_instance: ash::Instance,
    pub ash_surface: surface::Instance,
    pub ash_device: ash::Device,
    pub ash_swapchain: ash::khr::swapchain::Device,

    pub swapchain_info: SwapChainInfo,

    pub window_surface: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    pub swapchain: vk::SwapchainKHR,
    pub render_pass: vk::RenderPass,
    pub descriptor_pool: vk::DescriptorPool,

    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    pub transfer_queue: vk::Queue,

    pub graphics_command_pool: vk::CommandPool,
    pub transfer_command_pool: vk::CommandPool,

    graphics_command_buffers: [vk::CommandBuffer; vkutl::FRAMES_COUNT],
    transfer_command_buffers: [vk::CommandBuffer; vkutl::FRAMES_COUNT],

    acquire_semaphores: [vk::Semaphore; vkutl::FRAMES_COUNT],
    submit_semaphores: [vk::Semaphore; vkutl::SWAPCHAIN_IMAGES_COUNT],
    frame_fence: [vk::Fence; vkutl::FRAMES_COUNT],
    transfer_semaphores: [vk::Semaphore; vkutl::FRAMES_COUNT],

    pub frame_index: usize,
    image_index: usize,
    frame_count: usize,
    resized: bool,

    // vma
    pub vma_allocator: vk_mem::Allocator,

    // resources garbage
    garbage_list_pool: Vec<Vec<GarbageType>>,
    garbage_lists: Vec<(usize, Vec<GarbageType>)>,
    current_gargabe_list: Vec<GarbageType>,

    // global staging buffer
    //global_staging_buffer: [vk::Buffer; vkutl::FRAMES_COUNT],
    //global_staging_buffer_allocation: [vk_mem::Allocation; vkutl::FRAMES_COUNT],
    //global_staging_buffer_arena: [BufferArena; vkutl::FRAMES_COUNT],

    // buffers updates
    //updates_list: Vec<BufferDuplacteUpdateInfo>,

    // cache
    pub families_indices_cache: QueueFamilyIndices,
}

impl VulkanApp {
    pub fn new() -> Self {
        Self {
            ash_entry: unsafe { ash::Entry::load().expect("error to load vulkan library") },
            ash_instance: unsafe { std::mem::MaybeUninit::uninit().assume_init() }, // SAFETY: we init before using
            ash_surface: unsafe { std::mem::MaybeUninit::uninit().assume_init() }, // SAFETY: we init before using
            ash_device: unsafe { std::mem::MaybeUninit::uninit().assume_init() }, // SAFETY: we init before using
            ash_swapchain: unsafe { std::mem::MaybeUninit::uninit().assume_init() }, // SAFETY: we init before using

            swapchain_info: SwapChainInfo::new(),

            window_surface: vk::SurfaceKHR::null(),
            physical_device: vk::PhysicalDevice::null(),
            swapchain: vk::SwapchainKHR::null(),
            render_pass: vk::RenderPass::null(),
            descriptor_pool: vk::DescriptorPool::null(),

            graphics_queue: vk::Queue::null(),
            present_queue: vk::Queue::null(),
            transfer_queue: vk::Queue::null(),

            graphics_command_pool: vk::CommandPool::null(),
            transfer_command_pool: vk::CommandPool::null(),

            graphics_command_buffers: [vk::CommandBuffer::null(); vkutl::FRAMES_COUNT],
            transfer_command_buffers: [vk::CommandBuffer::null(); vkutl::FRAMES_COUNT],

            acquire_semaphores: [vk::Semaphore::null(); vkutl::FRAMES_COUNT],
            submit_semaphores: [vk::Semaphore::null(); vkutl::SWAPCHAIN_IMAGES_COUNT],
            frame_fence: [vk::Fence::null(); vkutl::FRAMES_COUNT],
            transfer_semaphores: [vk::Semaphore::null(); vkutl::FRAMES_COUNT],

            frame_index: 0,
            image_index: 0,
            frame_count: 0,
            resized: false,

            vma_allocator: unsafe { std::mem::zeroed() }, // SAFETY: we init before using

            garbage_list_pool: Vec::new(),
            garbage_lists: Vec::new(),
            current_gargabe_list: Vec::new(),

            //global_staging_buffer: [vk::Buffer::null(); vkutl::FRAMES_COUNT],
            //global_staging_buffer_allocation: unsafe { std::mem::zeroed() },
            //global_staging_buffer_arena: array::from_fn(|_| BufferArena::new(1 * BufferArena::MB, 1 * BufferArena::KB)),

            families_indices_cache: QueueFamilyIndices::new(),
        }
    }

    //pub fn update_buffer(&mut self, buffers: [vk::Buffer; vkutl::FRAMES_COUNT], data: *const u8, size: usize, offset: usize) {
        //let range = self.global_staging_buffer_arena[self.frame_index].find_range(size as u32).expect("Arena out of memory!");

        //let mut info = BufferDuplacteUpdateInfo {
        //    buffers,
        //    can_free: array::from_fn(|_| false),
        //    need_update: array::from_fn(|_| true),
        //    ranges: array::from_fn(|_| RangeInfo::EMPTY)
        //};

        //info.can_free[self.frame_index] = true;
        //info.need_update[self.frame_index] = false;
        //info.ranges[self.frame_index] = range;

        //self.updates_list.push(info);
    //}

    pub fn get_current_command_buffer(&self) -> vk::CommandBuffer {
        self.graphics_command_buffers[self.frame_index]
    }

    pub fn get_current_transfer_command_buffer(&self) -> vk::CommandBuffer {
        self.transfer_command_buffers[self.frame_index]
    }

    pub fn destroy_buffer(&mut self, buffer: &mut vk::Buffer, allocation: &mut vk_mem::Allocation, mapped_memory: &mut *mut u8) {
        self.current_gargabe_list.push(GarbageType::Buffer(*buffer, *allocation, !mapped_memory.is_null()));

        *buffer = vk::Buffer::null();
        *allocation = unsafe { std::mem::zeroed() };

        if !mapped_memory.is_null() {
            *mapped_memory = std::ptr::null_mut();
        }
    }
    pub fn destroy_descriptor_set_layout(&mut self, layout: &mut vk::DescriptorSetLayout) {
        self.current_gargabe_list.push(GarbageType::DescriptorSetLayout(*layout));

        *layout = vk::DescriptorSetLayout::null();
    }

    pub fn destroy_image(&mut self, image: &mut vk::Image, allocation: &mut vk_mem::Allocation, image_view:
                         &mut vk::ImageView, sampler: &mut vk::Sampler) {
        self.current_gargabe_list.push(GarbageType::Image(*image, *allocation, *image_view, *sampler));

        *image = vk::Image::null();
        *allocation = unsafe { std::mem::zeroed() };
        *image_view = vk::ImageView::null();
        *sampler = vk::Sampler::null();
    }

    fn destroy_garbage(&self, garbage: &mut GarbageType) {
        unsafe {
            match garbage {
                GarbageType::Buffer(buffer, allocation, need_unmap) => {
                    if *need_unmap {
                        self.vma_allocator.unmap_memory(allocation)
                    }

                    self.vma_allocator.destroy_buffer(*buffer, allocation);
                }
                GarbageType::Image(image, allocation, image_view, image_sampler) => {
                    self.ash_device.destroy_image_view(*image_view, None);
                    self.ash_device.destroy_sampler(*image_sampler, None);
                    self.vma_allocator.destroy_image(*image, allocation);
                }
                GarbageType::DescriptorSetLayout(layout) => {
                    self.ash_device.destroy_descriptor_set_layout(*layout, None);
                }
            }
        }
    }

    pub fn start(&mut self, glfw_window: &glfw::PWindow) {
        self.create_instance(); // init ash_instance
        self.ash_surface = surface::Instance::new(&self.ash_entry, &self.ash_instance);
        self.create_window_surface(&glfw_window);

        self.pick_physical_device();
        self.create_logical_device(); // init ash_device
        self.ash_swapchain = ash::khr::swapchain::Device::new(&self.ash_instance, &self.ash_device);

        // save cache
        self.families_indices_cache = self.find_queue_families(self.physical_device);

        let mut allocator_info = vk_mem::AllocatorCreateInfo::new(&self.ash_instance, &self.ash_device, self.physical_device);
        allocator_info.flags = vk_mem::AllocatorCreateFlags::EXTERNALLY_SYNCHRONIZED;

        self.vma_allocator = unsafe {
            vk_mem::Allocator::new(allocator_info).expect("error to init vma allocator")
        };

        self.create_swapchain(glfw_window);
        self.create_render_pass();
        self.create_swapchain_framebuffers();
        self.create_command_pool();
        self.create_command_buffers();
        self.create_sync_objects();
        self.create_descriptor_pool();

        //let mut staging_allocation_info = vk_mem::AllocationCreateInfo::default();
        //staging_allocation_info.usage = vk_mem::MemoryUsage::Auto;
        //staging_allocation_info.preferred_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;
        //staging_allocation_info.flags = vk_mem::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE | vk_mem::AllocationCreateFlags::DEDICATED_MEMORY;

        // create staging buffer
        //for i in 0..vkutl::FRAMES_COUNT {
            //(self.global_staging_buffer[i], self.global_staging_buffer_allocation[i]) = vkutl::create_buffer(
            //    self, (100 * BufferArena::MB) as u64,
            //    vk::BufferUsageFlags::TRANSFER_SRC,
            //    &staging_allocation_info, false
            //);
        //}
    }

    pub fn cleanup(&mut self) {
        unsafe {
            self.ash_device.device_wait_idle().unwrap();

            for garbage in &mut self.current_gargabe_list.clone() {
                self.destroy_garbage(garbage);
            }

            SwapChainInfo::clear(self);

            //for i in 0..vkutl::FRAMES_COUNT {
                //self.vma_allocator.destroy_buffer(
                //    self.global_staging_buffer[i],
                //    &mut self.global_staging_buffer_allocation[i]
                //);
            //}
        }

    }

    pub fn resize(&mut self, mut width: i32, mut height: i32, glfw_instance: &mut glfw::Glfw, glfw_window: &glfw::PWindow) {
        // avoid create a framebuffer with 0 as size
        while width == 0 || height == 0 {
            (width, height) = glfw_window.get_framebuffer_size();
            glfw_instance.wait_events();
        }

        self.resized = true;
    }

    fn create_instance(&mut self) {
        let app_info = vk::ApplicationInfo::default()
            .application_name(c"Vulkan App")
            .engine_name(c"No Engine")
            .api_version(vk::API_VERSION_1_3);


        // configure required extensions
        let mut required_extensions: Vec<*const std::ffi::c_char> = Vec::new();

        let mut glfw_extensions_count: u32 = 0;
        let glfw_extensions = unsafe {
            glfw::ffi::glfwGetRequiredInstanceExtensions(&mut glfw_extensions_count)
        };

        if !glfw_extensions.is_null() {
            for i in 0..glfw_extensions_count {
                required_extensions.push(unsafe { *glfw_extensions.offset(i as isize) });
            }
        }

        if vkutl::VALIDATION_LAYERS_ENABLED {
            required_extensions.push(vk::EXT_DEBUG_UTILS_NAME.as_ptr())
        }

        // create vk instance
        let mut create_info = vk::InstanceCreateInfo::default();
        create_info.p_application_info = &app_info;
        create_info.pp_enabled_extension_names = required_extensions.as_ptr();
        create_info.enabled_extension_count = required_extensions.len() as u32;

        if vkutl::VALIDATION_LAYERS_ENABLED {
            create_info.pp_enabled_layer_names = vkutl::VALIDATE_LAYER_NAMES.as_ptr();
            create_info.enabled_layer_count = vkutl::VALIDATE_LAYER_NAMES.len() as u32;
        }


        self.ash_instance = unsafe {
            self.ash_entry.create_instance(&create_info, None).expect("Failed to create Vulkan instance!")
        };
    }

    fn create_window_surface(&mut self, glfw_window: &glfw::Window)  {
        unsafe {
            self.window_surface = ash_window::create_surface(
                &self.ash_entry,
                &self.ash_instance,
                glfw_window.display_handle().unwrap().as_raw(),
                glfw_window.window_handle().unwrap().as_raw(),
                None
            ).expect("Failed to create window surface!");
        }
    }

    fn pick_physical_device(&mut self) {
        let is_suitable = |physical_device: vk::PhysicalDevice| -> bool  {
            // get physical device supported extensions
            let supported_extensions = unsafe {
                self.ash_instance.enumerate_device_extension_properties(physical_device).unwrap()
            };

            // check if device supports the required extensions by this app
            for extension in vkutl::REQUIRED_DEVICE_EXTENSIONS {
                let mut supported = false;

                for properties in &supported_extensions {
                    if unsafe { CStr::from_ptr(extension) == CStr::from_ptr(properties.extension_name.as_ptr()) } {
                        supported = true;
                    }
                }

                if !supported { return false }
            }

            let indices = self.find_queue_families(physical_device);

            // all requiered extensions is supported, then, check if swap chain support is adequate
            return indices.is_complete() && SwapChainInfo::is_adequade(self, physical_device)
        };



        // get all physical devices
        let physical_devices =  unsafe {
            self.ash_instance.enumerate_physical_devices().expect("Failed to find GPUs with Vulkan support!")
        };

        // check each device for suitability and choice the first one that is suitable
        for device in physical_devices {
            if is_suitable(device) {
                self.physical_device = device;
                return;
            }
        }

        panic!("Failed to find a suitable GPU!");
    }

    fn find_queue_families(&self, physical_device: vk::PhysicalDevice) -> QueueFamilyIndices {
        // get all queue family support for that physical device
        let queues = unsafe {
            self.ash_instance.get_physical_device_queue_family_properties(physical_device)
        };

        let mut indices = QueueFamilyIndices::new();

        // try to get graphics queue
        for i in 0..queues.len() {
            if queues[i].queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                indices.graphics = Some(i as u32);
                break;
            }
        }

        // graphics queue is necessary
        assert!(indices.graphics.is_some());

        // try to get present queue support that are not graphics queue
        for index in 0..queues.len() {
            //if !queues[index].queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            if queues[index].queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                // check if queue supports present
                let support = unsafe {
                    self.ash_surface.get_physical_device_surface_support(physical_device, index as u32, self.window_surface).unwrap()
                };

                if support {
                    indices.present = Some(index as u32);
                    break;
                }
            }
        }

        // try to get transfer queue that are not graphics queue
        for index in 0..queues.len() {
            let flags = queues[index].queue_flags;

            if flags.contains(vk::QueueFlags::TRANSFER) && !flags.contains(vk::QueueFlags::GRAPHICS) {
            //if flags.contains(vk::QueueFlags::TRANSFER) {
                indices.transfer = Some(index as u32);
                break;
            }
        }

        // graphics queue always supports transfer
        if indices.transfer.is_none() {
            indices.transfer = indices.graphics;
        }

        // graphics queue always supports present
        if indices.present.is_none() {
            indices.present = indices.graphics;
        }

        return indices;
    }

    fn create_logical_device(&mut self) {
        // features
        let mut features12 = vk::PhysicalDeviceVulkan12Features::default()
            .descriptor_binding_partially_bound(true)
            .runtime_descriptor_array(true)
            .shader_sampled_image_array_non_uniform_indexing(true);

        // get queue families indices supported by physical device
        let indices = self.find_queue_families(self.physical_device);

        const QUEUE_PRIORITY: [f32; 1] = [1.0];

        // used a HashSet because graphics and present or transfer can have the same index
        let mut queue_families: HashSet<u32> = HashSet::with_capacity(3);
        queue_families.insert(indices.graphics.unwrap());
        queue_families.insert(indices.transfer.unwrap());
        queue_families.insert(indices.present.unwrap());

        let mut queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = Vec::with_capacity(queue_families.len());

        for queue_family_index in queue_families {
            let create_info = vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_family_index)
                .queue_priorities(&QUEUE_PRIORITY);

            queue_create_infos.push(create_info);
        }

        let mut create_info = vk::DeviceCreateInfo::default();
        create_info.p_queue_create_infos = queue_create_infos.as_ptr();
        create_info.queue_create_info_count = queue_create_infos.len() as u32;
        create_info.pp_enabled_extension_names = vkutl::REQUIRED_DEVICE_EXTENSIONS.as_ptr();
        create_info.enabled_extension_count = vkutl::REQUIRED_DEVICE_EXTENSIONS.len() as u32;
        create_info.p_next = (&mut features12) as *mut _ as _;

        self.ash_device = unsafe {
            self.ash_instance.create_device(self.physical_device, &create_info, None).expect("Failed to create logical device!")
        };

        // get queues handler
        self.graphics_queue = unsafe { self.ash_device.get_device_queue(indices.graphics.unwrap(), 0) };
        self.present_queue = unsafe { self.ash_device.get_device_queue(indices.present.unwrap(), 0) };
        self.transfer_queue = unsafe { self.ash_device.get_device_queue(indices.transfer.unwrap(), 0) };
    }

    fn create_swapchain(&mut self, glfw_window: &glfw::PWindow) {
        let support_details = SwapChainInfo::query_swapchain_support(self, self.physical_device);

        assert!(support_details.capabilities.min_image_count <= vkutl::SWAPCHAIN_IMAGES_COUNT as u32, "SwapChain images count not supported");


        // choose the best settings for our swap chain

        // if the preferred format isn't available, just return the first one from the list
        self.swapchain_info.surface_format = *support_details.formats.iter().find(|format|
            format.format == vk::Format::B8G8R8A8_UNORM && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        ).unwrap_or(&support_details.formats[0]);

        // try to choose VK_PRESENT_MODE_MAILBOX_KHR mode
        // if isn't available just return FIFO mode (which is guaranteed to be available)
        self.swapchain_info.present_mode = *support_details.present_modes.iter().find(|mode|
            **mode == vk::PresentModeKHR::MAILBOX
                ).unwrap_or(&if vkutl::V_SYNC { vk::PresentModeKHR::FIFO } else { vk::PresentModeKHR::IMMEDIATE });


        self.swapchain_info.extent =
            if support_details.capabilities.current_extent.width != u32::MAX {
                support_details.capabilities.current_extent
            }
            else {
                let (width, height) = glfw_window.get_framebuffer_size();

                let min_extent = support_details.capabilities.min_image_extent;
                let max_extent = support_details.capabilities.max_image_extent;

                // limits extent between min and max extents supported by the surface
                vk::Extent2D {
                    width: (width as u32).clamp(min_extent.width, max_extent.width),
                    height: (height as u32).clamp(min_extent.height, max_extent.height),
                }
            };

        let mut create_info = vk::SwapchainCreateInfoKHR::default();
        create_info.surface = self.window_surface;
        create_info.min_image_count = vkutl::SWAPCHAIN_IMAGES_COUNT as u32;
        create_info.image_format = self.swapchain_info.surface_format.format;
        create_info.image_color_space = self.swapchain_info.surface_format.color_space;
        create_info.image_extent = self.swapchain_info.extent;
        create_info.image_array_layers = 1;
        create_info.image_usage = vk::ImageUsageFlags::COLOR_ATTACHMENT;

        let family_indices = [
            self.families_indices_cache.graphics.unwrap(),
            self.families_indices_cache.present.unwrap()
        ];

        if family_indices[0] != family_indices[1] {
            create_info.image_sharing_mode = vk::SharingMode::CONCURRENT;
            create_info.queue_family_index_count = 2;
            create_info.p_queue_family_indices = family_indices.as_ptr();
        }
        else {
            create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
        }

        create_info.pre_transform = support_details.capabilities.current_transform;
        create_info.composite_alpha = vk::CompositeAlphaFlagsKHR::OPAQUE;
        create_info.present_mode = self.swapchain_info.present_mode;
        create_info.clipped = vk::TRUE;

        // create the swap chain
        self.swapchain = unsafe {
            self.ash_swapchain.create_swapchain(&create_info, None).expect("Failed to create swap chain!")
        };


        // get swap chain images
        let images = unsafe {
            self.ash_swapchain.get_swapchain_images(self.swapchain).expect("Error to get swapchain images")
        };

        for i in 0..vkutl::SWAPCHAIN_IMAGES_COUNT {
            self.swapchain_info.images[i] = images[i];
        }

        for i in 0..vkutl::SWAPCHAIN_IMAGES_COUNT {
            self.swapchain_info.images_views[i] = vkutl::create_image_view(
                &self,
                images[i],
                self.swapchain_info.surface_format.format,
                vk::ImageAspectFlags::COLOR
            );
        }


        // create depth image
        (self.swapchain_info.depth_image, self.swapchain_info.depth_image_memory) = vkutl::create_image(
            self,
            self.swapchain_info.extent.width, self.swapchain_info.extent.height,
            vk::Format::D32_SFLOAT,
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            true
        );

        // create depth image view
        self.swapchain_info.depth_image_view = vkutl::create_image_view(
            self,
            self.swapchain_info.depth_image,
            vk::Format::D32_SFLOAT,
            vk::ImageAspectFlags::DEPTH
        );
    }

    fn create_render_pass(&mut self) {
        let dependency = [
            vk::SubpassDependency::default()
                .src_subpass(vk::SUBPASS_EXTERNAL)
                .dst_subpass(0)
                .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS)
                .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
                .src_access_mask(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)
                .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)
        ];

        let attachments = [
            // color
            vk::AttachmentDescription::default()
                .format(self.swapchain_info.surface_format.format)
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(vk::AttachmentLoadOp::DONT_CARE)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::PRESENT_SRC_KHR),

            // depth
            vk::AttachmentDescription::default()
                .format(vk::Format::D32_SFLOAT)
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
        ];

        let color_attachment_ref = [
            vk::AttachmentReference::default()
                .attachment(0)
                .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        ];

        let depth_attachment_ref = vk::AttachmentReference::default()
            .attachment(1)
            .layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

        let subpass = [
            vk::SubpassDescription::default()
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .color_attachments(&color_attachment_ref)
                .depth_stencil_attachment(&depth_attachment_ref)
        ];


        let render_pass_info = vk::RenderPassCreateInfo::default()
            .attachments(&attachments)
            .subpasses(&subpass)
            .dependencies(&dependency);


        self.render_pass = unsafe {
            self.ash_device.create_render_pass(&render_pass_info, None).expect("Failed to create render pass!")
        };
    }

    fn create_swapchain_framebuffers(&mut self) {
        for i in 0..vkutl::SWAPCHAIN_IMAGES_COUNT {
            let attachments = [ self.swapchain_info.images_views[i], self.swapchain_info.depth_image_view ];

            let framebuffer_info = vk::FramebufferCreateInfo::default()
                .render_pass(self.render_pass)
                .attachments(&attachments)
                .width(self.swapchain_info.extent.width)
                .height(self.swapchain_info.extent.height)
                .layers(1);

            self.swapchain_info.framebuffers[i] = unsafe {
                self.ash_device.create_framebuffer(&framebuffer_info, None).expect("Failed to create framebuffer!")
            }
        }
    }

    fn create_command_pool(&mut self) {
        let graphics_pool_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(self.families_indices_cache.graphics.unwrap());

        self.graphics_command_pool = unsafe {
            self.ash_device.create_command_pool(&graphics_pool_info, None).expect("Failed to create command pool!")
        };

        let transfer_pool_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(self.families_indices_cache.transfer.unwrap());

        self.transfer_command_pool = unsafe {
            self.ash_device.create_command_pool(&transfer_pool_info, None).expect("Failed to create command pool!")
        };
    }

    fn create_command_buffers(&mut self) {
        let graphics_alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(self.graphics_command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(vkutl::FRAMES_COUNT as u32);

        let grahics_command_buffers = unsafe {
            self.ash_device.allocate_command_buffers(&graphics_alloc_info).expect("Failed to allocate command buffers!")
        };

        for i in 0.. vkutl::FRAMES_COUNT {
            self.graphics_command_buffers[i] = grahics_command_buffers[i];
        }


        let transfer_alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(self.transfer_command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(vkutl::FRAMES_COUNT as u32);

        let transfer_command_buffers = unsafe {
            self.ash_device.allocate_command_buffers(&transfer_alloc_info).expect("Failed to allocate command buffers!")
        };

        for i in 0.. vkutl::FRAMES_COUNT {
            self.transfer_command_buffers[i] = transfer_command_buffers[i];
        }
    }

    fn create_sync_objects(&mut self) {
        let semaphore_info = vk::SemaphoreCreateInfo::default();

        let fence_info = vk::FenceCreateInfo::default()
            .flags(vk::FenceCreateFlags::SIGNALED);

        unsafe {
            for semaphore in &mut self.acquire_semaphores {
                *semaphore = self.ash_device.create_semaphore(&semaphore_info, None).expect("Failed to create semaphore!")
            }

            for semaphore in &mut self.submit_semaphores {
                *semaphore = self.ash_device.create_semaphore(&semaphore_info, None).expect("Failed to create semaphore!")
            }

            for fence in &mut self.frame_fence {
                *fence = self.ash_device.create_fence(&fence_info, None).expect("Failed to create fence!")
            }

            for semaphore in &mut self.transfer_semaphores {
                *semaphore = self.ash_device.create_semaphore(&semaphore_info, None).expect("Failed to create semaphore!")
            }
        }
    }

    fn create_descriptor_pool(&mut self) {
        const MAX_SETS_IN_DESCRIPTORS: u32 = 10;
        const MAX_DESCRIPTOR_COUNT: u32 = 10;

        const POOL_SIZES: [vk::DescriptorPoolSize; 3] = [
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: vkutl::FRAMES_COUNT as u32 * MAX_DESCRIPTOR_COUNT
            },
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: vkutl::FRAMES_COUNT as u32 * MAX_DESCRIPTOR_COUNT
            },
            vk::DescriptorPoolSize {
                ty: vk::DescriptorType::STORAGE_BUFFER,
                descriptor_count: vkutl::FRAMES_COUNT as u32 * MAX_DESCRIPTOR_COUNT
            },
        ];

        let pool_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(&POOL_SIZES)
            .max_sets(MAX_SETS_IN_DESCRIPTORS);


        self.descriptor_pool = unsafe {
            self.ash_device.create_descriptor_pool(&pool_info, None).expect("Failed to create descriptor pool!")
        }
    }

    pub fn begin_frame(&mut self, glfw_window: &glfw::PWindow) {
        // recreate swapchain
        if self.resized {
            self.resized = false;

            // wait for idle
            unsafe { self.ash_device.device_wait_idle().expect("failed to wait idle!") };

            SwapChainInfo::clear(self);
            self.create_swapchain(glfw_window);
            self.create_swapchain_framebuffers();
        }

        // wait for previous frame and reset fence state
        unsafe {
            self.ash_device.wait_for_fences( &[self.frame_fence[self.frame_index]], true, u64::MAX)
                .expect("failed to wait for fences!");

            // get next image from swapChain
            let acquire_result = self.ash_swapchain.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.acquire_semaphores[self.frame_index],
                vk::Fence::null()
            );

            self.image_index = acquire_result.unwrap().0 as usize;

            self.ash_device.reset_fences(&[self.frame_fence[self.frame_index]]).expect("failed to reset fence!");
        };

        // update garbage lists
        for i in (0..self.garbage_lists.len()).rev() {
            if self.frame_count > self.garbage_lists[i].0 {
                let (_, mut garbage_list) = self.garbage_lists.swap_remove(i);

                for garbage in &mut garbage_list {
                    self.destroy_garbage(garbage);
                }

                garbage_list.clear();

                // saves the list in pool to avoid allocate new lists
                self.garbage_list_pool.push(garbage_list);
            }
        }


        let graphics_command_buffer = self.graphics_command_buffers[self.frame_index];
        let transfer_command_buffer = self.transfer_command_buffers[self.frame_index];

        unsafe {
            let begin_info = vk::CommandBufferBeginInfo::default()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

            self.ash_device.reset_command_buffer(graphics_command_buffer, vk::CommandBufferResetFlags::empty()).expect("failed to reset command buffer!");
            self.ash_device.begin_command_buffer(graphics_command_buffer, &begin_info).expect("Failed to begin recording command buffer!");

            self.ash_device.reset_command_buffer(transfer_command_buffer, vk::CommandBufferResetFlags::empty()).expect("failed to reset command buffer!");
            self.ash_device.begin_command_buffer(transfer_command_buffer, &begin_info).expect("Failed to begin recording command buffer!");
        };


        // begin render pass
        const CLEAR_VALUES: [vk::ClearValue; 2] = [
            vk::ClearValue {color: vk::ClearColorValue { float32: [0.3, 0.5, 1.0, 1.0] } },
            vk::ClearValue {depth_stencil: vk::ClearDepthStencilValue { depth: 1.0, stencil: 0 } },
        ];

        let render_pass_info = vk::RenderPassBeginInfo::default()
            .render_pass(self.render_pass)
            .framebuffer(self.swapchain_info.framebuffers[self.image_index])
            .render_area(vk::Rect2D{ offset: vk::Offset2D::default(), extent: self.swapchain_info.extent })
            .clear_values(&CLEAR_VALUES);

        unsafe {
            self.ash_device.cmd_begin_render_pass(graphics_command_buffer, &render_pass_info, vk::SubpassContents::INLINE);
        }

        // set dynamic states

        let scissor = vk::Rect2D::default()
            .extent(self.swapchain_info.extent);

        let viewport = vk::Viewport::default()
            .y(self.swapchain_info.extent.height as f32)
            .width(self.swapchain_info.extent.width as f32)
            .height(-(self.swapchain_info.extent.height as f32))
            .max_depth(1.0);

        unsafe {
            self.ash_device.cmd_set_viewport(graphics_command_buffer, 0, &[viewport]);
            self.ash_device.cmd_set_scissor(graphics_command_buffer, 0, &[scissor]);
        };
    }

    pub fn end_frame(&mut self) {
        if !self.current_gargabe_list.is_empty() {
            // try to take a new allocated list or create a new list
            let new_list = self.garbage_list_pool.pop().unwrap_or(Vec::new());

            // 'self.current_gargabe_list' now is the list of previous frame, then replace by the new list
            let old_list = std::mem::replace(&mut self.current_gargabe_list, new_list);
            self.garbage_lists.push((self.frame_count + vkutl::SWAPCHAIN_IMAGES_COUNT, old_list));
        }

        let graphics_command_buffer = [ self.graphics_command_buffers[self.frame_index] ];
        let transfer_command_buffer = [ self.transfer_command_buffers[self.frame_index] ];

        unsafe {
            self.ash_device.cmd_end_render_pass(graphics_command_buffer[0]);
            self.ash_device.end_command_buffer(graphics_command_buffer[0]).expect("Failed to record graphics command buffer!");

            self.ash_device.end_command_buffer(transfer_command_buffer[0]).expect("Failed to record transfer command buffer!");
        };

        let graphics_signal_semaphore = [ self.submit_semaphores[self.image_index] ];
        let transfer_signal_semaphore = [ self.transfer_semaphores[self.frame_index] ];
        let graphics_wait_semaphores = [ self.acquire_semaphores[self.frame_index], transfer_signal_semaphore[0] ];

        const WAIT_STAGES: [vk::PipelineStageFlags; 2] = [ vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vk::PipelineStageFlags::TRANSFER ];

        let transfer_submit_info = vk::SubmitInfo::default()
            .command_buffers(&transfer_command_buffer)
            .signal_semaphores(&transfer_signal_semaphore);

        let graphics_submit_info = vk::SubmitInfo::default()
            .wait_dst_stage_mask(&WAIT_STAGES)
            .wait_semaphores(&graphics_wait_semaphores)
            .command_buffers(&graphics_command_buffer)
            .signal_semaphores(&graphics_signal_semaphore);


        let swapchain = [ self.swapchain ];
        let image_index = [ self.image_index as u32 ];

        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&graphics_signal_semaphore)
            .swapchains(&swapchain)
            .image_indices(&image_index);

        unsafe {
            self.ash_device.queue_submit(self.transfer_queue, &[transfer_submit_info], vk::Fence::null())
                .expect("Failed to submit transfer command buffer!");

            self.ash_device.queue_submit(self.graphics_queue, &[graphics_submit_info], self.frame_fence[self.frame_index])
                .expect("Failed to submit draw command buffer!");

            self.ash_swapchain.queue_present(self.present_queue, &present_info).expect("failed to present image");
        }

        // advance to the next frame
        self.frame_index = (self.frame_index + 1) % vkutl::FRAMES_COUNT;
        self.frame_count += 1;
    }
}
