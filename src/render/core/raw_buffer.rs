use std::ops::BitOr;
use ash::vk;
use crate::resources::buffer_arena::RangeInfo;

use super::{vkutl, VulkanApp};


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BufferResizeMode {
    Discard,
    Preserve,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BufferFlags(pub u32);

impl BufferFlags {
    pub const EMPTY: Self = Self(0b0000_0000);

    /// create buffer in vram (DEVICE_LOCAL)
    pub const VRAM: Self = Self(0b0000_0001);

    /// create buffer in ram (HOST_VISIBLE)
    pub const RAM: Self = Self(0b0000_0010);

    /// buffer is updated once time and is not duplicated because his data not change after be created.
    /// it is only compatible with VRAM flag
    pub const ONCE: Self = Self(0b0000_0100);

    /// the buffer is not update every frame
    pub const RARE_UPDATE: Self = Self(0b0000_1000);

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

#[derive(Clone)]
pub struct RawBuffer {
    pub usage: vk::BufferUsageFlags,
    pub flags: BufferFlags,
    pub size: usize,

    // Vram: used to copy buffer to vram
    // Ram: not used
    staging_buffer_ranges: [RangeInfo; vkutl::FRAMES_COUNT],

    // VRAM: not used
    buffers: [vk::Buffer; vkutl::FRAMES_COUNT],
    allocations: [vk_mem::Allocation; vkutl::FRAMES_COUNT],
    mapped_memory: [*mut u8; vkutl::FRAMES_COUNT],
}

impl RawBuffer {
    pub fn new() -> Self {
        Self {
            usage: vk::BufferUsageFlags::empty(),
            flags: BufferFlags::EMPTY,
            size: 0,

            staging_buffer_ranges: [RangeInfo::EMPTY; vkutl::FRAMES_COUNT],

            buffers: [vk::Buffer::null(); vkutl::FRAMES_COUNT],
            allocations: [vkutl::null_allocation(); vkutl::FRAMES_COUNT],

            mapped_memory: [std::ptr::null_mut(); vkutl::FRAMES_COUNT]
        }
    }

    pub fn get_buffer(&self, frame_index: usize) -> vk::Buffer {
        if self.flags.contains(BufferFlags::ONCE) {
            return self.buffers[0];
        }

        // SAFETY: frame_index is always 0..vkutl::FRAMES_COUNT
        return unsafe { *self.buffers.get_unchecked(frame_index) };
    }

    pub fn get_mapped_memory(&self, frame_index: usize) -> *mut u8 {
        assert!(!self.flags.contains(BufferFlags::ONCE), "ONCE buffer have not mapped memory");

        // SAFETY: frame_index is always 0..vkutl::FRAMES_COUNT
        return unsafe { *self.mapped_memory.get_unchecked(frame_index) };
    }

    pub fn get_all_buffers(&self) -> [vk::Buffer; vkutl::FRAMES_COUNT] { self.buffers }
    pub fn get_all_mapped_memory(&self) -> [*mut u8; vkutl::FRAMES_COUNT] { self.mapped_memory }

    pub fn create(&mut self, app: &mut VulkanApp, size: usize, data: *const u8, usage: vk::BufferUsageFlags, flags: BufferFlags) {
        debug_assert!(flags.contains(BufferFlags::VRAM) || flags.contains(BufferFlags::RAM), "Need RAM or VRAM flag");
        debug_assert!(!(flags.contains(BufferFlags::VRAM) && flags.contains(BufferFlags::RAM)), "Can not have VRAM and RAM flags");
        debug_assert!(!(flags.contains(BufferFlags::RAM) && flags.contains(BufferFlags::ONCE)), "Ram buffer can not have ONCE flag");
        debug_assert!(!(flags.contains(BufferFlags::ONCE) && flags.contains(BufferFlags::RARE_UPDATE)), "ONCE buffer can not have RARE_UPDATE flag");
        debug_assert!(!(flags.contains(BufferFlags::ONCE) && data.is_null()), "data cannot be null if buffer is ONCE");
        debug_assert!(size > 0, "Invalid buffer size!");

        self.size = size;
        self.flags = flags;
        self.usage = usage;
        
        if flags.contains(BufferFlags::VRAM) {
            let mut allocation_info = vk_mem::AllocationCreateInfo::default();
            allocation_info.usage = vk_mem::MemoryUsage::Auto;
            allocation_info.preferred_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;

            // create buffer in vram
            for i in 0..vkutl::FRAMES_COUNT {
                // create staging buffer
                if !flags.contains(BufferFlags::RARE_UPDATE) {
                    self.staging_buffer_ranges[i] = app.alloc_staging_buffer_range(size);
                }

                (self.buffers[i], self.allocations[i]) = vkutl::create_buffer(app,
                    size,
                    vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC | usage,
                    &allocation_info, !flags.contains(BufferFlags::ONCE)
                );


                // if not DUPLICATE then we use only first buffer
                if flags.contains(BufferFlags::ONCE) { break }
            }
        }
        else {
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
                if flags.contains(BufferFlags::ONCE) {
                    let release_barrier = vk::BufferMemoryBarrier::default()
                        .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                        .src_queue_family_index(app.transfer_queue_index)
                        .dst_queue_family_index(app.graphics_queue_index)
                        .buffer(self.buffers[0])
                        .size(vk::WHOLE_SIZE);


                    unsafe {
                        app.ash_device.cmd_pipeline_barrier(app.get_transfer_cmd(),
                            vk::PipelineStageFlags::TRANSFER,
                            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                            vk::DependencyFlags::empty(),
                            &[],
                            &[release_barrier],
                            &[]
                        );
                    }

                    app.add_buffer_transfer_ownership(self.buffers[0]);

                    break;
                }
            }
        }


        // if data is not null then copy to buffer
        if !data.is_null() {
            for i in 0..vkutl::FRAMES_COUNT {
                self.update_with_index(app, i, size, 0, data);

                // if not DUPLICATE then we use only first buffer
                if flags.contains(BufferFlags::ONCE) || flags.contains(BufferFlags::RARE_UPDATE) { break }
            }
        }

        // if ONCE we do not update the buffer again, then, dealloc staging buffer range
        if self.flags.contains(BufferFlags::ONCE) {
            app.dealloc_staging_buffer_range(&mut self.staging_buffer_ranges[0]);
        }
    }

    pub fn destroy(&mut self, app: &mut VulkanApp) {
        // buffer has not be created or has be destroyed
        if self.size == 0 { return }

        // unmap and dealloc staging buffer range
        if self.flags.contains(BufferFlags::VRAM) && !self.flags.contains(BufferFlags::ONCE) && !self.flags.contains(BufferFlags::RARE_UPDATE) {
            for i in 0..vkutl::FRAMES_COUNT {
                app.dealloc_staging_buffer_range(&mut self.staging_buffer_ranges[i]);
            }
        }

        // destroy buffers
        app.destroy_buffer(&mut self.buffers, &mut self.allocations, &mut self.mapped_memory);

        self.size = 0;
        self.flags = BufferFlags::EMPTY;
    }

    pub fn update(&mut self, app: &mut VulkanApp, size: usize, offset: usize, data: *const u8) {
        assert!(self.size >= size, "Invalid data size");
        debug_assert!(!data.is_null(), "Data is null!");
        debug_assert!(!self.flags.contains(BufferFlags::ONCE), "Buffers that constains ONCE flag cant be updated!");
        debug_assert!(size > 0, "Is not possible update zero bytes");

        self.update_with_index(app, app.frame_index, size, offset, data);
    }

    pub fn update_and_resize(&mut self,
        app: &mut VulkanApp,
        size: usize,
        offset: usize,
        data: *const u8,
        resize_mode: BufferResizeMode
    ) {
        debug_assert!(!data.is_null(), "Data is null!");
        debug_assert!(!self.flags.contains(BufferFlags::ONCE), "Buffers that constains ONCE flag cant be updated!");
        debug_assert!(size > 0, "Is not possible update zero bytes");

        if size > self.size {
            let mut new_size = self.size * 2;

            if new_size - self.size < size {
                new_size += size;
            }

            self.resize(app, new_size, resize_mode);
        }

        self.update(app, size, offset, data);
    }

    pub fn resize(&mut self, app: &mut VulkanApp, new_size: usize, resize_mode: BufferResizeMode) {
        assert!(new_size > self.size, "invalid new size");
        
        println!("Buffer Resized! {} -> {}", self.size, new_size);
        
        if resize_mode == BufferResizeMode::Preserve {
            app.resize_buffer_preserve_content(self, new_size);
        }
        else {
            let usage = self.usage;
            let flags = self.flags;
            
            self.destroy(app);
            self.create(app, new_size, std::ptr::null(), usage, flags);
        }
    }

    fn update_with_index(&mut self, app: &mut VulkanApp, index: usize, size: usize, offset: usize, data: *const u8) {
        if self.flags.contains(BufferFlags::RARE_UPDATE) {
            app.update_buffer(self, data, offset, size);
            return;
        }

        if self.flags.contains(BufferFlags::VRAM) {
            app.copy_data_to_staging_buffer(self.staging_buffer_ranges[index], offset, size, data);
            app.copy_stagin_buffer_async(self.staging_buffer_ranges[index], size, self.buffers[index], offset, 0);
        }
        else {
            unsafe {
                std::ptr::copy_nonoverlapping(data, self.mapped_memory[index].byte_add(offset), size);
            }

            // update gpu memory cache
            app.vma_allocator.flush_allocation(&self.allocations[index], offset as u64, size as u64)
                .expect("Failed to flush memory ranges!");
        }
    }
}
