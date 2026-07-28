use ash::vk;

use super::core::{VulkanApp, raw_buffer::BufferFlags, RawBuffer};
use crate::utils::NullSafePtrMut;


pub struct Ubo<T: Copy + Clone + Default> {
    pub data: T,

    pub buffer: RawBuffer,

    app: NullSafePtrMut<VulkanApp>
}

impl<T: Copy + Clone + Default> Ubo<T> {
    pub fn new() -> Self {
        Self {
            data: T::default(),

            buffer: RawBuffer::new(),

            app: NullSafePtrMut::null(),
        }
    }

    pub fn create(&mut self, app: &mut VulkanApp, flags: BufferFlags) {
        self.buffer.create(app, size_of::<T>(), std::ptr::null(), vk::BufferUsageFlags::UNIFORM_BUFFER, flags);

        self.app = NullSafePtrMut::new(app);
    }

    pub fn destroy(&mut self, app: &mut VulkanApp) {
        self.buffer.destroy(app);
    }

    pub fn flush_all_data(&mut self) {
        let ptr: *const T = &self.data;
        self.buffer.update(&mut self.app, size_of::<T>(), 0, ptr as _);
    }
}
