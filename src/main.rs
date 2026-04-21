mod render;
mod resources;
mod window;
mod inputs;
mod math;
mod game;
mod world;

use crate::game::Game;

use crate::resources::path;

fn main() {
    let mut window = window::Window::init(800, 600, "My First Rust Window");

    // init paths
    path::SHADER_PATH.get_or_init(|| std::env::current_dir().unwrap().display().to_string() + r"\assets\shaders");
    
    window::Window::set_window_instance(&mut window);

    
    window.run();
}

