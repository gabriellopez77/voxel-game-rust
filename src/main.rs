mod render;
mod resources;
mod window;
mod inputs;
mod math;
mod game;
mod world;
mod ui;

use crate::resources::path;

fn main() {
    let mut window = window::Window::init(800, 600, "My First Rust Window");

    // init paths
    path::SHADERS_PATH.get_or_init(|| std::env::current_dir().unwrap().display().to_string() + r"\assets\shaders");
    path::TEXTURES_PATH.get_or_init(|| std::env::current_dir().unwrap().display().to_string() + r"\assets\textures");
    
    window::Window::set_window_instance(&mut window);
    
    window.run();
}

