use crate::math;
use crate::render::{SpritesRenderer, Ubo};


pub struct ScreenManager {
    pub sprites_renderer: SpritesRenderer,
    sprites_ubo: Ubo,
}

impl ScreenManager {
    pub fn new() -> Self { Self { sprites_renderer: SpritesRenderer::new(), sprites_ubo: Ubo::new() } }

    pub fn start(&mut self) {
        self.sprites_renderer.start();

        self.sprites_ubo.add::<math::Matrix4>("projection");
        self.sprites_ubo.create(0);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        let projection = math::Matrix4::perspective(70.0, width / height, 0.1, 100.0);

        self.sprites_ubo.update("projection", projection.as_ptr() as *const ());
    }

    pub fn update(&mut self, dt: f32) {

    }

    pub fn draw(&mut self) {
        self.sprites_renderer.draw();
    }
}