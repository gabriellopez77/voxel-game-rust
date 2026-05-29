use std::collections::HashMap;
use std::ffi::{c_char, CStr, CString};

use crate::math::{Vec3, Matrix4, Vec2};
use crate::render::render_utils;


pub struct Shader {
    uniforms: HashMap<&'static str, i32>,
    id: u32,
}

impl Shader {
    pub fn new() -> Self { Self { uniforms: HashMap::new(), id: 0 } }

    pub fn create_from_disk(folder_path: &str, relative_vert_path: &str, relative_frag_path: &str) -> Shader {
        let full_vert_path = format!("{folder_path}{relative_vert_path}");
        let full_frag_path = format!("{folder_path}{relative_frag_path}");

        let vert_string_data = match std::fs::read_to_string(&full_vert_path) {
            Ok(x) => x,
            Err(x) => panic!("{}", x.to_string())
        };

        let frag_string_data = match std::fs::read_to_string(&full_frag_path) {
            Ok(x) => x,
            Err(x) => panic!("{}", x.to_string())
        };



        let mut shader_compile_info:[c_char; 512] = [0; 512];

        let vert_id = match Self::compile_shader(&vert_string_data, gl::VERTEX_SHADER, &mut shader_compile_info) {
            Ok(x) => x,
            Err(x) => panic!("{}", x.to_string())
        };

        let frag_id = match Self::compile_shader(&frag_string_data, gl::FRAGMENT_SHADER, &mut shader_compile_info) {
            Ok(x) => x,
            Err(x) => panic!("{}", x.to_string())
        };


        let id: u32;

        unsafe {
            id = gl::CreateProgram();

            gl::AttachShader(id, vert_id);
            gl::AttachShader(id, frag_id);
            gl::LinkProgram(id);

            gl::DeleteShader(vert_id);
            gl::DeleteShader(frag_id);
        }

        return Self {
            uniforms: HashMap::new(),
            id
        }
    }

    pub fn set_matrix(&mut self, uniform: &'static str, matrix: &Matrix4) {
        if self.id == 0 { return }

        let loc = self.get_or_add_uniform(uniform);

        unsafe {gl::ProgramUniformMatrix4fv(self.id, loc, 1, gl::FALSE, matrix.as_ptr())}
    }

    pub fn set_vec3(&mut self, uniform: &'static str, vec: Vec3) {
        if self.id == 0 { return }

        let loc = self.get_or_add_uniform(uniform);

        unsafe {gl::ProgramUniform3f(self.id, loc, vec.x, vec.y, vec.z)}
    }

    pub fn set_vec2(&mut self, uniform: &'static str, vec: Vec2) {
        if self.id == 0 { return }

        let loc = self.get_or_add_uniform(uniform);

        unsafe {gl::ProgramUniform2f(self.id, loc, vec.x, vec.y)}
    }

    pub fn bind(&self) {
        render_utils::bind_shader(self.id);
    }

    fn compile_shader(string_data: &str, shader_type: gl::types::GLenum, info: &mut [c_char; 512]) -> Result<u32, String> {
        let shader_id = unsafe { gl::CreateShader(shader_type) };

        // convert shader data to c_str
        let c_str = CString::new(string_data).unwrap();

        unsafe {
            gl::ShaderSource(shader_id, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);

            let mut compile_status: i32 = 0;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compile_status);

            // if equal to zero then we have a shader compile error
            if compile_status == 0 {
                let shader_type_string = match shader_type {
                    gl::VERTEX_SHADER => "VERTEX_SHADER",
                    gl::FRAGMENT_SHADER => "FRAGMENT_SHADER",
                    _ => "???"
                };

                gl::GetShaderInfoLog(shader_id, 512, std::ptr::null_mut(), info.as_mut_ptr());

                // convert c_str to str
                let str = CStr::from_ptr(info.as_ptr()).to_str();

                if let Ok(str) = str {
                    return Err(format!("Error to compile shader: ({shader_type_string}):\n{:?}", str.to_string()))
                }

                return Err(format!("Error to Compile: ({shader_type_string}):\nError to convert info long to UTF-8"));
            }
        }


        return Ok(shader_id);
    }

    fn get_or_add_uniform(&mut self, name: &'static str) -> i32 {
        return *self.uniforms.entry(name).or_insert_with(|| {
            let c_str = CString::new(name).unwrap();

            return unsafe { gl::GetUniformLocation(self.id, c_str.as_ptr()) };
        });
    }
}
