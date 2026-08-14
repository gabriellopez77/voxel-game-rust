use crate::resources::{ResourceManager, resources_manager};
use crate::math::{Color3b, KeyFrame};
use crate::render::{GlobalRenderer, Material, Mesh, material};
use crate::render::core::raw_buffer::BufferFlags;
use crate::world::Chunk;
use crate::world::player::Camera;
use crate::world::sky::{Clouds, SkyBodies};


pub struct Sky {
    renderer: Option<(Mesh, Material)>,

    sky_bodies: SkyBodies,
    clouds: Clouds,

    pub fog_enabled: i32,
    pub fog_norm_distance: f32,
    pub fog_distance: f32,
    pub fog_density: f32,
    pub fog_color: Color3b,
    pub sky_color: Color3b,
    pub clouds_color: Color3b,

    sky_color_gradient: KeyFrame<Color3b>,
    fog_color_gradient: KeyFrame<Color3b>,
    clouds_color_gradient: KeyFrame<Color3b>,

    pub time: f32,

    update_delay: f32,

    underwater_fog_distance: f32,
    underwater_fog_density: f32,
    underwater_fog_color: Color3b,
}

impl Sky {
    pub const MINUTES_SCALE: f32 = 60.0;
    pub const HOURS_SCALE: f32 = 24.0;
    const UPDATE_DELAY: f32 = 0.3;

    pub const CYCLE_TIME: f32 = Self::MINUTES_SCALE * Self::HOURS_SCALE;
    pub const TIME_MORNING: f32 = Self::MINUTES_SCALE * 6.0 + 30.0; // 6:30

    pub fn new() -> Self {
        Self {
            renderer: None,

            sky_bodies: SkyBodies::new(),
            clouds: Clouds::new(),

            fog_enabled: 0,
            fog_norm_distance: 0.0,
            fog_distance: 0.0,
            fog_density: 0.0,
            fog_color: Color3b::ZERO,
            sky_color: Color3b::ZERO,
            clouds_color: Color3b::ZERO,

            sky_color_gradient: KeyFrame::new(|factor, current, next| {
                let r = current.r as f32 + (next.r as f32 - current.r as f32) * factor;
                let g = current.g as f32 + (next.g as f32 - current.g as f32) * factor;
                let b = current.b as f32 + (next.b as f32 - current.b as f32) * factor;

                return Color3b::new(r as u8, g as u8, b as u8);
            }),

            fog_color_gradient: KeyFrame::new(|factor, current, next| {
                let r = current.r as f32 + (next.r as f32 - current.r as f32) * factor;
                let g = current.g as f32 + (next.g as f32 - current.g as f32) * factor;
                let b = current.b as f32 + (next.b as f32 - current.b as f32) * factor;

                return Color3b::new(r as u8, g as u8, b as u8);
            }),

            clouds_color_gradient: KeyFrame::new(|factor, current, next| {
                let r = current.r as f32 + (next.r as f32 - current.r as f32) * factor;
                let g = current.g as f32 + (next.g as f32 - current.g as f32) * factor;
                let b = current.b as f32 + (next.b as f32 - current.b as f32) * factor;

                return Color3b::new(r as u8, g as u8, b as u8);
            }),

            time: Self::TIME_MORNING,

            update_delay: 0.0,

            underwater_fog_distance: 0.2,
            underwater_fog_density: 0.7,
            underwater_fog_color: Color3b::new(24, 106, 178),
        }
    }
    pub fn start(&mut self, resources_manager: &ResourceManager, global_renderer: &mut GlobalRenderer) {
        let (vertices, indices) = ResourceManager::gen_sphere(16.0, 16.0);

        let (mut mesh, material) = global_renderer.create_mesh_material("skyDome", material::MaterialType::Sky);
        mesh.set(&vertices, &indices, BufferFlags::VRAM | BufferFlags::ONCE);
        self.renderer = Some((mesh, material));

        self.set_fog(true);


        self.sky_color_gradient.set_frames(vec![
            (00.0 * Self::MINUTES_SCALE + 00.0, Color3b::from_hex(0x010003)),
            (04.0 * Self::MINUTES_SCALE + 30.0, Color3b::from_hex(0x010003)),
            (06.0 * Self::MINUTES_SCALE + 00.0, Color3b::from_hex(0x055EFF)),
            (17.0 * Self::MINUTES_SCALE + 20.0, Color3b::from_hex(0x055EFF)),
            (18.0 * Self::MINUTES_SCALE + 40.0, Color3b::from_hex(0x020203)),
            (24.0 * Self::MINUTES_SCALE + 00.0, Color3b::from_hex(0x020203)),
        ]);

        self.fog_color_gradient.set_frames(vec![
            (00.0 * Self::MINUTES_SCALE + 00.0, Color3b::from_hex(0x010E22) ),
            (04.0 * Self::MINUTES_SCALE + 30.0, Color3b::from_hex(0x010E22) ),
            (06.0 * Self::MINUTES_SCALE + 00.0, Color3b::from_hex(0xD9CCC3) ),
            (07.0 * Self::MINUTES_SCALE + 00.0, Color3b::from_hex(0x80CCFF) ),
            (17.0 * Self::MINUTES_SCALE + 00.0, Color3b::from_hex(0x80CCFF) ),
            (17.0 * Self::MINUTES_SCALE + 50.0, Color3b::from_hex(0xFF9849) ),
            (18.0 * Self::MINUTES_SCALE + 40.0, Color3b::from_hex(0x415066) ),
            (19.0 * Self::MINUTES_SCALE + 00.0, Color3b::from_hex(0x010E22) ),
            (24.0 * Self::MINUTES_SCALE + 00.0, Color3b::from_hex(0x010E22) ),
        ]);

        self.clouds_color_gradient.set_frames(vec![
            (00.0 * Sky::MINUTES_SCALE + 00.0, Color3b::from_hex(0x0E0F18)),
            (04.0 * Sky::MINUTES_SCALE + 30.0, Color3b::from_hex(0x0E0F18)),
            (06.0 * Sky::MINUTES_SCALE + 00.0, Color3b::from_hex(0xFFFFFF)),
            (07.0 * Sky::MINUTES_SCALE + 00.0, Color3b::from_hex(0xFFFFFF)),
            (17.0 * Sky::MINUTES_SCALE + 00.0, Color3b::from_hex(0xFFFFFF)),
            (17.0 * Sky::MINUTES_SCALE + 50.0, Color3b::from_hex(0xFFFFFF)),
            (19.0 * Sky::MINUTES_SCALE + 00.0, Color3b::from_hex(0x0E0F18)),
            (24.0 * Sky::MINUTES_SCALE + 00.0, Color3b::from_hex(0x0E0F18)),
        ]);

        self.sky_bodies.start(resources_manager, global_renderer);
        self.clouds.start(resources_manager, global_renderer);

        self.set_sky_color(self.sky_color_gradient.get(0.0));
        self.set_fog_color(self.fog_color_gradient.get(0.0));
        self.set_clouds_color(self.clouds_color_gradient.get(0.0));
    }

