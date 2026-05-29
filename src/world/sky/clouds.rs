use std::cell::RefCell;
use std::rc::Rc;
use crate::math::{Color4b, Vec2, Vec2i, Vec3};
use crate::render::{Shader, Vao};
use crate::render::vao::VaoBuffers;
use crate::resources::ResourceManager;
use crate::world::Chunk;
use crate::render::render_utils;
use crate::render::render_utils::RenderCap;


pub struct Clouds {
    shader: Option<Rc<RefCell<Shader>>>,
    vao: Vao,

    instance_buffer: Vec<Vec2>,

    clouds_chunk: Vec2i,
    first_time: bool,

    image_data: Vec<u8>,
    image_width: i32,
    image_height: i32,
}

impl Clouds {
    const MAX_CLOUDS_COUNT: usize = 1024;

    const CLOUDS_SIZE: i32 = 12;
    const SLICE_SIZE: i32 = 1;

    pub fn new() -> Self {
        Self {
            shader: None,
            vao: Vao::new(),

            instance_buffer: Vec::with_capacity(Self::MAX_CLOUDS_COUNT),

            clouds_chunk: Vec2i::ZERO,
            first_time: true,

            image_data: Vec::new(),
            image_width: 0,
            image_height: 0,
        }
    }

    pub fn start(&mut self, resource_manager: Rc<RefCell<ResourceManager>>) {
        let indices: [u32; 36] = [
            0,  1,  3,  1,  2,  3,
            4,  5,  7,  5,  6,  7,
            8,  9,  11, 9,  10, 11,
            12, 13, 15, 13, 14, 15,
            16, 17, 19, 17, 18, 19,
            20, 21, 23, 21, 22, 23,
        ];

        // vertices, normal
        let vertices: [i8; 144] = [
            // up
            1, 1, 0,   0, 1, 0,
            0, 1, 0,   0, 1, 0,
            0, 1, 1,   0, 1, 0,
            1, 1, 1,   0, 1, 0,

            // down
            1, 0, 1,   0, -1, 0,
            0, 0, 1,   0, -1, 0,
            0, 0, 0,   0, -1, 0,
            1, 0, 0,   0, -1, 0,

            // south
            0, 1, 1,   0, 0, 1,
            0, 0, 1,   0, 0, 1,
            1, 0, 1,   0, 0, 1,
            1, 1, 1,   0, 0, 1,

            // north
            1, 1, 0,   0, 0, -1,
            1, 0, 0,   0, 0, -1,
            0, 0, 0,   0, 0, -1,
            0, 1, 0,   0, 0, -1,

            // west
            0, 1, 0,  -1, 0, 0,
            0, 0, 0,  -1, 0, 0,
            0, 0, 1,  -1, 0, 0,
            0, 1, 1,  -1, 0, 0,

            // east
            1, 1, 1,   1, 0, 0,
            1, 0, 1,   1, 0, 0,
            1, 0, 0,   1, 0, 0,
            1, 1, 0,   1, 0, 0,
        ];

        self.shader = resource_manager.borrow().get_shader("clouds");
        
        let mut vao = Vao::new();
        vao.gen_vao()
           .gen_buffer(gl::ELEMENT_ARRAY_BUFFER, VaoBuffers::Ebo)
           .gen_buffer(gl::ARRAY_BUFFER, VaoBuffers::Vbo)
           .gen_buffer(gl::ARRAY_BUFFER, VaoBuffers::Instance);

        vao.buffer_data_from_arr(VaoBuffers::Ebo, &indices, gl::STATIC_DRAW);

        vao.buffer_data_from_arr(VaoBuffers::Vbo, &vertices, gl::STATIC_DRAW)
           .attrib_info(0, 3, gl::BYTE, 0, false)
           .attrib_info(1, 3, gl::BYTE, 3 * size_of::<i8>(), false)
           .set_stride(6 * size_of::<i8>());

        vao.buffer_data(VaoBuffers::Instance, size_of::<Vec2>() * Self::MAX_CLOUDS_COUNT, None, gl::DYNAMIC_DRAW)
           .attrib_info(2, 2, gl::FLOAT, 0, true)
           .set_stride(8);

        self.vao = vao;

        let image = resource_manager.borrow().read_texture("misc/clouds.png");

        self.image_width = image.width() as i32;
        self.image_height = image.height() as i32;
        self.image_data = image.into_rgba8().into_raw();
    }

    pub fn update(&mut self, player_pos: Vec3, mut render_distance: i32) {
        let last_clouds_chunk = self.clouds_chunk;
        self.clouds_chunk = Self::get_clouds_chunk(player_pos);

        // avoid update every frame
        if self.clouds_chunk == last_clouds_chunk && !self.first_time {
            return
        }

        self.first_time = false;
        self.instance_buffer.clear();

        render_distance += 2;

        let c = (render_distance as f32 * Chunk::CHUNK_SIZE.x as f32 / (Self::SLICE_SIZE as f32 * Self::CLOUDS_SIZE as f32)).ceil() as i32;
        let start = self.clouds_chunk - c;
        let end = self.clouds_chunk + c;

        // cast image pixels [u8] to [Color4b]
        let (image_prefix, color_middle, image_suffix) = unsafe { self.image_data.align_to::<Color4b>() };

        // check if cast is valid
        if !image_prefix.is_empty() || !image_suffix.is_empty() { panic!("Cannot cast images pixels [u8] to [Color4b]") }

        for x in start.x..=end.x {
            for z in start.y..=end.y {
                if self.instance_buffer.len() >= Self::MAX_CLOUDS_COUNT {
                    continue;
                }

                let mut norm_x = x.abs() % (self.image_width / Self::SLICE_SIZE);
                let mut norm_z = z.abs() % (self.image_height / Self::SLICE_SIZE);

                // invert range (0 - 16) to (16 - 0) if x or z is < that 0
                norm_x = if x < 0 { if norm_x == 0 { 0 } else { self.image_width / Self::SLICE_SIZE - norm_x } } else { norm_x };
                norm_z = if z < 0 { if norm_z == 0 { 0 } else { self.image_height / Self::SLICE_SIZE - norm_z } } else { norm_z };


                // if pixel is not transparent then draw it
                if (color_middle[Self::get_pixel_index(norm_x, norm_z)].a != 0) {
                    self.instance_buffer.push(Vec2::new(x as f32, z as f32) * Self::CLOUDS_SIZE as f32 * Self::SLICE_SIZE as f32);
                }
            }
        }

        // update instances data
        self.vao.update_buffer(VaoBuffers::Instance, &self.instance_buffer);
    }

    pub fn draw(&self) {
        //render_utils::disable(RenderCap::Blend);
        
        render_utils::draw_indexed_instanced(
           gl::TRIANGLES,
           &self.shader.as_ref().unwrap().borrow(),
           None,
           &self.vao,
           self.instance_buffer.len()
        );
        
        //render_utils::enable(RenderCap::Blend);
    }

    fn get_clouds_chunk(global_coords: Vec3) -> Vec2i {
        Vec2i {
            x: (global_coords.x / (Self::SLICE_SIZE as f32 * Self::CLOUDS_SIZE as f32)).floor() as i32,
            y: (global_coords.z / (Self::SLICE_SIZE as f32 * Self::CLOUDS_SIZE as f32)).floor() as i32,
        }
    }

    fn get_pixel_index(x: i32, z: i32) -> usize {
        x as usize + 256 * z as usize
    }
}