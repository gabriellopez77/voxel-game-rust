use crate::render::UiRenderer;
use crate::resources::ResourceManager;
use crate::ui::{ScreenBase, ScreenUpdateArgs};


pub struct StartScreen {
    
}

impl ScreenBase for StartScreen {
    fn start(&mut self, resource_manager: &ResourceManager, args: &ScreenUpdateArgs) {

    }

    fn update(&mut self, dt: f32, args: &ScreenUpdateArgs) {

    }

    fn draw(&mut self, renderer: &mut UiRenderer) {

    }

    fn resize(&mut self, args: &ScreenUpdateArgs) {

    }
}

impl StartScreen {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
