use crate::math::vec3::Vec3;
use crate::math::vec2::Vec2;
use crate::math::matrix4::Matrix4;

use crate::inputs;
use crate::render::Ubo;

pub struct Camera {
    view_matrix: Matrix4,
    projection_matrix: Matrix4,
    pub position: Vec3,

    pub direction: Vec3,
    pub rot: Vec2,
    
    ubo: Ubo
}

impl Camera {
    pub fn new() -> Self {
        Self {
            view_matrix: Matrix4::ZERO,
            projection_matrix: Matrix4::ZERO,
            position: Vec3::ZERO,
            direction: Vec3::ZERO,
            rot: Vec2::ZERO,
            
            ubo: Ubo::new()
        }
    }
    
    pub fn start(&mut self) {
        self.ubo.add::<Matrix4>("projection");
        self.ubo.add::<Matrix4>("view");
        self.ubo.create(1);
    }

    pub fn update(&mut self, dt: f32) {
        self.process_rotation();

        self.view_matrix = Matrix4::look_at(self.position, self.position + self.direction);
        
        self.ubo.update("view", self.view_matrix.as_ptr());
    }
    
    pub fn resize(&mut self, width: f32, height: f32) {
        self.projection_matrix = Matrix4::perspective(80.0, width / height, 0.1, 100.0);
        self.ubo.update("projection", self.projection_matrix.as_ptr());
    }

    fn process_rotation(&mut self) {
        const SENSITIVYTY: f32 = 0.7;

        let delta = inputs::get_mouse_delta() * SENSITIVYTY;

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