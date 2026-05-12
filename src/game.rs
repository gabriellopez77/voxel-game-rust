use std::{rc::Rc, cell::RefCell};
use crate::resources::ResourceManager;
use crate::ui::screens_manager::ScreenManager;
use crate::world::Planet;
use crate::world::Player;
use crate::world::blocks::BlocksManager;


pub struct Game {
    player: Player,
    planet: Planet,
    
    screen_manager: ScreenManager,
    resource_manager: Rc<RefCell<ResourceManager>>,
    blocks_manager: BlocksManager,
}

impl Game {
    pub fn new() -> Self { 
        Self { 
            player: Player::new(),
            planet: Planet::new(),

            screen_manager: ScreenManager::new(),
            resource_manager: Rc::new(RefCell::new(ResourceManager::new())),
            blocks_manager: BlocksManager::new(),
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

    pub fn update(&mut self, dt: f32) {
        self.player.update(dt);

        
        self.screen_manager.update(dt);
    }

    pub fn render(&mut self) {
        
        self.screen_manager.draw();
        self.planet.draw();
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.screen_manager.resize(width as f32, height as f32);
        self.player.camera.resize(width as f32, height as f32)
    } 
}