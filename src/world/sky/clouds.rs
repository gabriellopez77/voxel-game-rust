use crate::math::{Color4b, Vec2, Vec2i, Vec3};
use crate::render::material::MaterialType;
use crate::render::{CUBE_INDICES, CUBE_VERTICES, CloudsVertices, GlobalRenderer, Material};
use crate::render::raw_buffer::BufferFlags;
use crate::resources::ResourceManager;
use crate::world::Chunk;


pub struct Clouds {
    instance_data: Vec<CloudsVertices>,
    material: Option<Material>,

    clouds_chunk: Vec2i,
    first_time: bool,

    image_data: Vec<u8>,
    image_width: i32,
    image_height: i32,
}

impl Clouds {
    const MAX_CLOUDS_COUNT: usize = 2048;

    const CLOUDS_SIZE: i32 = 12;
    const SLICE_SIZE: i32 = 1;
    const ADDITIONAL_DISTANCE: i32 = 16;

    pub fn new() -> Self {
        Self {
            instance_data: Vec::with_capacity(Self::MAX_CLOUDS_COUNT),
            material: None,

            clouds_chunk: Vec2i::ZERO,
            first_time: true,

            image_data: Vec::new(),
            image_width: 0,
            image_height: 0,
        }
    }

    pub fn start(&mut self, resources: &ResourceManager, global_renderer: &mut GlobalRenderer) {
        let mut material = global_renderer.create_material("clouds", MaterialType::Alpha);
        material.set_mesh(&CUBE_VERTICES, &CUBE_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        material.create_instance_buffer(size_of::<CloudsVertices>() * Self::MAX_CLOUDS_COUNT, None, BufferFlags::VRAM);
        self.material = Some(material);


        let image = resources.read_texture("misc/clouds.png");

        self.image_width = image.width() as i32;
        self.image_height = image.height() as i32;
        self.image_data = image.into_rgba8().into_raw();
    }

    pub fn cleanup(&mut self) {
        self.material.as_mut().unwrap().destroy();
    }

    pub fn update(&mut self, player_pos: Vec3, render_distance: i32) {
        let last_clouds_chunk = self.clouds_chunk;
        self.clouds_chunk = Self::get_clouds_chunk(player_pos);

        // avoid update every frame
        if self.clouds_chunk == last_clouds_chunk && !self.first_time {
            return
        }

        self.first_time = false;
        self.instance_data.clear();


        let c = (render_distance as f32 * Chunk::CHUNK_SIZEF.x / (Self::SLICE_SIZE as f32 * Self::CLOUDS_SIZE as f32)).ceil() as i32;

        let start = self.clouds_chunk - c - Self::ADDITIONAL_DISTANCE;
        let end = self.clouds_chunk + c + Self::ADDITIONAL_DISTANCE;

        // cast image pixels [u8] to [Color4b]
        let (image_prefix, color_middle, image_suffix) = unsafe { self.image_data.align_to::<Color4b>() };

        // check if cast is valid
        if !image_prefix.is_empty() || !image_suffix.is_empty() { panic!("Cannot cast images pixels [u8] to [Color4b]") }


        for x in start.x..=end.x {
            for z in start.y..=end.y {
                if self.instance_data.len() >= Self::MAX_CLOUDS_COUNT {
                    continue;
                }

                let mut norm_x = x.abs() % (self.image_width / Self::SLICE_SIZE);
                let mut norm_z = z.abs() % (self.image_height / Self::SLICE_SIZE);

                // invert range (0 - 16) to (16 - 0) if x or z is < that 0
                norm_x = if x < 0 { if norm_x == 0 { 0 } else { self.image_width / Self::SLICE_SIZE - norm_x } } else { norm_x };
                norm_z = if z < 0 { if norm_z == 0 { 0 } else { self.image_height / Self::SLICE_SIZE - norm_z } } else { norm_z };

                // if pixel is transparent then we not draw it
                if color_middle[Self::get_pixel_index(norm_x, norm_z)].a == 0 { continue }


                let cullface = self.get_clouds_cullface(&color_middle, norm_x, norm_z);
                let pos = Vec2i::new(x, z) * Self::CLOUDS_SIZE * Self::SLICE_SIZE;

                self.instance_data.push(CloudsVertices {
                    position: Vec2::new(pos.x as f32, pos.y as f32),
                    cullface,
                });

            }
        }

        // update instances data
        self.material.as_mut().unwrap().update_instance_data2(&self.instance_data);
    }

    pub fn draw(&self, global_renderer: &mut GlobalRenderer) {
        global_renderer.draw_obj_instanced(self.material.as_ref().unwrap(), self.instance_data.len());
    }

    fn get_clouds_chunk(global_coords: Vec3) -> Vec2i {
        Vec2i {
            x: (global_coords.x / (Self::SLICE_SIZE as f32 * Self::CLOUDS_SIZE as f32)).floor() as i32,
            y: (global_coords.z / (Self::SLICE_SIZE as f32 * Self::CLOUDS_SIZE as f32)).floor() as i32,
        }
    }

    fn get_pixel_index(x: i32, z: i32) -> usize { x as usize + 256 * z as usize }

    fn get_clouds_cullface(&self, data: &[Color4b], x: i32, z: i32) -> u8 {
        let mut cullface = 0;

        const NORTH_FLAG: u8 = 1;
        const SOUTH_FLAG: u8 = 1 << 1;
        const WEST_FLAG: u8 = 1 << 2;
        const EAST_FLAG: u8 = 1 << 3;

        if z > 0 {
            let north = (data[Self::get_pixel_index(x, z - 1)].a == 0) as u8;
            cullface |= north;
        }
        else if z == 0 { cullface |= NORTH_FLAG }

        if z < self.image_height - 1 {
            let south = (data[Self::get_pixel_index(x, z + 1)].a == 0) as u8;
            cullface |= south << 1;
        }
        else if z == self.image_height -1 { cullface |= SOUTH_FLAG }

        if x > 0 {
            let west = (data[Self::get_pixel_index(x - 1, z)].a == 0) as u8;
            cullface |= west << 2;
        }
        else if x == 0 { cullface |= WEST_FLAG }

        if x < self.image_width - 1 {
            let east = (data[Self::get_pixel_index(x + 1, z)].a == 0) as u8;
            cullface |= east << 3;
        }
        else if x == self.image_width -1 { cullface |= EAST_FLAG }

        return cullface;
    }
}
