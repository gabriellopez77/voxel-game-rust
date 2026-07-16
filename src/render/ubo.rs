use std::collections::HashMap;

use ash::vk;
use crate::utils::NullSafePtr;

use super::raw_buffer::BufferFlags;
use super::{vulkan_app::VulkanApp, RawBuffer};


#[derive(Clone, Copy)]
struct OffsetData {
    offset: u64,
    size: u64,
}

pub struct Ubo<T: Copy + Clone + Default> {
    pub data: T,

    pub buffer: RawBuffer,

    offsets: HashMap<&'static str, OffsetData>,
    last_field_size: u64,

    app: NullSafePtr<VulkanApp>
}

impl<T: Copy + Clone + Default> Ubo<T> {
    pub fn new() -> Self {
        Self {
            data: T::default(),

            buffer: RawBuffer::new(),

            offsets: HashMap::new(),
            last_field_size: 0,

            app: NullSafePtr::null(),
        }
    }

    pub fn create(&mut self, app: &mut VulkanApp, flags: BufferFlags) {
        debug_assert!(flags.contains(BufferFlags::DUPLICATE), "Ubo buffer need DUPLICATE");

        self.buffer.create(app, size_of::<T>() as u64, std::ptr::null(), vk::BufferUsageFlags::UNIFORM_BUFFER, flags);

        self.app = NullSafePtr::new(app);
    }

    pub fn destroy(&mut self, app: &mut VulkanApp) {
        self.buffer.destroy(app);
    }

    pub fn flush_all_data(&mut self) {
        let ptr: *const T = &self.data;
        self.buffer.update(&self.app, size_of::<T>() as u64, 0, ptr as _);
    }

    fn align_up(value: u64, alignment: u64) -> u64 {
        ((value + alignment - 1) / alignment) * alignment
    }
}
