use std::mem;
use ash::vk;

use super::VulkanApp;
use super::vkutl;


pub struct SwapchainSupportDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

pub struct SwapChainInfo {
    pub extent: vk::Extent2D,
    pub surface_format: vk::SurfaceFormatKHR,
    pub present_mode: vk::PresentModeKHR,

    pub images: [vk::Image; vkutl::SWAPCHAIN_IMAGES_COUNT],
    pub images_views: [vk::ImageView; vkutl::SWAPCHAIN_IMAGES_COUNT],
    pub framebuffers: [vk::Framebuffer; vkutl::SWAPCHAIN_IMAGES_COUNT],

    pub depth_image: vk::Image,
    pub depth_image_view: vk::ImageView,
    pub depth_image_allocation: vk_mem::Allocation,
}

impl SwapChainInfo {
    pub fn new() -> Self {
        Self {
            extent: vk::Extent2D { width: 0, height: 0 },
            surface_format: vk::SurfaceFormatKHR::default(),
            present_mode: vk::PresentModeKHR::default(),

            images: [vk::Image::null(); vkutl::SWAPCHAIN_IMAGES_COUNT],
            images_views: [vk::ImageView::null(); vkutl::SWAPCHAIN_IMAGES_COUNT],
            framebuffers: [vk::Framebuffer::null(); vkutl::SWAPCHAIN_IMAGES_COUNT],

            depth_image: vk::Image::null(),
            depth_image_view: vk::ImageView::null(),
            depth_image_allocation: unsafe { mem::zeroed() },
        }
    }

    pub fn clear(app: &mut VulkanApp) {
        let swapchain_info = &app.swapchain_info;

        unsafe {
            for i in 0..vkutl::SWAPCHAIN_IMAGES_COUNT {
                app.ash_device.destroy_framebuffer(swapchain_info.framebuffers[i], None);
                app.ash_device.destroy_image_view(swapchain_info.images_views[i], None);
            }

            // depth buffer
            app.ash_device.destroy_image_view(swapchain_info.depth_image_view, None);
            app.ash_device.destroy_image(swapchain_info.depth_image, None);
            app.vma_allocator.free_memory(&mut app.swapchain_info.depth_image_allocation);

            app.ash_swapchain.destroy_swapchain(app.swapchain, None);
        }
    }

    pub fn is_adequade(app: &VulkanApp, physical_device: vk::PhysicalDevice) -> bool {
        let support_details = Self::query_swapchain_support(app, physical_device);

        return !support_details.formats.is_empty() && !support_details.present_modes.is_empty();
    }

    pub fn query_swapchain_support(app: &VulkanApp, physical_device: vk::PhysicalDevice) -> SwapchainSupportDetails {
        let surface = app.window_surface;

        return unsafe { SwapchainSupportDetails {
            capabilities: app.ash_surface.get_physical_device_surface_capabilities(physical_device, surface).unwrap(),
            formats: app.ash_surface.get_physical_device_surface_formats(physical_device, surface).unwrap_or(Vec::new()),
            present_modes: app.ash_surface.get_physical_device_surface_present_modes(physical_device, surface).unwrap_or(Vec::new())
        }};
    }
}
