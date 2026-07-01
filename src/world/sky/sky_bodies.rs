use std::cell::RefCell;
use std::rc::Rc;
use rand::RngExt;
use crate::math;
use crate::math::{KeyFrame, Matrix4, Vec3, Vec4};
use crate::render::material::MaterialType;
use crate::render::raw_buffer::BufferFlags;
use crate::render::{CENTER_SPRITES_VERTICES, GlobalRenderer, Material, SPRITES_INDICES, SkyBodiesVertices, VulkanApp};
use crate::resources::{ResourceManager, TexCoords};
use crate::world::sky::Sky;


pub struct SkyBodies {
    stars_material: Option<Material>,
    sun_moon_material: Option<Material>,

    stars_transparency_gradient: KeyFrame<f32>,
    bodies_rotation_gradient: KeyFrame<f32>,

    matrix: Matrix4,
    stars_alpha: f32,
}

impl SkyBodies {
    const RADIUS: f32 = 10.0;
    const STARS_COUNT: usize = 1000;
    const STARS_MAX_DEGREES: f32 = 360.0f32.to_radians();

    pub fn new() -> Self {
        Self {
            stars_material: None,
            sun_moon_material: None,

            stars_transparency_gradient: KeyFrame::new(|factor, current, next| {
                current + (next - current) * factor
            }),
            bodies_rotation_gradient: KeyFrame::new(|factor, current, next| {
                current + (next - current) * factor
            }),

            matrix: Matrix4::ZERO,
            stars_alpha: 0.0,
        }
    }

