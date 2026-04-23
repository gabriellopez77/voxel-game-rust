use gl::types::GLenum;
use crate::resources;
use crate::render;


pub struct Vao {
    pub triangles_count: i32,
    
    id: u32,
    buffers: [(GLenum, u32); 3]
}

pub enum VaoBuffers {
    Vbo,
    Ebo,
    Instance,
}

impl Vao {
    pub fn new() -> Self { Self { id: 0, triangles_count: 0, buffers: [(0, 0); 3] } }
    
    pub fn gen_vao(&mut self) {
        let mut id: u32 = 0;
        unsafe {gl::GenVertexArrays(1, &mut id) }
        
        self.id = id;
    }
    
    pub fn bind(&self) {
        static mut CURRENT_BIND_VAO_ID: u32 = 0;
        
        unsafe {
            if CURRENT_BIND_VAO_ID == self.id { return }
            
            CURRENT_BIND_VAO_ID = self.id;
            gl::BindVertexArray(self.id);
        }
    }

    pub fn gen_buffer(&mut self, buffer_type: GLenum, buffer: VaoBuffers) {
        let mut buffer_id: u32 = 0;
        unsafe {gl::GenBuffers(1, &mut buffer_id) }

        self.buffers[buffer as usize] = (buffer_type, buffer_id);
    }

    pub fn bind_buffer(&self, buffer: VaoBuffers) {
        let (buffer_type, buffer_id) = self.buffers[buffer as usize];

        render::bind_buffer(buffer_type, buffer_id);
    }

    pub fn buffer_data_from_arr<T>(&mut self, vao_buffer: VaoBuffers, data: &[T], usage: GLenum) {
        use std::ffi::c_void;

        // if buffer is ebo then calculate triangles count
        if matches!(vao_buffer, VaoBuffers::Ebo) {
            assert_eq!(size_of::<T>(), size_of::<u32>());
            self.triangles_count = data.len() as i32;
        }

        unsafe {
            gl::BufferData(self.buffers[vao_buffer as usize].0, size_of_val(data) as isize, data.as_ptr() as *const c_void, usage)
        }
    }

    pub fn buffer_data(&mut self, vao_buffer: VaoBuffers, size: usize, data: Option<*const ()>, usage: GLenum) {
        use std::ffi::c_void;

        unsafe {
            let p: *const c_void = match data {
                Some(data) => data as *const c_void,
                _ => std::ptr::null(),
            };

            gl::BufferData(self.buffers[vao_buffer as usize].0, size as isize, p, usage)
        }
    }

    pub fn update_buffer<T: Default + Copy, const SIZE: usize>(&mut self, vao_buffer: VaoBuffers, arr: &resources::ArrayBuffer<T, SIZE>) {
        unsafe {
            let p = arr.as_ptr();
            let lb = arr.len_bytes();
            gl::NamedBufferSubData(self.buffers[vao_buffer as usize].1, 0, arr.len_bytes(), arr.as_ptr())
        }
    }

    pub fn attrib_info(&self, index: u32, size: i32, attrib_type: GLenum, stride: usize, offset: usize, instance: bool) {
        use std::ffi::c_void;

        unsafe {
            gl::VertexAttribPointer(index, size, attrib_type, gl::FALSE, stride as i32, offset as *const c_void);
            gl::EnableVertexAttribArray(index);

            if instance { gl::VertexAttribDivisor(index, 1) }
        }

    }
}