    pub fn cleanup(&mut self) {
        let renderer = self.renderer.as_mut().unwrap();
        renderer.0.destroy();
        renderer.1.destroy();

        self.sky_bodies.cleanup();
        self.clouds.cleanup();
    }

    pub fn update(&mut self, dt: f32, camera: &Camera, render_distance: i32) {
        self.time += dt;
        self.update_delay += dt;

        if self.time > Self::CYCLE_TIME { self.time = 0.0 }

        if self.update_delay > Self::UPDATE_DELAY {

            self.sky_bodies.update(self.time);

            self.update_delay = 0.0;
        }

        self.set_clouds_color(self.clouds_color_gradient.get(self.time));

        if camera.is_underwater {
            self.set_sky_color(self.underwater_fog_color);
            self.set_fog_color(self.underwater_fog_color);
            self.set_fog_distance(self.underwater_fog_distance);
            self.set_fog_density(self.underwater_fog_density);
        }
        else {
            self.set_sky_color(self.sky_color_gradient.get(self.time));
            self.set_fog_color(self.fog_color_gradient.get(self.time));
            self.set_fog_distance(render_distance as f32 - 0.5);
            self.set_fog_density(16.0);
        }

        self.clouds.update(camera.get_pos(), render_distance);
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        // draw sky dome
        let renderer = self.renderer.as_mut().unwrap();
        global_renderer.draw(&renderer.0, &mut renderer.1);

        // draw stars, sun, moon and clouds
        self.sky_bodies.draw(global_renderer);
        self.clouds.draw(global_renderer);
    }

    pub fn set_fog_distance(&mut self, distance: f32) {
        self.fog_distance = distance;
        self.fog_norm_distance = 1.0 / Chunk::CHUNK_SIZEF.x / distance;
    }

    pub fn set_fog_density(&mut self, density: f32) {
        self.fog_density = density;
    }

    pub fn set_fog(&mut self, value: bool) {
        self.fog_enabled = value as i32;
    }

    pub fn set_sky_color(&mut self, color: Color3b) {
        self.sky_color = color;
    }

    pub fn set_fog_color(&mut self, color: Color3b) {
        self.fog_color = color;
    }

    pub fn set_clouds_color(&mut self, color: Color3b) {
        self.clouds_color = color;
    }
}
