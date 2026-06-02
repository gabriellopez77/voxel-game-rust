use std::{cell::RefCell, rc::Rc};

use crate::{math::Vec3, render::{Shader, Ubo, Vao, render_utils}, resources::{ResourceManager, resources_manager}};
use crate::math::{Color3b, KeyFrame};
use crate::render::vao::VaoBuffers;
use crate::world::Chunk;


pub struct Sky {
    shader: Option<Rc<RefCell<Shader>>>,
    vao: Vao,
    ubo: Option<Rc<Ubo>>,

    fog_distance: f32,
    fog_density: f32,
    fog_color: Color3b,
    sky_color: Color3b,
    clouds_color: Color3b,

    sky_color_gradient: KeyFrame<Color3b>,
    fog_color_gradient: KeyFrame<Color3b>,
    clouds_color_gradient: KeyFrame<Color3b>,

    pub time: f32,

    update_delay: f32,

}

impl Sky {
    pub const MINUTES_SCALE: f32 = 60.0;
    pub const HOURS_SCALE: f32 = 24.0;
    const UPDATE_DELAY: f32 = 0.25;

    pub const CYCLE_TIME: f32 = Self::MINUTES_SCALE * Self::HOURS_SCALE;
    pub const TIME_MORNING: f32 = Self::MINUTES_SCALE * 6.0 + 30.0; // 6:30

