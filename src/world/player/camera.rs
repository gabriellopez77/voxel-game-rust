use std::f32;

use crate::math::{self, Matrix4, Vec2, Vec3, Vec3i};

use crate::world::{Aabb, Chunk, Planet};


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PerspectiveMode {
    FirstPerson,
    ThridPersonBack,
    ThridPersonFront,
}

#[derive(Copy, Clone)]
struct Plane {
    normal: Vec3,
    d: f32,
}

impl Plane {
    pub fn distance(&self, p: Vec3) -> f32 { Vec3::dot(&self.normal, p) + self.d }
}

pub struct Camera {
    position: Vec3,
    chunk_block: Vec3i,
    chunk_pos: Vec3i,

    target: Vec3,
    direction: Vec3,
    rot: Vec2,

    pub view_matrix: Matrix4,
    pub projection_matrix: Matrix4,
    pub viewproj_matrix: Matrix4,
    pub view_no_translate_matrix: Matrix4,
    pub first_person_projection_matrix: Matrix4,

    frustum_planes: [Plane; 6],

    pub view_changed: bool,
    pub is_underwater: bool,
    perspective_mode: PerspectiveMode,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            chunk_block: Vec3i::ZERO,
            chunk_pos: Vec3i::ZERO,

            target: Vec3::ZERO,
            direction: Vec3::new(1.0, 0.0, 0.0),
            rot: Vec2::ZERO,

            view_matrix: Matrix4::ZERO,
            projection_matrix: Matrix4::ZERO,
            viewproj_matrix: Matrix4::ZERO,
            view_no_translate_matrix: Matrix4::ZERO,
            first_person_projection_matrix: Matrix4::ZERO,

            frustum_planes: [Plane { normal: Vec3::ZERO, d: 0.0 }; 6],

            view_changed: false,
            is_underwater: false,
            perspective_mode: PerspectiveMode::FirstPerson,
        }
    }

    pub fn start(&mut self) {

    }

    pub fn get_perspective_type(&self) -> PerspectiveMode { self.perspective_mode }
    pub fn get_pos(&self) -> Vec3 { self.position }
    pub fn get_dir(&self) -> Vec3 { self.direction }
    pub fn get_rot(&self) -> Vec2 { self.rot }

    pub fn get_forward(&self) -> Vec3 {
        let yaw_rad = self.rot.x.to_radians();

        Vec3::new(yaw_rad.cos(), 0.0, yaw_rad.sin())
    }

    pub fn change_type(&mut self, new_type: PerspectiveMode) {
        self.view_changed = true;

        self.perspective_mode = new_type;
    }

    pub fn update(&mut self, player_aabb: &Aabb, planet: &Planet, camera_delta: Vec2) {
        let mut new_pos = player_aabb.get_center();
        new_pos.y = player_aabb.y0 + 1.7;

        self.process_rotation(new_pos, camera_delta, planet);

        self.chunk_pos = math::get_chunk_pos(self.position);
        self.chunk_block = math::get_chunk_block(self.chunk_pos, self.position);


        // check if camera is on water
        self.is_underwater = if let Some(chunk) = planet.chunks_manager.get_chunk(self.chunk_pos) {
            *chunk.read().unwrap().data.read().unwrap().get_block_properties(self.chunk_block) == planet.blocks_manager.water_block
        } else { false };


        if !self.view_changed { return }

        self.view_matrix = Matrix4::look_at(self.position, self.target);
        self.viewproj_matrix = self.projection_matrix * self.view_matrix;
        self.view_no_translate_matrix = self.view_matrix.remove_translation();
        self.update_frustum_planes();
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.view_changed = true;

        self.view_matrix = Matrix4::look_at(self.position, self.target);
        self.projection_matrix = Matrix4::perspective(70.0, width / height, 0.04, 1000.0);
        self.viewproj_matrix = self.projection_matrix * self.view_matrix;
        self.view_no_translate_matrix = self.view_matrix.remove_translation();
        self.first_person_projection_matrix = Matrix4::perspective(60.0, width / height, 0.04, 10.0);
        self.update_frustum_planes();
    }

    pub fn chunk_inside_frustum(&self, visual_chunk_pos: Vec3) -> bool  {
        for plane in &self.frustum_planes {
            let n = Vec3::new(
                if plane.normal.x >= 0.0 { visual_chunk_pos.x + Chunk::CHUNK_SIZEF.x } else { visual_chunk_pos.x },
                if plane.normal.y >= 0.0 { Chunk::CHUNK_SIZEF.y } else { visual_chunk_pos.y },
                if plane.normal.z >= 0.0 { visual_chunk_pos.z + Chunk::CHUNK_SIZEF.z } else { visual_chunk_pos.z }
            );

            // completely outside frustum
            if plane.distance(n) < 0.0 {
                return false;
            }
        }

        return true;
    }

    fn process_rotation(&mut self, new_pos: Vec3, camera_delta: Vec2, planet: &Planet) {
        const SENSITIVYTY: f32 = 0.2;

        let delta = camera_delta * SENSITIVYTY;


        let last_rotate = self.rot;

        self.rot.x += delta.x;
        self.rot.y -= delta.y;

        self.rot.y = self.rot.y.clamp(-89.0, 89.0);

        self.direction = Vec3::new(
            f32::to_radians(self.rot.x).cos() * f32::to_radians(self.rot.y).cos(),
            f32::to_radians(self.rot.y).sin(),
            f32::to_radians(self.rot.x).sin() * f32::to_radians(self.rot.y).cos()
        ).normalized();

        if last_rotate != self.rot || self.position != new_pos {
            self.view_changed = true;
        }

        match self.perspective_mode {
            PerspectiveMode::FirstPerson => {
                self.target = new_pos + self.direction;

                self.position = new_pos;
            }
            PerspectiveMode::ThridPersonBack => {
                self.target = new_pos;

                self.position = Self::get_ray_pos(self.target, -self.direction, planet);
            }
            PerspectiveMode::ThridPersonFront => {
                self.target = new_pos;

                self.position = Self::get_ray_pos(self.target, self.direction, planet);
            }
        }
    }

    fn update_frustum_planes(&mut self) {
        let row0 = self.viewproj_matrix.get_row0();
        let row1 = self.viewproj_matrix.get_row1();
        let row2 = self.viewproj_matrix.get_row2();
        let row3 = self.viewproj_matrix.get_row3();

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

    fn get_ray_pos(ray_origin: Vec3, ray_dir: Vec3, planet: &Planet) -> Vec3 {
        const DISTANCE: f32 = 4.5;

        let mut pos = Option::<Vec3>::None;

        planet.iterate_over_blocks_raycast(ray_origin, ray_dir, DISTANCE, |stop, it| {
            if let Some(selection_box) = it.block_properties.selection_box {
                let aabb = selection_box.clone_movev(it.global_block);

                if let Some(hit) = aabb.ray_intersect(ray_origin, ray_dir) {
                    *stop = true;
                    pos = Some(hit + (-ray_dir * 0.15));
                }
            }
        });

        if let Some(pos) = pos {
            return pos
        }
        else {
            return ray_origin + ray_dir * DISTANCE;
        }
    }
}
