use crate::resources::resources_utils::ResourceManager;
use crate::world::player::camera::Camera;
use crate::ui::screens_manager::ScreenManager;


pub struct Game {
    camera: Camera,

    screen_manager: ScreenManager,
    resource_manager: ResourceManager
}

impl Game {
    pub fn new() -> Self { 
        Self { 
            camera: Camera::new(), 
            screen_manager: ScreenManager::new(),
            resource_manager: ResourceManager::new()
        } 
    }

    pub fn start(&mut self) {
        self.resource_manager.start();
        
        self.screen_manager.start(&self.resource_manager);
    }

    pub fn update(&mut self, dt: f32) {
        self.camera.update(dt);

        self.screen_manager.update(dt);
    }

    pub fn render(&mut self) {
        self.screen_manager.draw();
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.screen_manager.resize(width as f32, height as f32);
    } 
}