use std::ops::BitOr;
use ash::vk;
use super::{vkutl, VulkanApp};


#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BufferFlags(u32);

impl BufferFlags {
    pub const EMPTY: Self = Self(0b0000_0000);

    /// create buffer in vram (DEVICE_LOCAL)
    pub const VRAM: Self = Self(0b0000_0001);

    /// create buffer in ram (HOST_VISIBLE)
    pub const RAM: Self = Self(0b0000_0010);

    /// buffer is updated once time. it is only compatible with VRAM flag and it is not duplicated because his data not change after be created
    pub const ONCE: Self = Self(0b0000_0100);

    pub fn contains(self, flag: Self) -> bool {
        self.0 & flag.0 == flag.0
    }
}

impl BitOr for BufferFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}


pub struct RawBuffer {
    pub flags: BufferFlags,
    pub size: u64,

    // Vram: used to copy buffer to vram
    // Ram: not used
    staging_buffer: [vk::Buffer; vkutl::FRAMES_COUNT],
    staging_allocation: [vk_mem::Allocation; vkutl::FRAMES_COUNT],

    buffers: [vk::Buffer; vkutl::FRAMES_COUNT],
    allocations: [vk_mem::Allocation; vkutl::FRAMES_COUNT],

    // Vram: staging buffer
    // Ram: buffer
    mapped_memory: [*mut u8; vkutl::FRAMES_COUNT],
}

impl RawBuffer {
    pub fn new() -> Self {
        Self {
            flags: BufferFlags::EMPTY,
            size: 0,

            staging_buffer: [vk::Buffer::null(); vkutl::FRAMES_COUNT],
            staging_allocation: unsafe { std::mem::zeroed() },

            buffers: [vk::Buffer::null(); vkutl::FRAMES_COUNT],
            allocations: unsafe { std::mem::zeroed() },

            mapped_memory: [std::ptr::null_mut(); vkutl::FRAMES_COUNT]
        }
    }

    pub fn get_buffer(&self, frame_index: usize) -> vk::Buffer {
        if self.flags.contains(BufferFlags::ONCE) {
            return self.buffers[0];
        }

        return self.buffers[frame_index];
    }

    pub fn get_all_buffers(&self) -> [vk::Buffer; vkutl::FRAMES_COUNT] {
        self.buffers
    }

    pub fn get_all_mapped_memory(&self) -> [*mut u8; vkutl::FRAMES_COUNT] {
        self.mapped_memory
    }

