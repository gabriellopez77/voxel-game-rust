use crate::render::core::VulkanApp;

mod render;
mod resources;
mod window;
mod inputs;
mod math;
mod game;
mod world;
mod ui;
mod utils;


fn main() {
    let mut window = window::Window::init(1050, 650, "Voxel Game");
    let mut vulkan_app = VulkanApp::new();

    window.run(&mut vulkan_app);
    vulkan_app.cleanup();
}
