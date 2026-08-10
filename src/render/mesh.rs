use std::array;
use ash::vk;
use crate::{render::core::raw_buffer::BufferResizeType, utils::SafePtrMut};

use super::core::{vkutl, VulkanApp, raw_buffer::BufferFlags, RawBuffer};


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BuffersTypes {
    Vertex,
    Instance,
    Index,
}

pub struct Mesh {
    app: SafePtrMut<VulkanApp>,
    
    raw_buffers: [RawBuffer; vkutl::MAX_VERTEX_BINDING_COUNT + 1],

    triangles_count: u32,
}

unsafe impl Send for Mesh {}

impl Mesh {
    pub fn new(app: SafePtrMut<VulkanApp>) -> Self {
        Self {
            app,
            
            raw_buffers: array::from_fn(|_| RawBuffer::new()),

            triangles_count: 0,
        }
    }

    pub fn destroy(&mut self) {
        for buffer in &mut self.raw_buffers {
            buffer.destroy(&mut self.app);
        }
    }

    pub fn get_triangles_count(&self) -> u32 { self.triangles_count }

    pub fn get_buffers(&self, frame_index: usize) -> [vk::Buffer; vkutl::MAX_VERTEX_BINDING_COUNT + 1] {
        let mut buffers = [vk::Buffer::null(); vkutl::MAX_VERTEX_BINDING_COUNT + 1];

        for i in 0..vkutl::MAX_VERTEX_BINDING_COUNT + 1 {
            buffers[i] = self.raw_buffers[i].get_buffer(frame_index);
        }

        return buffers;
    }

    pub fn set<T>(&mut self, vertices: &[T], indices: &[u32], flags: BufferFlags) {
        self.update_or_realloc(
            BuffersTypes::Index,
            indices.len() * size_of::<u32>(),
            indices.as_ptr() as _,
            flags,
            BufferResizeType::Discard
        );
        
        self.update_or_realloc(
            BuffersTypes::Vertex,
            vertices.len() * size_of::<T>(),
            vertices.as_ptr() as _,
            flags,
            BufferResizeType::Discard
        );
    }

    pub fn create_instance_buffer(&mut self, size: usize, data: Option<*const u8>, flags: BufferFlags) {
        self.create_buffer(BuffersTypes::Instance, size, data.unwrap_or(std::ptr::null()), flags);
    }

    pub fn create_instance_buffer_from_arr<T>(&mut self, arr: &[T], flags: BufferFlags) {
        self.create_instance_buffer(size_of::<T>() * arr.len(), Some(arr.as_ptr() as _), flags);
    }

    pub fn update_instance_buffer<T>(&mut self, arr: &[T], resize_type: BufferResizeType) {
        if arr.len() > 0 {
            self.update_and_resize_buffer(
                BuffersTypes::Instance,
                arr.len() * size_of::<T>(),
                arr.as_ptr() as _,
                resize_type
            );
        }
    }

    fn create_buffer(&mut self, buffer_type: BuffersTypes, size: usize, data: *const u8, flags: BufferFlags) {
        let mut usage = vk::BufferUsageFlags::VERTEX_BUFFER;

        if buffer_type == BuffersTypes::Index {
            usage = vk::BufferUsageFlags::INDEX_BUFFER;
            self.triangles_count = (size / 4) as u32;
        }

        self.raw_buffers[buffer_type as usize].create(&mut self.app, size, data, usage, flags);
    }

    fn update_or_realloc(&mut self,
        buffer_type: BuffersTypes,
        size: usize,
        data: *const u8,
        flags: BufferFlags,
        resize_type: BufferResizeType
    ) {
        let buffer = &mut self.raw_buffers[buffer_type as usize];
        let mut usage = vk::BufferUsageFlags::VERTEX_BUFFER;

        if buffer_type == BuffersTypes::Index {
            usage = vk::BufferUsageFlags::INDEX_BUFFER;
            self.triangles_count = (size / 4) as u32
        }
        
        // ONCE buffers can not be updated, then we destroy and create it again
        if size > buffer.size || buffer.flags.contains(BufferFlags::ONCE) {
            buffer.destroy(&mut self.app);

            buffer.create(&mut self.app, size, data, usage, flags);
        }
        else {
            self.update_and_resize_buffer(buffer_type, size, data, resize_type);
        }
    }

    fn update_and_resize_buffer(&mut self,
        buffer_type: BuffersTypes,
        size: usize,
        data: *const u8,
        resize_type: BufferResizeType
    ) {
        if buffer_type == BuffersTypes::Index {
            self.triangles_count = (size / 4) as u32
        }

        self.raw_buffers[buffer_type as usize].update_and_resize(&mut self.app, size, 0, data, resize_type);
    }
}
