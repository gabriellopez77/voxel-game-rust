use std::collections::hash_map::HashMap;
use std::ffi::{c_char, CStr, CString};
use crate::math::matrix4::Matrix4;
use crate::resources::path;


pub struct Shader {
    uniforms: HashMap<&'static str, i32>,
    id: u32,
}

impl Shader {
    pub fn new() -> Self { Self { uniforms: HashMap::new(), id: 0 } }

    pub fn compile_from_disk(&mut self, vert_path: &str, frag_path: &str) -> Option<String> {
        let full_vert_path = format!("{}{}", path::SHADERS_PATH.get().unwrap(), vert_path);
        let full_frag_path = format!("{}{}", path::SHADERS_PATH.get().unwrap(), frag_path);

        let vert_string_data = match std::fs::read_to_string(&full_vert_path) {
            Ok(x) => x,
            Err(x) => return Some(x.to_string())
        };
        
        let frag_string_data = match std::fs::read_to_string(&full_frag_path) {
            Ok(x) => x,
            Err(x) => return Some(x.to_string())
        };


        let mut shader_compile_info:[c_char; 512] = [0; 512];

        
        let vert_id = match Self::compile_shader(&vert_string_data, gl::VERTEX_SHADER, &mut shader_compile_info) {
            Ok(x) => x,
            Err(x) => return Some(x)
        };
        
        let frag_id = match Self::compile_shader(&frag_string_data, gl::FRAGMENT_SHADER, &mut shader_compile_info) {
            Ok(x) => x,
            Err(x) => return Some(x)
        };
        

        let id:u32;

        unsafe {
            id = gl::CreateProgram();

            gl::AttachShader(id, vert_id);
            gl::AttachShader(id, frag_id);
            gl::LinkProgram(id);

            gl::DeleteShader(vert_id);
            gl::DeleteShader(frag_id);
        }

        self.id = id;

        return None;
    }

    pub fn uniform_matrix(&mut self, uniform: &'static str, matrix: &Matrix4) {
        if self.id == 0 { return }

        let loc = self.get_or_add_uniform(uniform);

        unsafe {gl::ProgramUniformMatrix4fv(self.id, loc, 1, gl::FALSE, matrix.as_ptr())}
    }

    pub fn bind(&self) {
        static mut CURRENT_BIND_SHADER:u32 = 0;

        unsafe{
            if CURRENT_BIND_SHADER == self.id { return }

            CURRENT_BIND_SHADER = self.id;

            gl::UseProgram(self.id);
        }
    }

    fn compile_shader(string_data: &str, shader_type: gl::types::GLenum, info: &mut [c_char; 512]) -> Result<u32, String> {
        let shader_id = unsafe { gl::CreateShader(shader_type) };

        let mut compile_status: i32 = 0;

        let c_str = CString::new(string_data).unwrap();

        unsafe {
            gl::ShaderSource(shader_id, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);

            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compile_status);

            if compile_status == 0 {
                let shader_type_string = match shader_type {
                    gl::VERTEX_SHADER => "VERTEX_SHADER",
                    gl::FRAGMENT_SHADER => "FRAGMENT_SHADER",
                    _ => "???"
                };
                
                gl::GetShaderInfoLog(shader_id, 512, std::ptr::null_mut(), info.as_mut_ptr());
                let info_string = CStr::from_ptr(info.as_ptr());
                return Err(format!("Error to compile shader: ({}):\n{}", shader_type_string, info_string.to_str().unwrap().to_string()));
            }
        }


        return Ok(shader_id);
    }

    fn get_or_add_uniform(&mut self, name: &'static str) -> i32 {
        let key = self.uniforms.entry(name).or_insert_with(|| {
            let c_str = CString::new(name).unwrap();

            return unsafe { gl::GetUniformLocation(self.id, c_str.as_ptr()) };
        });

        return *key;
    }
}