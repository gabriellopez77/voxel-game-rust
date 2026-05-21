use std::{rc::Rc, cell::RefCell};
use crate::{inputs, window};
use crate::resources::ResourceManager;
use crate::ui::screens_manager::ScreenManager;
use crate::world::{Player, Planet, blocks::BlocksManager};


pub struct Game {
    player: Player,
    planet: Planet,

    screen_manager: ScreenManager,
    resource_manager: Rc<RefCell<ResourceManager>>,
    blocks_manager: BlocksManager,

    paused: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            planet: Planet::new(),

            screen_manager: ScreenManager::new(),
            resource_manager: Rc::new(RefCell::new(ResourceManager::new())),
            blocks_manager: BlocksManager::new(),

            paused: false,
        }
    }

    pub fn start(&mut self) {
        self.resource_manager.borrow_mut().start();
        self.blocks_manager.start();
        self.screen_manager.start(self.resource_manager.clone());

        self.player.start();
        self.planet.start(self.resource_manager.clone());


        //let now = std::time::Instant::now();

        //println!("{}", now.elapsed().as_micros());
    }

    pub fn update(&mut self, dt: f32, window: &mut window::Window) {
        if inputs::is_key_pressed(inputs::Keys::Escape) {
            self.paused = !self.paused;

            if self.paused {window.set_cursor(glfw::CursorMode::Normal);}
            else {window.set_cursor(glfw::CursorMode::Hidden);}
        }

        if !self.paused {
            self.player.update(dt);

            self.planet.update(self.player.camera.position);
        }

        self.screen_manager.update(dt);
    }

    pub fn render(&mut self) {
        self.screen_manager.draw();
        self.planet.draw(&self.player.camera);

        self.player.camera.view_changed = false;
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.screen_manager.resize(width as f32, height as f32);
        self.player.camera.resize(width as f32, height as f32)
    }
}