    pub fn create(&mut self, app: &mut VulkanApp, size: u64, data: *const u8, usage: vk::BufferUsageFlags, flags: BufferFlags) {
        debug_assert!(flags.contains(BufferFlags::VRAM) || flags.contains(BufferFlags::RAM), "Need RAM or VRAM flag");
        debug_assert!(!(flags.contains(BufferFlags::VRAM) && flags.contains(BufferFlags::RAM)), "Can not have VRAM and RAM flags");
        debug_assert!(!(flags.contains(BufferFlags::RAM) && flags.contains(BufferFlags::ONCE)), "Ram buffer can not have ONCE flag");
        debug_assert_ne!(size, 0, "Invalid buffer size!");

        self.size = size;
        self.flags = flags;

        if flags.contains(BufferFlags::VRAM) {
            let mut staging_allocation_info = vk_mem::AllocationCreateInfo::default();
            staging_allocation_info.usage = vk_mem::MemoryUsage::Auto;
            staging_allocation_info.preferred_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;
            staging_allocation_info.flags = vk_mem::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE;

            let mut allocation_info = vk_mem::AllocationCreateInfo::default();
            allocation_info.usage = vk_mem::MemoryUsage::Auto;
            allocation_info.preferred_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;

            // create buffer in vram
            for i in 0..vkutl::FRAMES_COUNT {
                // create staging buffer
                (self.staging_buffer[i], self.staging_allocation[i]) = vkutl::create_buffer(
                    app, size,
                    vk::BufferUsageFlags::TRANSFER_SRC,
                    &staging_allocation_info, false
                );

                (self.buffers[i], self.allocations[i]) = vkutl::create_buffer(
                    app, size,
                    vk::BufferUsageFlags::TRANSFER_DST | usage,
                    &allocation_info, !flags.contains(BufferFlags::ONCE)
                );

                // map staging buffer
                self.mapped_memory[i] = unsafe {
                    app.vma_allocator.map_memory(&mut self.staging_allocation[i]).expect("Failed to map memory!")
                };

                // if not DUPLICATE then we use only first buffer
                if flags.contains(BufferFlags::ONCE) { break }
            }
        }
        else {
            assert!(!flags.contains(BufferFlags::ONCE), "RAM buffer can not be ONCE");

            let mut allocation_info = vk_mem::AllocationCreateInfo::default();
            allocation_info.usage = vk_mem::MemoryUsage::Auto;
            allocation_info.preferred_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;
            allocation_info.flags = vk_mem::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE;

            // create and map memory
            for i in 0..vkutl::FRAMES_COUNT {
                (self.buffers[i], self.allocations[i]) = vkutl::create_buffer(app, size, usage, &allocation_info, false);

                self.mapped_memory[i] = unsafe {
                    app.vma_allocator.map_memory(&mut self.allocations[i]).expect("Failed to map memory!")
                };

                // if not DUPLICATE then we use only first buffer
                if flags.contains(BufferFlags::ONCE) { break }
            }
        }


        // if data is not null then copy to buffer
        if !data.is_null() {
            for i in 0..vkutl::FRAMES_COUNT {
                self.update_with_index(app, i, size, 0, data);

                // if not DUPLICATE then we use only first buffer
                if flags.contains(BufferFlags::ONCE) { break }
            }
        }

        // if ONCE then we do not update the buffer again, in other words, we can destroy staging buffer
        if self.flags.contains(BufferFlags::ONCE) {
            app.destroy_buffer(&mut self.staging_buffer[0], &mut self.staging_allocation[0], &mut self.mapped_memory[0]);
        }
    }

    pub fn update(&self, app: &VulkanApp, size: u64, offset: usize, data: *const u8) {
        assert!(self.size >= size, "Invalid data size");
        debug_assert!(!data.is_null(), "Data is null!");
        debug_assert!(!self.flags.contains(BufferFlags::ONCE), "Buffers that constains ONCE flag cant be updated!");

        // is not possible update zero bytes
        if size == 0 { return }

        self.update_with_index(app, app.frame_index, size, offset,  data);
    }

    fn update_with_index(&self, app: &VulkanApp, index: usize, size: u64, offset: usize, data: *const u8) {
        unsafe {
            if self.flags.contains(BufferFlags::VRAM) {
                // copy data to mapped staging buffer
                std::ptr::copy_nonoverlapping(data, self.mapped_memory[index].byte_add(offset), size as usize);

                // update gpu memory cache
                app.vma_allocator.flush_allocation(&self.staging_allocation[index], offset as u64, size).expect("Failed to flush memory ranges!");

                vkutl::copy_buffer_async(app, self.staging_buffer[index], self.buffers[index], size, offset as u64, offset as u64);
            }
            else {
                // copy data to mapped memory
                std::ptr::copy_nonoverlapping(data, self.mapped_memory[index].byte_add(offset), size as usize);

                // update gpu memory cache
                app.vma_allocator.flush_allocation(&self.allocations[index], offset as u64, size).expect("Failed to flush memory ranges!");
            }
        }
    }

    pub fn destroy(&mut self, app: &mut VulkanApp) {
        // buffer is not create or has destroyed
        if self.size == 0 { return }

        // destroy buffers
        for i in 0..vkutl::FRAMES_COUNT {
            // unmap and destroy staging buffer
            if self.flags.contains(BufferFlags::VRAM) && !self.flags.contains(BufferFlags::ONCE) {
                app.destroy_buffer(&mut self.staging_buffer[i], &mut self.staging_allocation[i], &mut self.mapped_memory[i]);
            }

            app.destroy_buffer(&mut self.buffers[i], &mut self.allocations[i], &mut self.mapped_memory[i]);

            // if not DUPLICATE then we use only first buffer
            if self.flags.contains(BufferFlags::ONCE) { break }
        }

        self.size = 0;
        self.flags = BufferFlags::EMPTY;
    }
}
