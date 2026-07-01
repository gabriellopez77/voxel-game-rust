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
    pub depth_image_memory: vk_mem::Allocation,
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
            depth_image_memory: unsafe { mem::zeroed() },
        }
    }

    pub fn clear(vulkan_app: &mut VulkanApp) {
        //for i in 0..vkutl::FRAMES_COUNT {
        //    unsafe {
        //        vulkan_app.ash_device.destroy_framebuffer(vulkan_app.swapchain_info.framebuffers[i], None);
        //        vulkan_app.ash_device.destroy_image_view(vulkan_app.swapchain_info.images_views[i], None);
        //    }
        //}

        unsafe {
            // depth buffer
            vulkan_app.ash_device.destroy_image_view(vulkan_app.swapchain_info.depth_image_view, None);
            vulkan_app.ash_device.destroy_image(vulkan_app.swapchain_info.depth_image, None);
            vulkan_app.vma_allocator.free_memory(&mut vulkan_app.swapchain_info.depth_image_memory);

            vulkan_app.ash_swapchain.destroy_swapchain(vulkan_app.swapchain, None);
        }
    }

    pub fn is_adequade(vulkan_app: &VulkanApp, physical_device: vk::PhysicalDevice) -> bool {
        let support_details = Self::query_swapchain_support(vulkan_app, physical_device);

        return !support_details.formats.is_empty() && !support_details.present_modes.is_empty();
    }

    pub fn query_swapchain_support(vulkan_app: &VulkanApp, physical_device: vk::PhysicalDevice) -> SwapchainSupportDetails {
        return SwapchainSupportDetails {
            capabilities: unsafe {
                vulkan_app.ash_surface.get_physical_device_surface_capabilities(physical_device, vulkan_app.window_surface).unwrap()
            },

            formats: unsafe {
                vulkan_app.ash_surface.get_physical_device_surface_formats(physical_device, vulkan_app.window_surface)
            }.unwrap_or_else(|_| Vec::new()),

            present_modes: unsafe {
                vulkan_app.ash_surface.get_physical_device_surface_present_modes(physical_device, vulkan_app.window_surface)
            }.unwrap_or_else(|_| Vec::new())
        };
    }
}
