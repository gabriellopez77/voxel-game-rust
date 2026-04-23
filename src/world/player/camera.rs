use crate::math::vec3::Vec3;
use crate::math::vec2::Vec2;
use crate::math::matrix4::Matrix4;

use crate::inputs;

pub struct Camera {
    pub view_matrix: Matrix4,
    pub position: Vec3,

    pub direction: Vec3,
    pub rot: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            view_matrix: Matrix4::ZERO,
            position: Vec3::ZERO,
            direction: Vec3::ZERO,
            rot: Vec2::ZERO
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut dir = Vec3::ZERO;

        let yaw = self.rot.x.to_radians();
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

        self.position += dir * (SPEED * dt);

        self.process_rotation();

        self.view_matrix = Matrix4::look_at(self.position, self.position + self.direction);
    }

    fn process_rotation(&mut self) {
        const sensitivity: f32 = 0.7;

        let delta = inputs::get_mouse_delta() * sensitivity;

        self.rot.x += delta.x;
        self.rot.y -= delta.y;

        self.rot.y = self.rot.y.clamp(-89.0, 89.0);

        let direction = Vec3 {
            x: f32::to_radians(self.rot.x).cos() * f32::to_radians(self.rot.y).cos(),
            y: f32::to_radians(self.rot.y).sin(),
            z: f32::to_radians(self.rot.x).sin() * f32::to_radians(self.rot.y).cos()
        };
        
        self.direction = direction.normalized();
    }
}