use std::cell::RefCell;
use std::rc::Rc;
use rand::RngExt;
use crate::math;
use crate::math::{KeyFrame, Matrix4, Vec3, Vec4};
use crate::render::{Shader, SkyBodiesRenderer, SkyBodiesVertices, Texture};
use crate::resources::{ResourceManager, TexCoords};
use crate::world::sky::Sky;


pub struct SkyBodies {
    shader: Option<Rc<RefCell<Shader>>>,
    texture: Option<Rc<Texture>>,

    stars_renderer: SkyBodiesRenderer,
    sun_mon_renderer: SkyBodiesRenderer,

    stars_transparency_gradient: KeyFrame<f32>,
    bodies_rotation_gradient: KeyFrame<f32>,

    stars_alpha: f32,
}

impl SkyBodies {
    const RADIUS: f32 = 10.0;
    const STARS_COUNT: usize = 1000;
    const STARS_MAX_DEGREES: f32 = 360.0f32.to_radians();

    pub fn new() -> Self {
        Self {
            shader: None,
            texture: None,

            stars_renderer: SkyBodiesRenderer::new(),
            sun_mon_renderer: SkyBodiesRenderer::new(),

            stars_transparency_gradient: KeyFrame::new(|factor, current, next| {
                current + (next - current) * factor
            }),
            bodies_rotation_gradient: KeyFrame::new(|factor, current, next| {
                current + (next - current) * factor
            }),

            stars_alpha: 0.0,
        }
    }

    pub fn start(&mut self, resources: &ResourceManager) {
        self.shader = resources.get_shader("skyBodies");
        self.texture = resources.get_texture("skyBodies");

        // possible stars colors
        let stars_color = [
            Vec3::new(255.0, 178.0, 085.0) / 255.0,
            Vec3::new(091.0, 157.0, 255.0) / 255.0,
            Vec3::new(173.0, 255.0, 174.0) / 255.0,
            Vec3::new(248.0, 153.0, 255.0) / 255.0,
            Vec3::new(255.0, 255.0, 255.0) / 255.0
        ];

        let texture = self.texture.as_ref().unwrap();

        let stars_tex = texture.get_coords("stars").denormalized(texture.get_size());

        // all stars_textures
        let stars_textures = [
            stars_tex.get_sub_tex(00.0, 0.0, 8.0, 8.0).normalized(texture.get_size()),
            stars_tex.get_sub_tex(08.0, 0.0, 8.0, 8.0).normalized(texture.get_size()),
            stars_tex.get_sub_tex(16.0, 0.0, 8.0, 8.0).normalized(texture.get_size()),
            stars_tex.get_sub_tex(24.0, 0.0, 8.0, 8.0).normalized(texture.get_size()),
            stars_tex.get_sub_tex(32.0, 0.0, 8.0, 8.0).normalized(texture.get_size()),
        ];

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

        self.stars_renderer.start(&stars_buffer);


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

        let sun_moon_buffer =[
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

        self.sun_mon_renderer.start(&sun_moon_buffer);


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

        self.shader.as_ref().unwrap().borrow_mut().set_matrix("model", &model_matrix);
        self.shader.as_ref().unwrap().borrow_mut().set_1f("alpha", stars_alpha);
    }

    pub fn draw(&self) {
        let shader = self.shader.as_ref().unwrap();
        let texture = self.texture.as_ref().unwrap();

        if self.stars_alpha > 0.0 {
            self.stars_renderer.draw(shader, texture);
        }

        self.sun_mon_renderer.draw(shader, texture);
    }
}