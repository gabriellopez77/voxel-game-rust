use rand::RngExt;
use crate::math;
use crate::math::{KeyFrame, Matrix4, Vec3, Vec4};
use crate::render::material::MaterialType;
use crate::render::core::raw_buffer::BufferFlags;
use crate::render::{CENTER_SPRITES_VERTICES, GlobalRenderer, Material, Mesh, SPRITES_INDICES, SkyBodiesVertices};
use crate::resources::{ResourceManager, TexCoords};
use crate::world::sky::Sky;


pub struct SkyBodies {
    stars_renderer: Option<(Mesh, Material)>,
    sun_moon_renderer: Option<(Mesh, Material)>,

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
            stars_renderer: None,
            sun_moon_renderer: None,

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
            matrix.translatev(dir.normalized() * Self::RADIUS);
            matrix = matrix * math::look_at_rotation(Vec3::ZERO, dir);
            matrix.scale(0.04, 0.04, 0.04);

            stars_buffer[i] = SkyBodiesVertices {
                matrix,
                uv: TexCoords::ZERO,
                color: Vec4::from3(stars_color[rand.random_range(0..stars_color.len())], 1.0),
            };
        }


        let mut sun_matrix = Matrix4::IDENTITY;
        let sun_pos = Vec3::new(5.0, 0.0, 0.0);
        sun_matrix.translatev(sun_pos);
        sun_matrix = sun_matrix * math::look_at_rotation(Vec3::ZERO, sun_pos);
        sun_matrix.scale(2.0, 2.0, 2.0);

        let mut moon_matrix = Matrix4::IDENTITY;
        let moon_pos = Vec3::new(-5.0, 0.0, 0.0);
        moon_matrix.translatev(moon_pos);
        moon_matrix = moon_matrix * math::look_at_rotation(Vec3::ZERO, moon_pos);
        moon_matrix.scale(2.0, 2.0, 2.0);

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

        let (mut stars_mesh, stars_material) = global_renderer.create_mesh_material("skyBodies", MaterialType::Sky);
        stars_mesh.set(&CENTER_SPRITES_VERTICES, &SPRITES_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        stars_mesh.create_instance_buffer_from_arr(&stars_buffer, BufferFlags::VRAM | BufferFlags::ONCE);
        self.stars_renderer = Some((stars_mesh, stars_material));

        let (mut sun_moon_mesh, sun_moon_material) = global_renderer.create_mesh_material("skyBodies", MaterialType::Sky);
        sun_moon_mesh.set(&CENTER_SPRITES_VERTICES, &SPRITES_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        sun_moon_mesh.create_instance_buffer_from_arr(&sun_moon_buffer, BufferFlags::VRAM | BufferFlags::ONCE);
        self.sun_moon_renderer = Some((sun_moon_mesh, sun_moon_material));



        self.stars_transparency_gradient.set_frames(vec![
            (00.0 * Sky::MINUTES_SCALE + 00.0, 1.0),
            (04.0 * Sky::MINUTES_SCALE + 00.0, 1.0),
            (05.0 * Sky::MINUTES_SCALE + 00.0, 0.0),
            (18.0 * Sky::MINUTES_SCALE + 00.0, 0.0),
            (18.0 * Sky::MINUTES_SCALE + 40.0, 1.0),
            (24.0 * Sky::MINUTES_SCALE + 00.0, 1.0)
        ]);

        self.bodies_rotation_gradient.set_frames(vec![
            (00.0 * Sky::MINUTES_SCALE + 00.0, -090.0),
            (05.0 * Sky::MINUTES_SCALE + 00.0,  000.0),
            (12.0 * Sky::MINUTES_SCALE + 00.0,  090.0),
            (18.0 * Sky::MINUTES_SCALE + 40.0,  180.0),
            (24.0 * Sky::MINUTES_SCALE + 00.0,  270.0),
        ]);
    }

    pub fn update(&mut self, day_time: f32) {
        let degrees = self.bodies_rotation_gradient.get(day_time);
        let stars_alpha = self.stars_transparency_gradient.get(day_time);

        let mut model_matrix = Matrix4::IDENTITY;
        model_matrix.rotate(degrees, 0.0, 0.0, 1.0);

        self.stars_alpha = stars_alpha;
        self.matrix = model_matrix;
    }

    pub fn cleanup(&mut self) {
        let stars_renderer = self.stars_renderer.as_mut().unwrap();
        stars_renderer.0.destroy();
        stars_renderer.1.destroy();

        let sun_moon_renderer = self.sun_moon_renderer.as_mut().unwrap();
        sun_moon_renderer.0.destroy();
        sun_moon_renderer.1.destroy();
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        if self.stars_alpha > 0.0 {
            global_renderer.set_push_constant(0, &self.matrix);
            global_renderer.set_push_constant(size_of::<Matrix4>(), &self.stars_alpha);

            let renderer = self.stars_renderer.as_mut().unwrap();
            global_renderer.draw_instanced(&renderer.0, &mut renderer.1, Self::STARS_COUNT);
        }

        global_renderer.set_push_constant(0, &self.matrix);
        global_renderer.set_push_constant(size_of::<Matrix4>(), &self.stars_alpha);

        let renderer = self.sun_moon_renderer.as_mut().unwrap();
        global_renderer.draw_instanced(&renderer.0, &mut renderer.1, Self::STARS_COUNT);
    }
}