    pub fn start(&mut self, resources: &ResourceManager, global_renderer: &mut GlobalRenderer) {
        // possible stars colors
        let stars_color = [
            Vec3::new(255.0, 178.0, 085.0) / 255.0,
            Vec3::new(091.0, 157.0, 255.0) / 255.0,
            Vec3::new(173.0, 255.0, 174.0) / 255.0,
            Vec3::new(248.0, 153.0, 255.0) / 255.0,
            Vec3::new(255.0, 255.0, 255.0) / 255.0
        ];

        let texture = &resources.sky_bodies_texture;

        //let stars_tex = texture.get_coords("stars").denormalized(texture.get_size());

        // all stars_textures
        //let stars_textures = [
        //    stars_tex.get_sub_tex(00.0, 0.0, 8.0, 8.0).normalized(texture.get_size()),
        //    stars_tex.get_sub_tex(08.0, 0.0, 8.0, 8.0).normalized(texture.get_size()),
        //    stars_tex.get_sub_tex(16.0, 0.0, 8.0, 8.0).normalized(texture.get_size()),
        //    stars_tex.get_sub_tex(24.0, 0.0, 8.0, 8.0).normalized(texture.get_size()),
        //    stars_tex.get_sub_tex(32.0, 0.0, 8.0, 8.0).normalized(texture.get_size()),
        //];

        let mut stars_buffer: [SkyBodiesVertices; Self::STARS_COUNT] = [
            SkyBodiesVertices {
                matrix: Matrix4::ZERO,
                uv: TexCoords::ZERO,
                color: Vec4::ZERO
            };
            Self::STARS_COUNT
        ];

        let mut rand =  rand::rng();


        // configure stars
        for i in 0..Self::STARS_COUNT {
            let dir = Vec3 {
                x: rand.random_range(-Self::STARS_MAX_DEGREES..=Self::STARS_MAX_DEGREES),
                y: rand.random_range(-Self::STARS_MAX_DEGREES..=Self::STARS_MAX_DEGREES),
                z: rand.random_range(-Self::STARS_MAX_DEGREES..=Self::STARS_MAX_DEGREES),
            };

            let mut matrix = Matrix4::IDENTITY;
            matrix.translate(dir.normalized() * Self::RADIUS);
            matrix = matrix * math::look_at_rotation(Vec3::ZERO, dir);
            matrix.scale(Vec3::new(0.04, 0.04, 0.04));

            stars_buffer[i] = SkyBodiesVertices {
                matrix,
                uv: TexCoords::ZERO,
                color: Vec4::from3(stars_color[rand.random_range(0..stars_color.len())], 1.0),
            };
        }


        let mut sun_matrix = Matrix4::IDENTITY;
        let sun_pos = Vec3::new(5.0, 0.0, 0.0);
        sun_matrix.translate(sun_pos);
        sun_matrix = sun_matrix * math::look_at_rotation(Vec3::ZERO, sun_pos);
        sun_matrix.scale(Vec3::from1(2.0));

        let mut moon_matrix = Matrix4::IDENTITY;
        let moon_pos = Vec3::new(-5.0, 0.0, 0.0);
        moon_matrix.translate(moon_pos);
        moon_matrix = moon_matrix * math::look_at_rotation(Vec3::ZERO, moon_pos);
        moon_matrix.scale(Vec3::from1(2.0));

        let sun_moon_buffer = [
            SkyBodiesVertices {
                matrix: sun_matrix,
                uv: texture.get_coords("sun"),
                color: Vec4::ZERO,
            },

            SkyBodiesVertices {
                matrix: moon_matrix,
                uv: texture.get_coords("moon"),
                color: Vec4::ZERO,
            }
        ];

        let mut stars_material = global_renderer.create_material("skyBodies", MaterialType::Sky);
        stars_material.set_mesh(&CENTER_SPRITES_VERTICES, &SPRITES_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        stars_material.create_instance_buffer_from_arr(&stars_buffer, BufferFlags::VRAM | BufferFlags::ONCE);
        self.stars_material = Some(stars_material);

        let mut sun_moon_material = global_renderer.create_material("skyBodies", MaterialType::Sky);
        sun_moon_material.set_mesh(&CENTER_SPRITES_VERTICES, &SPRITES_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        sun_moon_material.create_instance_buffer_from_arr(&sun_moon_buffer, BufferFlags::VRAM | BufferFlags::ONCE);
        self.sun_moon_material = Some(sun_moon_material);



        self.stars_transparency_gradient.frames = vec![
            ((00.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME, 1.0),
            ((04.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME, 1.0),
            ((05.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME, 0.0),
            ((18.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME, 0.0),
            ((18.0 * Sky::MINUTES_SCALE + 40.0) / Sky::CYCLE_TIME, 1.0),
            ((24.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME, 1.0)
        ];

        self.bodies_rotation_gradient.frames = vec![
            ((00.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME, -090.0),
            ((05.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME,  000.0),
            ((12.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME,  090.0),
            ((18.0 * Sky::MINUTES_SCALE + 40.0) / Sky::CYCLE_TIME,  180.0),
            ((24.0 * Sky::MINUTES_SCALE + 00.0) / Sky::CYCLE_TIME,  270.0),
        ];
    }

    pub fn update(&mut self, day_time_factor: f32) {
        let degrees = self.bodies_rotation_gradient.get(day_time_factor);
        let stars_alpha = self.stars_transparency_gradient.get(day_time_factor);

        let mut model_matrix = Matrix4::IDENTITY;
        model_matrix.rotate(degrees, 0.0, 0.0, 1.0);

        self.stars_alpha = stars_alpha;
        self.matrix = model_matrix;
    }

    pub fn cleanup(&mut self) {
        self.stars_material.as_mut().unwrap().destroy();
        self.sun_moon_material.as_mut().unwrap().destroy();
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        let stars_material = self.stars_material.as_mut().unwrap();
        let sun_moon_material = self.sun_moon_material.as_mut().unwrap();

        stars_material.update_push_constant(0, size_of::<Matrix4>(), self.matrix.as_ptr());
        stars_material.update_push_constant(size_of::<Matrix4>(), size_of::<f32>(), &self.stars_alpha);

        sun_moon_material.update_push_constant(size_of::<Matrix4>(), size_of::<f32>(), &self.stars_alpha);

        if self.stars_alpha > 0.0 {
            global_renderer.draw_obj_instanced(self.stars_material.as_ref().unwrap(), Self::STARS_COUNT);
        }

        global_renderer.draw_obj_instanced(self.sun_moon_material.as_ref().unwrap(), 2);
    }
}
