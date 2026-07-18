use std::array;
use ash::vk;
use super::raw_buffer::BufferFlags;
use super::vulkan_app::VulkanApp;
use super::{vkutl, RawBuffer};


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BuffersTypes {
    Vertex,
    Instance,
    Index,
}

pub struct VerticesAttributes {
    raw_buffers: [RawBuffer; vkutl::MAX_VERTEX_BINDING_COUNT],
    index_buffer_info: RawBuffer,

    pub triangles_count: u32,
}

impl VerticesAttributes {
    pub fn new() -> Self {
        Self {
            raw_buffers: array::from_fn(|_| RawBuffer::new()),
            index_buffer_info: RawBuffer::new(),

            triangles_count: 0,
        }
    }

    pub fn destroy(&mut self, app: &mut VulkanApp) {
        for buffer in &mut self.raw_buffers {
            buffer.destroy(app);
        }

        self.index_buffer_info.destroy(app)
    }

    pub fn get_raw_buffer(&self, buffer_type: BuffersTypes) -> &RawBuffer {
        if buffer_type == BuffersTypes::Index {
            return &self.index_buffer_info;
        }

        &self.raw_buffers[buffer_type as usize]
    }

    pub fn create_buffer_from_arr<T>(&mut self, app: &mut VulkanApp, buffer_type: BuffersTypes, arr: &[T],
                                            flags: BufferFlags) -> &mut Self {
        self.create_buffer(app, buffer_type, arr.len() * size_of::<T>(), arr.as_ptr() as _, flags)
    }

    pub fn create_buffer(&mut self, app: &mut VulkanApp, buffer_type: BuffersTypes, size: usize, data: *const u8,
                                flags: BufferFlags) -> &mut Self {
        if buffer_type == BuffersTypes::Index {
            self.index_buffer_info.create(app, size as u64, data as _, vk::BufferUsageFlags::INDEX_BUFFER, flags);

            self.triangles_count = (size / 4) as u32;
        }
        else {
            self.raw_buffers[buffer_type as usize].create(app, size as u64, data, vk::BufferUsageFlags::VERTEX_BUFFER, flags);
        }

        self
    }

    pub fn bind(&self, app: &VulkanApp) {
        let command_buffer = app.get_current_command_buffer();

        let mut buffers = [vk::Buffer::null(); vkutl::MAX_VERTEX_BINDING_COUNT];
        const OFFSETS: [u64; vkutl::MAX_VERTEX_BINDING_COUNT] = [0; vkutl::MAX_VERTEX_BINDING_COUNT];

        let mut buffers_count = 0;


        for i in 0..vkutl::MAX_VERTEX_BINDING_COUNT {
            // if size == 0 then it is not used
            if self.raw_buffers[i].size == 0 { continue }

            buffers[buffers_count] = self.raw_buffers[i].get_buffer(app.frame_index);

            buffers_count += 1;
        }

        // bind all buffers with one bind function
        if buffers_count != 0 {
            unsafe {
                app.ash_device.cmd_bind_vertex_buffers(command_buffer, 0, &buffers[0..buffers_count], &OFFSETS[0..buffers_count]);
            }
        }

        // if size == 0 then we do not use index buffer
        if self.index_buffer_info.size != 0 {
            unsafe {
                app.ash_device.cmd_bind_index_buffer(
                    command_buffer,
                    self.index_buffer_info.get_buffer(app.frame_index),
                    0,
                    vk::IndexType::UINT32
                );
            };
        }
    }

    pub fn update_or_realloc<T>(&mut self, app: &mut VulkanApp, buffer_type: BuffersTypes, arr: &[T], flags: BufferFlags) {
        let buffer: &mut RawBuffer;
        let mut usage = vk::BufferUsageFlags::VERTEX_BUFFER;
        let data_size = (arr.len() * size_of::<T>()) as u64;

        if buffer_type == BuffersTypes::Index {
            buffer = &mut self.index_buffer_info;
            usage = vk::BufferUsageFlags::INDEX_BUFFER;
            self.triangles_count = (data_size / 4) as u32
        }
        else {
            buffer = &mut self.raw_buffers[buffer_type as usize];
        }

        // ONCE buffers can not be updated, then we destroy and create it again
        if buffer.flags.contains(BufferFlags::ONCE) {
            buffer.destroy(app);
            buffer.create(app, data_size, arr.as_ptr() as _, usage, flags);
            return;
        }

        // create a new buffer
        if data_size > buffer.size {
            buffer.destroy(app);
            buffer.create(app, data_size, arr.as_ptr() as _, usage, flags);
        }
        else {
            self.update_buffer(app, buffer_type, arr);
        }
    }

    pub fn update_buffer<T>(&mut self, app: &VulkanApp, buffer_type: BuffersTypes, arr: &[T]) {
        let data_size = arr.len() * size_of::<T>();

        if buffer_type == BuffersTypes::Index {
            self.index_buffer_info.update(app, data_size as u64, 0, arr.as_ptr() as _);

            self.triangles_count = (data_size / 4) as u32
        }
        else {
            self.raw_buffers[buffer_type as usize].update(app, data_size as u64, 0, arr.as_ptr() as _);
        }
    }

    pub fn update_buffer2<T>(&mut self, app: &mut VulkanApp, buffer_type: BuffersTypes, arr: &[T]) {
        let data_size = arr.len() * size_of::<T>();

        if buffer_type == BuffersTypes::Index {
            self.index_buffer_info.update(app, data_size as u64, 0, arr.as_ptr() as _);

            self.triangles_count = (data_size / 4) as u32
        }
        else {
            self.raw_buffers[buffer_type as usize].update(app, data_size as u64, 0, arr.as_ptr() as _);
            app.update_buffer(&self.raw_buffers[buffer_type as usize], arr.as_ptr() as _, data_size, 0);
        }

    }
}
