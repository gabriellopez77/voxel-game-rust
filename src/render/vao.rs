use gl::types::GLenum;


pub struct Vao {
    id: u32,
    pub triangles_count: i32,
    buffers: [(GLenum, u32); 3]
}

pub enum VaoBuffers {
    Vbo,
    Ebo,
    //Instance,
}

impl Vao {
    pub fn new() -> Self {
        let mut id: u32 = 0;
        unsafe {gl::GenVertexArrays(1, &mut id) }

        return Vao { id, triangles_count: 0, buffers: [(0, 0); 3] };
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
        static mut CURRENT_BIND_BUFFER_ID: u32 = 0;
        static mut CURRENT_BIND_BUFFER_TYPE: GLenum = 0;

        let (buffer_type, buffer_id) = self.buffers[buffer as usize];

        // unsafe block is safe because this func can just be call in main thread
        unsafe {
            // if current buffer type and id is the same that we want bind, then return
            if buffer_type == CURRENT_BIND_BUFFER_TYPE || buffer_id == CURRENT_BIND_BUFFER_ID { return }

            CURRENT_BIND_BUFFER_TYPE = buffer_type;
            CURRENT_BIND_BUFFER_ID = buffer_id;
            
            gl::BindBuffer(buffer_type, buffer_id);
        }
    }

    pub fn buffer_data<T>(&mut self, vao_buffer: VaoBuffers, data: &[T], usage: GLenum) {
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

    pub fn attrib_pointer(&self, index: u32, size: i32, attrib_type: GLenum, stride: usize, offset: usize, instance: bool) {
        use std::ffi::c_void;

        unsafe {
            gl::VertexAttribPointer(index, size, attrib_type, gl::FALSE, stride as i32, offset as *const c_void);
            gl::EnableVertexAttribArray(index);

            if instance { gl::VertexAttribDivisor(index, 1) }
        }

    }
}