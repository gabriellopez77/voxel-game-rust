use crate::math::{Color4b, Vec2, Vec2i, Vec3};
use crate::render::material::MaterialType;
use crate::render::{CUBE_INDICES, CLOUDS_VERTICES, CloudsVertices, GlobalRenderer, Material, Mesh};
use crate::render::core::raw_buffer::{BufferFlags, BufferResizeMode};
use crate::resources::ResourceManager;
use crate::world::Chunk;


pub struct Clouds {
    instance_data: Vec<CloudsVertices>,
    renderer: Option<(Mesh, Material)>,

    clouds_chunk: Vec2i,
    
    first_time: bool,

    image_data: Vec<u8>,
    image_width: i32,
    image_height: i32,
}

impl Clouds {
    const CLOUDS_SIZE: i32 = 12;
    const CLOUDS_SIZEF: f32 = 12.0;
    const SLICE_SIZE: i32 = 1;
    const ADDITIONAL_DISTANCE: i32 = 16;

    pub fn new() -> Self {
        Self {
            instance_data: Vec::new(),
            renderer: None,

            clouds_chunk: Vec2i::ZERO,
            
            first_time: true,

            image_data: Vec::new(),
            image_width: 0,
            image_height: 0,
        }
    }

    pub fn start(&mut self, resources: &ResourceManager, global_renderer: &mut GlobalRenderer) {
        let (mut mesh, material) = global_renderer.create_mesh_material("clouds", MaterialType::Alpha);
        mesh.set(&CLOUDS_VERTICES, &CUBE_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        mesh.create_instance_buffer(size_of::<CloudsVertices>() * 64, None, BufferFlags::VRAM | BufferFlags::RARE_UPDATE);
        self.renderer = Some((mesh, material));


        let image = resources.read_texture("misc/clouds.png");

        self.image_width = image.width() as i32;
        self.image_height = image.height() as i32;
        self.image_data = image.into_rgba8().into_raw();
    }

    pub fn cleanup(&mut self) {
        let renderer = self.renderer.as_mut().unwrap();
        renderer.0.destroy();
        renderer.1.destroy();
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


        let c = (render_distance as f32 * Chunk::CHUNK_SIZEF.x / (Self::SLICE_SIZE as f32 * Self::CLOUDS_SIZEF)).ceil() as i32;

        let start = self.clouds_chunk - (c + Self::ADDITIONAL_DISTANCE);
        let end = self.clouds_chunk + (c + Self::ADDITIONAL_DISTANCE);
  
        // cast image pixels [u8] to [Color4b]
        let (image_prefix, color_middle, image_suffix) = unsafe { self.image_data.align_to::<Color4b>() };

        // check if cast is valid
        assert!(image_prefix.is_empty() && image_suffix.is_empty(), "Cannot cast images pixels [u8] to [Color4b]");
        
        for x in start.x..=end.x {
            for z in start.y..=end.y {
                let mut norm_x = x.abs() % (self.image_width / Self::SLICE_SIZE);
                let mut norm_z = z.abs() % (self.image_height / Self::SLICE_SIZE);

                // invert range (0 - 16) to (16 - 0) if x or z is < that 0
                norm_x = if x < 0 { if norm_x == 0 { 0 } else { self.image_width / Self::SLICE_SIZE - norm_x } } else { norm_x };
                norm_z = if z < 0 { if norm_z == 0 { 0 } else { self.image_height / Self::SLICE_SIZE - norm_z } } else { norm_z };

                // if pixel is transparent then we not draw it
                if color_middle[self.get_pixel_index(norm_x, norm_z)].a == 0 { continue }


                let cullface = self.get_clouds_cullface(&color_middle, norm_x, norm_z);
                let pos = Vec2i::new(x, z) * Self::CLOUDS_SIZE * Self::SLICE_SIZE;

                self.instance_data.push(CloudsVertices {
                    position: Vec2::new(pos.x as f32, pos.y as f32),
                    cullface,
                });

            }
        }

        // update instances data
        self.renderer.as_mut().unwrap().0.update_instance_buffer(&self.instance_data, BufferResizeMode::Discard);
    }

    pub fn draw(&self, global_renderer: &mut GlobalRenderer) {
        let renderer = self.renderer.as_ref().unwrap();

        global_renderer.draw_instanced(&renderer.0, &renderer.1, self.instance_data.len());
    }

    fn get_clouds_chunk(global_coords: Vec3) -> Vec2i {
        Vec2i {
            x: (global_coords.x / (Self::SLICE_SIZE as f32 * Self::CLOUDS_SIZEF)).floor() as i32,
            y: (global_coords.z / (Self::SLICE_SIZE as f32 * Self::CLOUDS_SIZEF)).floor() as i32,
        }
    }

    fn get_pixel_index(&self, x: i32, z: i32) -> usize { (x + self.image_width * z) as usize }

    fn get_clouds_cullface(&self, data: &[Color4b], x: i32, z: i32) -> u8 {
        let mut cullface: u8 = 0b00000011;

        const NORTH_FLAG: u8 = 1 << 2;
        const SOUTH_FLAG: u8 = 1 << 3;
        const WEST_FLAG: u8 = 1 << 4;
        const EAST_FLAG: u8 = 1 << 5;

        if z > 0 { cullface |= ((data[self.get_pixel_index(x, z - 1)].a == 0) as u8) << 2 }
        else if z == 0 { cullface |= NORTH_FLAG }

        if z < self.image_height - 1 { cullface |= ((data[self.get_pixel_index(x, z + 1)].a == 0) as u8) << 3 }
        else if z == self.image_height -1 { cullface |= SOUTH_FLAG }

        if x > 0 {cullface |= ((data[self.get_pixel_index(x - 1, z)].a == 0) as u8) << 4 }
        else if x == 0 { cullface |= WEST_FLAG }

        if x < self.image_width - 1 { cullface |= ((data[self.get_pixel_index(x + 1, z)].a == 0) as u8) << 5 }
        else if x == self.image_width -1 { cullface |= EAST_FLAG }

        return cullface;
    }
}
