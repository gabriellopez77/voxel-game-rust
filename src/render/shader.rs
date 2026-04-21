use std::collections::hash_map::HashMap;
use crate::resources::path;


pub struct Shader {
    //uniforms: HashMap<String, i32>,
    id: u32,
}

impl Shader {
    pub fn bind(&self) {
        static mut CURRENT_BIND_SHADER:u32 = 0;
        
        unsafe{
            if CURRENT_BIND_SHADER == self.id { return }
            
            CURRENT_BIND_SHADER = self.id;
            
            gl::UseProgram(self.id);
        }
    }
    
    pub fn from_disk(vert_path: &str, frag_path: &str) -> Result<Self, String> {
        let full_vert_path = format!("{}{}", path::SHADER_PATH.get().unwrap(), vert_path);
        let full_frag_path = format!("{}{}", path::SHADER_PATH.get().unwrap(), frag_path);

        let vert_string_data = match std::fs::read_to_string(full_vert_path) {
                                        Ok(x) => x,
                                        Err(x) => return Err(x.to_string())
                                    };
        let frag_string_data = match std::fs::read_to_string(full_frag_path) {
                                        Ok(x) => x,
                                        Err(x) => return Err(x.to_string())
                                    };

        let vert_id = match Self::compile_shader(&vert_string_data, gl::VERTEX_SHADER) {
                            Ok(x) => x,
                            Err(x) => return Err(x)
                        };
        let frag_id = match Self::compile_shader(&frag_string_data, gl::FRAGMENT_SHADER) {
                            Ok(x) => x,
                            Err(x) => return Err(x)
                        };
        
        let id = unsafe { gl::CreateProgram() };

        unsafe { gl::AttachShader(id, vert_id); }
        unsafe { gl::AttachShader(id, frag_id); }
        unsafe { gl::LinkProgram(id);}

        unsafe { gl::DeleteShader(vert_id); }
        unsafe { gl::DeleteShader(frag_id); }

        return Ok(Shader { id });
    }

    fn compile_shader(string_data: &String, shader_type: gl::types::GLenum) -> Result<u32, String> {
        let shader_id = unsafe { gl::CreateShader(shader_type) };

        let c_string = std::ffi::CString::new(string_data.as_bytes()).unwrap();

        unsafe { gl::ShaderSource(shader_id, 1, &c_string.as_ptr(), std::ptr::null()); }
        unsafe { gl::CompileShader(shader_id); }

        let mut compile_status: i32 = 0;
        unsafe {gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compile_status);}

        if compile_status == 0 { return Err("Error to compile shader".to_string()) }

        return Ok(shader_id);
    }
}