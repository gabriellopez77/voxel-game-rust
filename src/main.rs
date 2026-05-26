use std::collections::HashSet;

use crate::utils::FastString;

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
    //test1();
    //test2();
    let (mut window, events) = window::Window::init(1050, 650, "My First Rust Window");
    let mut game = game::Game::new();

    window.run(&mut game, &events);
}

fn test1() {
    println!("\ncommon:");
    let mut test: HashSet<String> = HashSet::with_capacity(100_000);

    for i in 0..10 {
        let now = std::time::Instant::now();

        for i in 0..100_00 {
            test.clear();
            test.insert("Hello, World, 123456789".to_string());
        }

        println!("time: {}", now.elapsed().as_millis());
    }
}

fn test2() {
    println!("\nfast:");
    let mut test2: HashSet<FastString> = HashSet::with_capacity(100_000);
    for i in 0..10 {
        let now = std::time::Instant::now();

        for i in 0..100_00 {
            test2.clear();
            test2.insert(FastString::StaticRawString("Hello, World, 123456789"));
        }

        println!("time: {}", now.elapsed().as_millis());
    }
}
