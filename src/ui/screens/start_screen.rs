use crate::render::UiRenderer;
use crate::resources::ResourceManager;
use crate::ui::screen_base::ScreenInfo;
use crate::ui::ScreenBase;


pub struct StartScreen {
    
}

impl ScreenBase for StartScreen {
    fn start(&mut self, resource_manager: &ResourceManager, args: &ScreenInfo) {

    }

    fn update(&mut self, dt: f32, args: &ScreenInfo) {

    }

    fn draw(&mut self, renderer: &mut UiRenderer) {

    }

    fn resize(&mut self, args: &ScreenInfo) {

    }
}

impl StartScreen {
    pub fn new() -> Self {
        Self {
            
        }
    }
}
