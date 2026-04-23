use std::collections::HashMap;
use crate::math;
use crate::render;


#[derive(Clone, Copy)]
struct OffseData {
    offset: i32,
    size: i32,
}

pub struct Ubo  {
    offsets: HashMap<&'static str, OffseData>,
    current_offset: i32,
    id: u32,
}

impl Ubo {
    pub fn new() -> Self { Self { offsets: HashMap::new(), current_offset: 0, id: 0 } }

    pub fn add<T>(&mut self, name: &'static str) {
        let size = size_of::<T>() as i32;
        let offset = self.current_offset;

        // align size to opengl memory layout specification
        let alignment = match size {
            1..=4 => 4,
            5..=8 => 8,
            12..=16 => 16,
            64 => 16,
            _ => panic!("Ubo size not supported: {}", size),
        };

        self.current_offset += math::align_up(size, alignment);

        self.offsets.insert(name, OffseData { offset, size: size });
    }

    pub fn create(&mut self, index: u32) {
        unsafe {
            gl::CreateBuffers(1, &mut self.id);
            
            gl::NamedBufferData(self.id, self.current_offset as isize, std::ptr::null(), gl::DYNAMIC_DRAW);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, index, self.id);
        }
    }

    pub fn update<T>(&self, name: &'static str, data: *const T) {
        let offset_data = self.offsets[name];
        let len = offset_data.offset + offset_data.size;

        // check out of bounds
        if len > self.current_offset {
            panic!("Ubo data out of bounds! offset: {}, size: {}, len: {}, ubo size: {}",
                   offset_data.offset, offset_data.size, len, self.current_offset);
        }

        unsafe {
            gl::NamedBufferSubData(self.id, offset_data.offset as isize, offset_data.size as isize,
                                   data as *const std::ffi::c_void);
        }
    }
}