use gl::types::GLenum;

use crate::render::render_utils;


#[derive(Copy, Clone)]
struct BufferInfo {
    id: u32,
    size: u32,
    binding_index: u32
}

pub struct Vao {
    pub triangles_count: i32,

    id: u32,
    binding_index: u32,
    binding_buffer: VaoBuffers,
    buffers: [BufferInfo; 3],
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
            buffers: [BufferInfo{ id: 0, size: 0, binding_index: 0 }; 3]
        }
    }

    pub fn is_generated(&self) -> bool { self.id != 0 }

    pub fn gen_vao(&mut self) -> &mut Vao {
        let mut id: u32 = 0;
        unsafe {gl::CreateVertexArrays(1, &mut id) }

        self.id = id;

        return self;
    }

    pub fn delete(&mut self) {
        unsafe {
            for buffer_info in &mut self.buffers {
                let id = buffer_info.id;

                if id != 0 {
                    gl::DeleteBuffers(1, &id);
                }

                buffer_info.id = 0;
            }

            gl::DeleteVertexArrays(1, &self.id);
            self.id = 0;
        }
    }

    pub fn bind(&self) {
        render_utils::bind_vao(self.id);
    }

    pub fn gen_buffer(&mut self, vao_buffer: VaoBuffers) -> &mut Vao {
        let mut buffer_id: u32 = 0;
        unsafe {gl::CreateBuffers(1, &mut buffer_id) }

        self.buffers[vao_buffer as usize] = BufferInfo {
            id: buffer_id,
            size: 0,
            binding_index: self.binding_index
        };

        if !matches!(vao_buffer, VaoBuffers::Ebo) {
            self.binding_index += 1
        }

        return self;
    }

    pub fn buffer_data_from_arr<T>(&mut self, vao_buffer: VaoBuffers, data: &[T], usage: GLenum) -> &Vao {
        return self.buffer_data(vao_buffer, size_of_val(data), Some(data.as_ptr() as *const std::ffi::c_void), usage);
    }

    pub fn buffer_data(&mut self, buffer: VaoBuffers, size: usize, data: Option<*const std::ffi::c_void>, usage: GLenum) -> &Vao {
        let buffer_info = &mut self.buffers[buffer as usize];

        let data_ptr = match data {
            Some(data) => data,
            _ => std::ptr::null(),
        };


        buffer_info.size = size as u32;

        unsafe {
            gl::NamedBufferData(buffer_info.id, size as isize, data_ptr, usage);

            // if buffer is ebo then calculate triangles count
            if matches!(buffer, VaoBuffers::Ebo) {
                self.triangles_count = size as i32 / 4;
                gl::VertexArrayElementBuffer(self.id, buffer_info.id);
            }
            else { self.binding_index = buffer_info.binding_index }
        }

        self.binding_buffer = buffer;

        return self;
    }

    pub fn update_buffer<T: Copy>(&mut self, buffer: VaoBuffers, arr: &Vec<T>) {
        let buffer_info = &self.buffers[buffer as usize];

        let size= (arr.len() * size_of::<T>()) as isize;

        if size > buffer_info.size as isize {
            panic!("data size out of buffer bounds! buffer size: {}, data size: {}", buffer_info.size, size)
        }

        unsafe {
            gl::NamedBufferSubData(
                buffer_info.id,
                0, size,
                arr.as_ptr() as *const std::ffi::c_void
            )
        }
    }

    pub fn smart_reallocate_buffer<T>(&mut self, buffer: VaoBuffers, data: &Vec<T>) {
        let buffer_info = &self.buffers[buffer as usize];
        let data_size = (data.len() * size_of::<T>()) as isize;
        let data_ptr = data.as_ptr() as *const std::ffi::c_void;

        // if buffer is ebo then calculate triangles count
        if matches!(buffer, VaoBuffers::Ebo) {
            self.triangles_count = data_size as i32 / 4;
        }

        unsafe {
            if data_size >= buffer_info.size as isize {
                gl::NamedBufferData(buffer_info.id, data_size, data_ptr, gl::STATIC_DRAW);
            }
            else {
                gl::NamedBufferSubData(buffer_info.id, 0, data_size, data_ptr);
            }
        }
    }

    pub fn attrib_info(&self, index: u32, size: i32, attrib_type: GLenum, offset: usize, instance: bool) -> &Vao {
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
            let buffer_id = self.buffers[self.binding_buffer as usize].id;

            gl::VertexArrayVertexBuffer(self.id, self.binding_index, buffer_id, 0, stride as i32);
        }
    }
}
