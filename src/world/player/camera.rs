use crate::math::vec3::Vec3;
use crate::math::vec2::Vec2;
use crate::math::matrix4::Matrix4;

use crate::inputs;

pub struct Camera {
    pub view_matrix: Matrix4,
    pub position: Vec3,

    pub direction: Vec3,
    pub rotation: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            view_matrix: Matrix4::ZERO,
            position: Vec3::ZERO,
            direction: Vec3::ZERO,
            rotation: Vec2::ZERO
        }
    }

    pub fn update(&mut self) {
        self.process_rotation();

        self.view_matrix = Matrix4::look_at(self.position, self.position + self.direction);
    }

    fn process_rotation(&mut self) {
        const sensitivity: f32 = 0.7;

        let delta = inputs::get_mouse_delta() * sensitivity;

        self.rotation.x += delta.x;
        self.rotation.y -= delta.y;

        self.rotation.y = self.rotation.y.clamp(-89.0, 89.0);

        let direction = Vec3 {
            x: f32::cos(f32::to_radians(self.rotation.x)) * f32::cos(f32::to_radians(self.rotation.y)),
            y: f32::sin(f32::to_radians(self.rotation.y)),
            z: f32::sin(f32::to_radians(self.rotation.x)) * f32::cos(f32::to_radians(self.rotation.y))
        };
        
        self.direction = direction.normalized();
    }
}