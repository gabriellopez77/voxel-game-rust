use std::{rc::Rc, cell::RefCell};
use crate::{inputs, window};
use crate::resources::ResourceManager;
use crate::ui::UiManager;
use crate::window::Window;
use crate::world::{Player, Planet, blocks::BlocksManager, sky::Sky, sky::Clouds};


pub struct Game {
    pub player: Player,
    planet: Planet,
    sky: Sky,
    clouds: Clouds,

    ui_manager: Rc<RefCell<UiManager>>,
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
            clouds: Clouds::new(),

            ui_manager: Rc::new(RefCell::new(UiManager::new())),
            resources_manager: Rc::new(RefCell::new(ResourceManager::new())),
            blocks_manager: None,

            paused: false,
        }
    }

    pub fn start(&mut self) {
        self.resources_manager.borrow_mut().start();
        self.ui_manager.borrow_mut().start(self.resources_manager.clone(), self);
        self.blocks_manager = Some(BlocksManager::new(&self.resources_manager));

        self.player.start(&self.resources_manager.borrow(), &self.blocks_manager.as_ref().unwrap());
        self.planet.start(self.resources_manager.clone());

        self.sky.start(&self.resources_manager.borrow());
        self.clouds.start(self.resources_manager.clone());

        //let now = std::time::Instant::now();

        //println!("{}", now.elapsed().as_micros());
    }

    pub fn update(&mut self, dt: f32, window: &mut Window) {
        if inputs::key_pressed(inputs::Keys::Escape) {
            self.paused = !self.paused;

            if self.paused { window.set_cursor(glfw::CursorMode::Normal) }
            else { window.set_cursor(glfw::CursorMode::Disabled) }
        }

        if !self.paused {
            self.player.update(dt, &self.planet, &self.blocks_manager.as_ref().unwrap());
            
            self.sky.update(dt, self.planet.render_distance);
            self.clouds.update(self.player.camera.position, self.planet.render_distance, self.sky.time);

            self.planet.update(self.player.camera.position, &self.blocks_manager.as_ref().unwrap());
        }

        self.ui_manager.clone().borrow_mut().update(dt, self);
    }

    pub fn render(&mut self) {
        self.sky.draw();
        self.clouds.draw();
        
        self.planet.draw(&self.player.camera, &self.blocks_manager.as_ref().unwrap());

        self.player.selection_box.draw();


        self.player.camera.view_changed = false;
        self.ui_manager.clone().borrow_mut().draw();
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.ui_manager.clone().borrow_mut().resize(width, height, self);
        self.player.camera.resize(width, height)
    }
}
