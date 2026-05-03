use gl::types::GLenum;

use crate::{resources::ArrayBuffer};


pub struct Vao {
    pub triangles_count: i32,
    
    id: u32,
    binding_index: u32,
    binding_buffer: VaoBuffers,
    buffers: [(GLenum, u32, u32); 3],
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum VaoBuffers {
    Vbo,
    Ebo,
    Instance,
}

impl Vao {
    pub fn new() -> Self { 
        Self {
            triangles_count: 0,
            id: 0,
            binding_index: 0,
            binding_buffer: VaoBuffers::Vbo,
            buffers: [(0, 0, 0); 3]
        } 
    }
    
    pub fn gen_vao(&mut self) -> &mut Vao {
        let mut id: u32 = 0;
        unsafe {gl::CreateVertexArrays(1, &mut id) }
        
        self.id = id;

        return self;
    }
    
    pub fn bind(&self) {
        static mut CURRENT_BIND_VAO_ID: u32 = 0;

        unsafe {
            if CURRENT_BIND_VAO_ID == self.id { return }

            CURRENT_BIND_VAO_ID = self.id;
            gl::BindVertexArray(self.id);
        }
    }

    pub fn gen_buffer(&mut self, buffer_type: GLenum, vao_buffer: VaoBuffers) -> &mut Vao {
        let mut buffer_id: u32 = 0;
        unsafe {gl::CreateBuffers(1, &mut buffer_id) }

        self.buffers[vao_buffer as usize] = (buffer_type, buffer_id, self.binding_index);

        if !matches!(vao_buffer, VaoBuffers::Ebo) {
            self.binding_index += 1
        }

        return self;
    }

    pub fn buffer_data_from_arr<T>(&mut self, vao_buffer: VaoBuffers, data: &[T], usage: GLenum) -> &Vao {
        return self.buffer_data(vao_buffer, size_of_val(data), Some(data.as_ptr() as *const ()), usage);
    }

    pub fn buffer_data(&mut self, buffer: VaoBuffers, size: usize, data: Option<*const ()>, usage: GLenum) -> &Vao {
        let mut is_ebo: bool = false;
        let (_, buffer_id, buffer_binding_index) = self.buffers[buffer as usize];

        let p = match data {
            Some(data) => data as *const std::ffi::c_void,
            _ => std::ptr::null(),
        };

        // if buffer is ebo then calculate triangles count
        if matches!(buffer, VaoBuffers::Ebo) {
            is_ebo = true;
            self.triangles_count = size as i32 / 4;
        }
        else {
            self.binding_index = buffer_binding_index;
        }

        unsafe {
            gl::NamedBufferData(buffer_id, size as isize, p, usage);

            if is_ebo {
                gl::VertexArrayElementBuffer(self.id, buffer_id)
            }
        }

        self.binding_buffer = buffer;

        return self;
    }

    pub fn update_buffer<T: Default + Copy, const SIZE: usize>(&mut self, buffer: VaoBuffers, arr: &ArrayBuffer<T, SIZE>) {
        unsafe {
            gl::NamedBufferSubData(self.buffers[buffer as usize].1, 0, arr.len_bytes(), arr.as_ptr())
        }
    }

    pub fn attrib_info(&self, index: u32, size: i32, attrib_type: GLenum, offset: usize, instance: bool) -> &Vao{
        let vao = self.id;
        
        unsafe {
            gl::EnableVertexArrayAttrib(vao, index);
            gl::VertexArrayAttribBinding(vao, index, self.binding_index);
            gl::VertexArrayAttribFormat(vao, index, size, attrib_type, gl::FALSE, offset as u32);
            
            if instance {
                gl::VertexArrayBindingDivisor(vao, self.binding_index, 1)
            }
        }

        return self;
    }

    pub fn set_stride(&self, stride: usize) {
        unsafe {
            let buffer_id = self.buffers[self.binding_buffer as usize].1;
            
            gl::VertexArrayVertexBuffer(self.id, self.binding_index, buffer_id, 0, stride as i32);
        }
    }
}