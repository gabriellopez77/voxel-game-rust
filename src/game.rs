use std::{rc::Rc, cell::RefCell};
use crate::{inputs, window};
use crate::resources::ResourceManager;
use crate::ui::UiManager;
use crate::window::Window;
use crate::world::{Player, Planet, blocks::BlocksManager, sky::Sky};


pub struct Game {
    player: Player,
    planet: Planet,
    sky: Sky,

    ui_manager: UiManager,
    resources_manager: Rc<RefCell<ResourceManager>>,
    blocks_manager: Option<BlocksManager>,

    paused: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            planet: Planet::new(),
            sky: Sky::new(),

            ui_manager: UiManager::new(),
            resources_manager: Rc::new(RefCell::new(ResourceManager::new())),
            blocks_manager: None,

            paused: false,
        }
    }

    pub fn start(&mut self) {
        self.resources_manager.borrow_mut().start();
        self.ui_manager.start(self.resources_manager.clone());
        self.blocks_manager = Some(BlocksManager::new(&self.resources_manager));

        self.player.start(self.resources_manager.clone());
        self.planet.start(self.resources_manager.clone());

        self.sky.start(self.resources_manager.clone());
        
        //let now = std::time::Instant::now();

        //println!("{}", now.elapsed().as_micros());
    }

    pub fn update(&mut self, dt: f32, window: &mut Window) {
        if inputs::is_key_pressed(inputs::Keys::Escape) {
            self.paused = !self.paused;

            if self.paused { window.set_cursor(glfw::CursorMode::Normal) }
            else { window.set_cursor(glfw::CursorMode::Disabled) }
        }

        if !self.paused {
            self.player.update(dt);
            self.sky.update();
            self.planet.update(self.player.camera.position, &self.blocks_manager.as_ref().unwrap());
        }

        self.ui_manager.update(dt);
    }

    pub fn render(&mut self) {
        self.sky.draw();
        self.planet.draw(&self.player.camera, &self.blocks_manager.as_ref().unwrap());

        self.player.camera.view_changed = false;
        self.ui_manager.draw();
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.ui_manager.resize(width as f32, height as f32);
        self.player.camera.resize(width as f32, height as f32)
    }
}
