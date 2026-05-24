use std::cell::RefCell;
use std::rc::Rc;

use crate::inputs;
use crate::math::Vec3;
use crate::resources::ResourceManager;
use crate::world::player::camera::Camera;


pub struct Player {
    pub camera: Camera,
}

impl Player {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
        }
    }

    pub fn start(&mut self, resources_manager: Rc<RefCell<ResourceManager>>) {
        self.camera.start(resources_manager);
        self.camera.position.y = 60.0;
    }

    pub fn update(&mut self, dt: f32) {
        let mut dir = Vec3::ZERO;

        let yaw = self.camera.rot.x.to_radians();
        let front = Vec3 { x: yaw.cos(), y: 0.0, z: yaw.sin() };

        if inputs::is_key_down(inputs::Keys::W) { dir = dir + front };
        if inputs::is_key_down(inputs::Keys::A) { dir = dir - front.cross(Vec3 { x: 0.0, y: 1.0, z: 0.0 }) };
        if inputs::is_key_down(inputs::Keys::S) { dir = dir - front };
        if inputs::is_key_down(inputs::Keys::D) { dir = dir + front.cross(Vec3 { x: 0.0, y: 1.0, z: 0.0 }) };
        if inputs::is_key_down(inputs::Keys::LeftShift) { dir.y -= 1.0 };
        if inputs::is_key_down(inputs::Keys::Space) { dir.y += 1.0 };

        const SPEED: f32 = 10.0;

        if dir.length() > 1.0 {
            dir = dir.normalized()
        }

        let new_pos = self.camera.position + dir * (SPEED * dt);
        self.camera.update(new_pos);
    }
}
