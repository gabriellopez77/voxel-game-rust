mod render;
mod resources;
mod window;
mod inputs;
mod math;
mod game;
mod world;
mod ui;

use crate::resources::resources_utils;

fn main() {
    let mut window = window::Window::init(800, 600, "My First Rust Window");
    
    window::Window::set_window_instance(&mut window);
    
    window.run();
}

