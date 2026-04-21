use crate::math::matrix4::Matrix4;
use crate::math::vec3::Vec3;
use crate::render::shader::Shader;
use crate::render::vao::{Vao, VaoBuffers};
use crate::render;
use crate::world::player::camera::Camera;
use crate::inputs;

pub struct Game {
    vao: Vao,
    shader: Shader,

    camera: Camera,
}

impl Game {
    pub fn new() -> Self { Self { vao: Vao::new(), shader: Shader::new(), camera: Camera::new() } }

    pub fn start(&mut self) {
        const VERTICES: [f32; 20] = [
            1.0, 1.0,  1.0, 1.0, 0.0, // top right
            1.0, 0.0,  1.0, 0.0, 0.0, // bottom right
            0.0, 0.0,  0.0, 0.0, 1.0, // bottom left
            0.0, 1.0,  0.0, 1.0, 0.0, // top left
        ];

        const INDICES: [u32; 6] = [ 0, 1, 2, 2, 3, 0 ];

        self.vao = Vao::new();
        self.vao.gen_vao();
        self.vao.gen_buffer(gl::ELEMENT_ARRAY_BUFFER, VaoBuffers::Ebo);
        self.vao.gen_buffer(gl::ARRAY_BUFFER, VaoBuffers::Vbo);

        self.vao.bind();

        self.vao.bind_buffer(VaoBuffers::Ebo);
        self.vao.buffer_data(VaoBuffers::Ebo, &INDICES, gl::STATIC_DRAW);

        self.vao.bind_buffer(VaoBuffers::Vbo);
        self.vao.buffer_data(VaoBuffers::Vbo, &VERTICES, gl::STATIC_DRAW);

        self.vao.attrib_pointer(0, 2, gl::FLOAT, 5 * size_of::<f32>(), 0, false);
        self.vao.attrib_pointer(1, 3, gl::FLOAT, 5 * size_of::<f32>(), 2 * size_of::<f32>(), false);
        
        match self.shader.compile_from_disk(r"\vert.glsl", r"\frag.glsl") {
            Some(x) => panic!("{}", x),
            None => ()
        };
    }

    pub fn update(&mut self, dt: f32) {
        let mut dir = Vec3::ZERO;

        let yaw = self.camera.rotation.x.to_radians();
        let front = Vec3 { x: yaw.cos(), y: 0.0, z: yaw.sin() };

        if inputs::is_key_down(inputs::Keys::W) { dir = dir + front };
        if inputs::is_key_down(inputs::Keys::A) { dir = dir - front.cross(Vec3 { x: 0.0, y: 1.0, z: 0.0 }) };
        if inputs::is_key_down(inputs::Keys::S) { dir = dir - front };
        if inputs::is_key_down(inputs::Keys::D) { dir = dir + front.cross(Vec3 { x: 0.0, y: 1.0, z: 0.0 }) };
        if inputs::is_key_down(inputs::Keys::LeftShift) { dir.y -= 1.0 };
        if inputs::is_key_down(inputs::Keys::Space) { dir.y += 1.0 };

        const SPEED: f32 = 10.0;

        if dir.length() > 1.0 { dir = dir.normalized()}

        self.camera.position += dir * (SPEED * dt);

        self.camera.update();

        self.shader.uniform_matrix("view", &self.camera.view_matrix);
    }

    pub fn render(&mut self) {
        self.vao.bind();
        self.shader.bind();
    
        render::utils::draw_indexed(gl::TRIANGLES, self.vao.triangles_count);
    }

    pub fn resize(&mut self,width: i32, height: i32) {
        let projection = Matrix4::perspective(70.0, width as f32 / height as f32, 0.1, 100.0);

        self.shader.uniform_matrix("projection", &projection);
    }
}