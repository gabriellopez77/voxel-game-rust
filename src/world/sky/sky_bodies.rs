use std::cell::RefCell;
use std::rc::Rc;
use crate::math::{KeyFrame, Matrix4, Vec3};
use crate::render::{Shader, SkyBodiesRenderer, Texture};
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
    const RADIUS: f32 = 5.0;
    const STARS_COUNT: i32 = 500;
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

        // stores stars tex coords
        let stars_tex = self.texture.as_ref().unwrap().get_coords("stars");
        let start_tex_non_normalized = stars_tex.denormalized(self.texture.as_ref().unwrap().get_size());

        //let stars_textures = [
        //    TexCoords::new(00.0, 0.0, 08.0, 8.0) + start_tex_non_normalized,
        //    TexCoords::new(08.0, 0.0, 16.0, 8.0) + start_tex_non_normalized,
        //    TexCoords::new(16.0, 0.0, 24.0, 8.0) + start_tex_non_normalized,
        //    TexCoords::new(24.0, 0.0, 32.0, 8.0) + start_tex_non_normalized,
        //    TexCoords::new(32.0, 0.0, 40.0, 8.0) + start_tex_non_normalized,
        //];

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

    pub fn update(&mut self, dt: f32, day_time: f32, camera_pos: Vec3) {
        let degrees = self.bodies_rotation_gradient.get(day_time);
        let stars_alpha = self.stars_transparency_gradient.get(day_time);

        let mut model_matrix = Matrix4::IDENTITY;
        model_matrix.translate(camera_pos);
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