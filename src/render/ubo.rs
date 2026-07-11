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

pub struct Ubo {
    pub size: u64,

    pub buffer: RawBuffer,

    offsets: HashMap<&'static str, OffsetData>,
    last_field_size: u64,

    app: NullSafePtr<VulkanApp>
}

impl Ubo {
    pub fn new() -> Self {
        Self {
            size: 0,

            buffer: RawBuffer::new(),

            offsets: HashMap::new(),
            last_field_size: 0,

            app: NullSafePtr::null(),
        }
    }

    pub fn add<T>(&mut self, name: &'static str) {
        let size = size_of::<T>() as u64;

        // align size to opengl memory layout specification
        let alignment = match size {
            1..=4 => 4,
            5..=8 => 8,
            12..=16 => 16,
            64 => 16,
            _ => panic!("Ubo size not supported: {size}"),
        };

        let offset = Self::align_up(self.size, alignment);
        self.size = offset + size;
        self.last_field_size = size;

        // fits int, bool or float in last vec3's padding
        if self.last_field_size == 12 && size == 4 {
            self.offsets.insert(name, OffsetData { offset: self.size - 4, size });
        }
        else {
            self.offsets.insert(name, OffsetData { offset, size });
        }
    }

    pub fn create(&mut self, app: &mut VulkanApp, flags: BufferFlags) {
        debug_assert!(flags.contains(BufferFlags::DUPLICATE), "Ubo buffer need DUPLICATE");

        self.buffer.create(app, self.size, std::ptr::null(), vk::BufferUsageFlags::UNIFORM_BUFFER, flags);

        self.app = NullSafePtr::new(app);
    }

    pub fn destroy(&mut self, app: &mut VulkanApp) {
        self.buffer.destroy(app);
    }

    pub fn update<T>(&self, name: &'static str, data: *const T) {
        let offset_data = self.offsets[name];

        self.buffer.update(&self.app, offset_data.size, offset_data.offset as usize, data as _);
    }

    fn align_up(value: u64, alignment: u64) -> u64 {
        ((value + alignment - 1) / alignment) * alignment
    }
}
