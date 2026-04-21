mod render;
mod resources;
mod window;
mod inputs;
mod math;


use crate::render::shader::Shader;
use crate::render::vao::*;

use crate::resources::path;

fn main() {
    let mut window = window::Window::init(800, 600, "My First Rust Window");

    // init paths
    path::SHADER_PATH.get_or_init(|| std::env::current_dir().unwrap().display().to_string() + r"\assets\shaders");

    let shader = Shader::from_disk(r"\vert.glsl", r"\frag.glsl").expect("Failed to compile shader");

    window::Window::set_window_instance(&mut window);

    let vertices: [f32; _] = [
        1.0, 1.0,  1.0, 1.0, 0.0, // top right
        1.0, 0.0,  1.0, 0.0, 0.0, // bottom right
        0.0, 0.0,  0.0, 0.0, 1.0, // bottom left
        0.0, 1.0,  0.0, 1.0, 0.0, // top left
    ];

    let indices: [u32; _] = [ 0, 1, 2, 2, 3, 0 ];


    let mut vao = Vao::new();
    vao.gen_buffer(gl::ELEMENT_ARRAY_BUFFER, VaoBuffers::Ebo);
    vao.gen_buffer(gl::ARRAY_BUFFER, VaoBuffers::Vbo);

    vao.bind();

    vao.bind_buffer(VaoBuffers::Ebo);
    vao.buffer_data(VaoBuffers::Ebo, &indices, gl::STATIC_DRAW);

    vao.bind_buffer(VaoBuffers::Vbo);
    vao.buffer_data(VaoBuffers::Vbo, &vertices, gl::STATIC_DRAW);

    vao.attrib_pointer(0, 2, gl::FLOAT, 5 * size_of::<f32>(), 0, false);
    vao.attrib_pointer(1, 3, gl::FLOAT, 5 * size_of::<f32>(), 2 * size_of::<f32>(), false);



    window.run();

    //while !window.should_close() {
    //    glfw.poll_events();
//
    //    vao.bind();
    //    shader.bind();
//
    //    render::utils::draw_indexed(gl::TRIANGLES, vao.triangles_count);
//
    //    unsafe {
    //        if gl::GetError() != gl::NO_ERROR { panic!() }
    //    }
//
    //    window.swap_buffers();
    //}
}

