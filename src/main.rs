mod render;
mod resources;
mod window;
mod inputs;
mod math;
mod game;
mod world;
mod ui;


fn main() {
    let (mut window, events) = window::Window::init(800, 600, "My First Rust Window");
    let mut game = game::Game::new();
    
    window.run(&mut game, &events);
}