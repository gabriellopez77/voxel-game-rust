use std::cell::RefCell;
use std::rc::Rc;

use crate::math::Vec3i;
use crate::math::vec3::Vec3;
use crate::math::vec2::Vec2;
use crate::math::matrix4::Matrix4;

use crate::inputs;
use crate::render::Ubo;
use crate::resources::ResourceManager;
use crate::world::Chunk;


#[derive(Copy, Clone)]
struct Plane {
    normal: Vec3,
    d: f32,
}

impl Plane {
    pub fn distance(&self, p: Vec3) -> f32 { Vec3::dot(&self.normal, p) + self.d }
}

pub struct Camera {
    pub position: Vec3,

    pub direction: Vec3,
    pub rot: Vec2,
    pub view_changed: bool,

    ubo: Option<Rc<Ubo>>,

    view_matrix: Matrix4,
    projection_matrix: Matrix4,
    projection_view_matrix: Matrix4,

    frustum_planes: [Plane; 6],

    last_mouse_pos: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            direction: Vec3::ZERO,
            rot: Vec2::ZERO,
            view_changed: false,

            ubo: None,

            view_matrix: Matrix4::ZERO,
            projection_matrix: Matrix4::ZERO,
            projection_view_matrix: Matrix4::ZERO,

            frustum_planes: [Plane{normal: Vec3::ZERO, d: 0.0}; 6],

            last_mouse_pos: Vec2::ZERO,
        }
    }

    pub fn start(&mut self, resources_manager: Rc<RefCell<ResourceManager>>) {
        self.ubo = resources_manager.borrow().get_ubo("globalData");
    }

    pub fn update(&mut self, new_pos: Vec3) {
        if self.position != new_pos {
            self.view_changed = true;
        }

        self.position = new_pos;

        self.process_rotation();

        if !self.view_changed { return }

        self.view_matrix = Matrix4::look_at(self.position, self.position + self.direction);
        self.projection_view_matrix = self.projection_matrix * self.view_matrix;

        self.update_frustum_planes();

        self.ubo.as_ref().unwrap().update("camView", self.view_matrix.as_ptr());
        self.ubo.as_ref().unwrap().update("camViewNoTranslate", &self.view_matrix.remove_translation());
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.view_changed = true;

        self.projection_matrix = Matrix4::perspective(70.0, width / height, 0.1, 1000.0);
        self.ubo.as_ref().unwrap().update("camProj", self.projection_matrix.as_ptr());
    }

    pub fn chunk_inside_frustum(&self, chunk_pos: Vec3i) -> bool  {
        let visual_chunk_pos = Vec3::new(
            (chunk_pos.x * Chunk::CHUNK_SIZE.x) as f32,
            (chunk_pos.y * Chunk::CHUNK_SIZE.y) as f32,
            (chunk_pos.z * Chunk::CHUNK_SIZE.z) as f32
        );

        for plane in &self.frustum_planes {
            let n = Vec3::new(
                if plane.normal.x >= 0.0 { visual_chunk_pos.x + Chunk::CHUNK_SIZEF.x } else { visual_chunk_pos.x },
                if plane.normal.y >= 0.0 { Chunk::CHUNK_SIZEF.y } else { visual_chunk_pos.y },
                if plane.normal.z >= 0.0 { visual_chunk_pos.z + Chunk::CHUNK_SIZEF.z } else { visual_chunk_pos.z }
            );

            // completely outside frustum
            if plane.distance(n) < 0.0 { return false }
        }

        return true;
    }

    fn process_rotation(&mut self) {
        let last_rotate = self.rot;

        const SENSITIVYTY: f32 = 0.2;

        let delta = (inputs::get_mouse_pos() - self.last_mouse_pos) * SENSITIVYTY;
        self.last_mouse_pos = inputs::get_mouse_pos();

        self.rot.x += delta.x;
        self.rot.y -= delta.y;

        self.rot.y = self.rot.y.clamp(-89.0, 89.0);

        let direction = Vec3 {
            x: f32::to_radians(self.rot.x).cos() * f32::to_radians(self.rot.y).cos(),
            y: f32::to_radians(self.rot.y).sin(),
            z: f32::to_radians(self.rot.x).sin() * f32::to_radians(self.rot.y).cos()
        };

        self.direction = direction.normalized();

        if last_rotate != self.rot {
            self.view_changed = true;
        }
    }

    fn update_frustum_planes(&mut self) {
        let row0 = self.projection_view_matrix.get_row0();
        let row1 = self.projection_view_matrix.get_row1();
        let row2 = self.projection_view_matrix.get_row2();
        let row3 = self.projection_view_matrix.get_row3();

        // left
        let left = row3 + row0;
        self.frustum_planes[0] = Plane { normal: Vec3::from4(left), d: left.w };

        // right
        let right = row3 - row0;
        self.frustum_planes[1] = Plane { normal: Vec3::from4(right), d: right.w };

        // bottom
        let bottom = row3 + row1;
        self.frustum_planes[2] = Plane { normal: Vec3::from4(bottom), d: bottom.w };

        // top
        let top = row3 - row1;
        self.frustum_planes[3] = Plane { normal: Vec3::from4(top), d: top.w };

        // near
        let near = row3 + row2;
        self.frustum_planes[4] = Plane { normal: Vec3::from4(near), d: near.w };

        // far
        let far = row3 - row2;
        self.frustum_planes[5] = Plane { normal: Vec3::from4(far), d: far.w };

        // normalize planes
        for plane in &mut self.frustum_planes {
            let length = plane.normal.length();

            plane.normal /= length;
            plane.d /= length;
        }
    }
}