    pub fn new() -> Self {
        Self {
            shader: None,
            vao: Vao::new(),
            ubo: None,

            fog_distance: 0.0,
            fog_density: 0.0,
            fog_color: Color3b::ZERO,
            sky_color: Color3b::ZERO, 
            clouds_color: Color3b::ZERO,

            sky_color_gradient: KeyFrame::new(|factor, current, next| {
                let r = (current.r as f32 + (next.r as f32 - current.r as f32) * factor);
                let g = (current.g as f32 + (next.g as f32 - current.g as f32) * factor);
                let b = (current.b as f32 + (next.b as f32 - current.b as f32) * factor);

                return Color3b::new(r as u8, g as u8, b as u8);
            }),

            fog_color_gradient: KeyFrame::new(|factor, current, next| {
                let r = (current.r as f32 + (next.r as f32 - current.r as f32) * factor);
                let g = (current.g as f32 + (next.g as f32 - current.g as f32) * factor);
                let b = (current.b as f32 + (next.b as f32 - current.b as f32) * factor);

                return Color3b::new(r as u8, g as u8, b as u8);
            }),

            clouds_color_gradient: KeyFrame::new(|factor, current, next| {
                let r = (current.r as f32 + (next.r as f32 - current.r as f32) * factor);
                let g = (current.g as f32 + (next.g as f32 - current.g as f32) * factor);
                let b = (current.b as f32 + (next.b as f32 - current.b as f32) * factor);

                return Color3b::new(r as u8, g as u8, b as u8);
            }),

            time: Self::TIME_MORNING,
            update_delay: 0.0,
        }
    }
    pub fn start(&mut self, resources_manager: &ResourceManager) {
        let (vertices, indices) = resources_manager::gen_sphere(16.0, 16.0);

        let mut vao = Vao::new();

        vao.gen_vao()
            .gen_buffer(VaoBuffers::Ebo)
            .gen_buffer(VaoBuffers::Vbo);

        vao.buffer_data_from_arr(VaoBuffers::Ebo, &indices, gl::STATIC_DRAW);

        vao.buffer_data_from_arr(VaoBuffers::Vbo, &vertices, gl::STATIC_DRAW)
            .attrib_info(0, 3, gl::FLOAT, 0, false)
            .set_stride(size_of::<Vec3>());

        self.vao = vao;
        self.shader = resources_manager.get_shader("skyDome");
        self.ubo = resources_manager.get_ubo("worldData");

        self.set_fog(true);
        self.set_sky_color(Color3b::new(5, 94, 255));
        self.set_fog_color(Color3b::new(128, 204, 255));
        self.set_fog_density(16.0);

        self.sky_color_gradient.frames = vec![
            ((00.0 * Self::MINUTES_SCALE + 00.0) / Self::CYCLE_TIME, Color3b::from_hex(0x010003)),
            ((04.0 * Self::MINUTES_SCALE + 30.0) / Self::CYCLE_TIME, Color3b::from_hex(0x010003)),
            ((06.0 * Self::MINUTES_SCALE + 00.0) / Self::CYCLE_TIME, Color3b::from_hex(0x055EFF)),
            ((17.0 * Self::MINUTES_SCALE + 20.0) / Self::CYCLE_TIME, Color3b::from_hex(0x055EFF)),
            ((18.0 * Self::MINUTES_SCALE + 40.0) / Self::CYCLE_TIME, Color3b::from_hex(0x020203)),
            ((24.0 * Self::MINUTES_SCALE + 00.0) / Self::CYCLE_TIME, Color3b::from_hex(0x020203)),
        ];

        self.fog_color_gradient.frames = vec![
            ((00.0 * Self::MINUTES_SCALE + 00.0) / Self::CYCLE_TIME, Color3b::from_hex(0x010E22) ),
            ((04.0 * Self::MINUTES_SCALE + 30.0) / Self::CYCLE_TIME, Color3b::from_hex(0x010E22) ),
            ((06.0 * Self::MINUTES_SCALE + 00.0) / Self::CYCLE_TIME, Color3b::from_hex(0xD9CCC3) ),
            ((07.0 * Self::MINUTES_SCALE + 00.0) / Self::CYCLE_TIME, Color3b::from_hex(0x80CCFF) ),
            ((17.0 * Self::MINUTES_SCALE + 00.0) / Self::CYCLE_TIME, Color3b::from_hex(0x80CCFF) ),
            ((17.0 * Self::MINUTES_SCALE + 50.0) / Self::CYCLE_TIME, Color3b::from_hex(0xFF9849) ),
            ((18.0 * Self::MINUTES_SCALE + 40.0) / Self::CYCLE_TIME, Color3b::from_hex(0x415066) ),
            ((19.0 * Self::MINUTES_SCALE + 00.0) / Self::CYCLE_TIME, Color3b::from_hex(0x010E22) ),
            ((24.0 * Self::MINUTES_SCALE + 00.0) / Self::CYCLE_TIME, Color3b::from_hex(0x010E22) ),
        ];

        self.clouds_color_gradient.frames = vec![
            ((00.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME, Color3b::from_hex(0x0E0F18)),
            ((04.0 * Sky::MINUTES_SCALE + 30.0) / Sky::CYCLE_TIME, Color3b::from_hex(0x0E0F18)),
            ((06.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME, Color3b::from_hex(0xFFFFFF)),
            ((07.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME, Color3b::from_hex(0xFFFFFF)),
            ((17.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME, Color3b::from_hex(0xFFFFFF)),
            ((17.0 * Sky::MINUTES_SCALE + 50.0) / Sky::CYCLE_TIME, Color3b::from_hex(0xFFFFFF)),
            ((19.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME, Color3b::from_hex(0x0E0F18)),
            ((24.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME, Color3b::from_hex(0x0E0F18)),
        ];
    }

    pub fn update(&mut self, dt: f32, render_distance: i32) {
        self.set_fog_distance(render_distance as f32 - 1.0);

        self.time += dt * 60.0;
        self.update_delay += dt;

        if self.time > Self::CYCLE_TIME { self.time = 0.0 }

        let factor = self.time / Self::CYCLE_TIME;

        //if self.update_delay > Self::UPDATE_DELAY {
            self.set_sky_color(self.sky_color_gradient.get(factor));
            self.set_fog_color(self.fog_color_gradient.get(factor));
            self.set_clouds_color(self.clouds_color_gradient.get(factor));
            self.update_delay = 0.0;
        //}
    }

    pub fn draw(&mut self) {
        unsafe { gl::Disable(gl::DEPTH_TEST) }

        render_utils::draw_indexed(
            &self.shader.as_ref().unwrap(),
            None,
            &self.vao
        );
        unsafe { gl::Enable(gl::DEPTH_TEST) }
    }

    pub fn set_fog_distance(&mut self, distance: f32) {
        if self.fog_distance == distance { return }

        self.fog_distance = distance;

        let norm_distance = 1.0 / Chunk::CHUNK_SIZEF.x / distance;
        self.ubo.as_ref().unwrap().update("fogDistance", &norm_distance);
    }

    pub fn set_fog_density(&mut self, density: f32) {
        if self.fog_density == density { return }

        self.fog_density = density;

        self.ubo.as_ref().unwrap().update("fogDensity", &density);
    }

    pub fn set_fog(&self, value: bool) {
        let i32_value = value as i32;

        self.ubo.as_ref().unwrap().update("fogEnable", &i32_value);
    }

    pub fn set_sky_color(&mut self, color: Color3b) {
        if self.sky_color == color { return }

        self.sky_color = color;

        self.ubo.as_ref().unwrap().update("skyColor", &color.normalized());
    }

    pub fn set_fog_color(&mut self, color: Color3b) {
        if self.fog_color == color { return }

        self.fog_color = color;

        self.ubo.as_ref().unwrap().update("fogColor", &color.normalized());
    }
    
    pub fn set_clouds_color(&mut self, color: Color3b) {
        if self.clouds_color == color { return }
        
        self.clouds_color = color;
        
        self.ubo.as_ref().unwrap().update("cloudsColor", &color.normalized());
    }
}
