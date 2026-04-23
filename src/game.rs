use crate::math::{Color4b, Vec2i16, Vec4};
use crate::render::SpritesVertices;
use crate::world::player::camera::Camera;
use crate::ui::screens_manager::ScreenManager;


pub struct Game {
    camera: Camera,

    screen_manager: ScreenManager,
}

impl Game {
    pub fn new() -> Self { Self { camera: Camera::new(), screen_manager: ScreenManager::new() } }

    pub fn start(&mut self) {
        self.screen_manager.start();
    }

    pub fn update(&mut self, dt: f32) {
        self.camera.update(dt);

        self.screen_manager.update(dt);

        self.screen_manager.sprites_renderer.buffer.add(SpritesVertices {
            position: Vec2i16::ZERO,
            size: Vec2i16{x: 10, y: 10},
            uv: Vec4{x: 0.0, y: 0.0, z: 1.0, w: 1.0},
            color: Color4b::ZERO
        });
    }

    pub fn render(&mut self) {
        self.screen_manager.sprites_renderer.shader.uniform_matrix("view", &self.camera.view_matrix);
        self.screen_manager.draw();
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.screen_manager.resize(width as f32, height as f32);
    }
}