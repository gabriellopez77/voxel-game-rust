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
    let (mut window, events) = window::Window::init(1050, 650, "My First Rust Window");

    window.run(&events);
}
