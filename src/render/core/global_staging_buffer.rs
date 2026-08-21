use ash::vk;
use super::{VulkanApp, vkutl};
use crate::resources::{BufferArena, buffer_arena::RangeInfo};


pub struct GlobalStagingBuffer {
    capacity: u32,

    arena: BufferArena,

    pub buffer: vk::Buffer,
    pub allocation: vk_mem::Allocation,
    pub mapped_memory: *mut u8,
}

impl GlobalStagingBuffer {
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            arena: BufferArena::new(capacity, BufferArena::KB / 2),

            buffer: vk::Buffer::null(),
            allocation: vkutl::null_allocation(),
            mapped_memory: std::ptr::null_mut(),
        }
    }

    pub fn get_used_mb(&self) -> f32 {
        self.arena.get_used_mb()
    }

    pub fn get_capacity_mb(&self) -> f32 {
        self.capacity as f32 / BufferArena::MB as f32
    }

    pub fn start(&mut self, app: &VulkanApp) {
        (self.buffer, self.allocation, self.mapped_memory) = Self::create_buffer(app, self.capacity as usize);
    }

    pub fn destroy(&mut self, app: &mut VulkanApp) {
        app.destroy_buffer(
            &mut [self.buffer, vk::Buffer::null()],
            &mut [self.allocation, vkutl::null_allocation()],
            &mut [self.mapped_memory, std::ptr::null_mut()]
        );
    }

    pub fn copy_data(&mut self, vma: &vk_mem::Allocator, range: RangeInfo, offset: u32, size: u32, data: *const u8) {
        debug_assert!(range.start + size <= self.capacity, "Invalid size!");

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.byte_add(offset as usize),
                self.mapped_memory.byte_add(range.start as usize),
                size as usize,
            );
        }

        vma.flush_allocation(&self.allocation, range.start as u64, size as u64).unwrap();
    }

    pub fn copy_self(&mut self, vma: &vk_mem::Allocator, src_offset: u32, dst_offset: u32, size: u32) {
        debug_assert!(src_offset + size <= self.capacity, "Invalid src_offset!");
        debug_assert!(dst_offset + size <= self.capacity, "invalid dst_offset!");

        unsafe {
            std::ptr::copy_nonoverlapping(
                self.mapped_memory.byte_add(src_offset as usize),
                self.mapped_memory.byte_add(dst_offset as usize),
                size as usize
            );
        }

        vma.flush_allocation(&self.allocation, dst_offset as u64, size as u64).unwrap();
    }

    pub fn copy_to_memory(&self, src_offset: u32, dst_offset: u32, size: u32, dst_mapped_memory: *mut u8) {
        debug_assert!(src_offset + size <= self.capacity, "Invalid src_offset!");

        unsafe {
            std::ptr::copy_nonoverlapping(
                self.mapped_memory.byte_add(src_offset as usize),
                dst_mapped_memory.byte_add(dst_offset as usize),
                size as usize
            );
        }
    }

    pub fn copy_to_buffer_async(&self,
        app: &VulkanApp,
        dst_buffer: vk::Buffer,
        size: u32,
        src_offset: u32,
        dst_offset: u32,
    ) {
        debug_assert!(src_offset + size <= self.capacity, "Invalid src_offset!");

        vkutl::copy_buffer_async(app,
            self.buffer,
            dst_buffer,
            size as usize,
            src_offset as usize,
            dst_offset as usize
        );
    }

    pub fn allocate(&mut self, app: &mut VulkanApp, size: u32) -> RangeInfo {
        let range = self.arena.find_range(size);

        if let Some(range) = range {
            return range;
        }

        let old_capacity = self.capacity;

        self.arena.grow(size);
        self.capacity = self.arena.get_capacity();

        let (new_buffer, new_allocation, new_mapped_memory) = Self::create_buffer(app, self.capacity as usize);

        self.copy_to_memory(0, 0, old_capacity, new_mapped_memory);
        self.destroy(app);

        self.buffer = new_buffer;
        self.allocation = new_allocation;
        self.mapped_memory = new_mapped_memory;

        // the arena grew, then is guaranteed that contains capacity for the range
        return self.arena.find_range(size).unwrap();
    }

    pub fn deallocate(&mut self, range: &mut RangeInfo) {
        self.arena.restore_range(range);
    }

    fn create_buffer(app: &VulkanApp, size: usize) -> (vk::Buffer, vk_mem::Allocation, *mut u8) {
        let mut alloc_info = vk_mem::AllocationCreateInfo::default();
        alloc_info.usage = vk_mem::MemoryUsage::Auto;
        alloc_info.preferred_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;
        alloc_info.flags = vk_mem::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE;

        let (buffer, mut allocation) = vkutl::create_buffer(app,
            size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            &alloc_info, false
        );

        let mapped_memory = unsafe { app.vma_allocator.map_memory(&mut allocation).unwrap() };

        return (buffer, allocation, mapped_memory);
    }
}
