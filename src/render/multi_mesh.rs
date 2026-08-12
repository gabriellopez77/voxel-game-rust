use ash::vk;

use crate::{render::{core::{RawBuffer, VulkanApp, raw_buffer::{BufferFlags, BufferResizeMode}, vkutl}}, resources::{BufferArena, buffer_arena::RangeInfo}, utils::SafePtrMut};


#[derive(Clone, Copy)]
pub struct MultiMeshInfo {
    vertices_range: RangeInfo,
    indices_range: RangeInfo,

    index_count: u32,
}

impl MultiMeshInfo {
    pub fn new() -> Self {
        Self {
            vertices_range: RangeInfo::EMPTY,
            indices_range: RangeInfo::EMPTY,

            index_count: 0,
        }
    }
}

#[derive(Clone)]
struct ProfileInfo {
    indirect_data: Vec<vk::DrawIndexedIndirectCommand>,
    buffer: RawBuffer,
    draw_count: u32,
}

enum Profiles {
    Single(ProfileInfo),
    Multi(Vec<ProfileInfo>)
}

impl Profiles {
    fn get(&self, profile_idx: usize) -> &ProfileInfo {
        match self {
            Profiles::Single(profile) => profile,
            Profiles::Multi(profiles) => &profiles[profile_idx]
        }
    }

    fn get_mut(&mut self, profile_idx: usize) -> &mut ProfileInfo {
        match self {
            Profiles::Single(profile) => profile,
            Profiles::Multi(profiles) => &mut profiles[profile_idx]
        }
    }

}

pub struct MultiMesh {
    app: SafePtrMut<VulkanApp>,

    vertices_size: u32,

    vertices_arena: BufferArena,
    indices_arena: BufferArena,

    vertices_buffer: RawBuffer,
    indices_buffer: RawBuffer,

    profiles: Profiles,
} 

impl MultiMesh {
    pub fn new(app: SafePtrMut<VulkanApp>, vertices_size: usize) -> Self {
        Self {
            app,

            vertices_size: vertices_size as u32,

            vertices_arena: BufferArena::new(BufferArena::MB, 0),
            indices_arena: BufferArena::new(BufferArena::MB, 0),

            vertices_buffer: RawBuffer::new(),
            indices_buffer: RawBuffer::new(),

            profiles: Profiles::Single(ProfileInfo{ indirect_data: Vec::new(), buffer: RawBuffer::new(), draw_count: 0 }),
        }
    }

    pub fn get_buffers(&self, frame_index: usize, profile_idx: usize) -> [vk::Buffer; vkutl::MAX_BUFFERS_REQUIRED_TO_DRAW_COUNT] {
        return [
            self.vertices_buffer.get_buffer(frame_index),
            self.profiles.get(profile_idx).buffer.get_buffer(frame_index),
            self.indices_buffer.get_buffer(frame_index),
        ];
    }

    pub fn get_profile_draw_count(&self, profile_idx: usize) -> u32 { self.profiles.get(profile_idx).draw_count }

    pub fn start(&mut self, flags: BufferFlags) {
        self.vertices_buffer.create(&mut self.app,
            BufferArena::MB as usize,
            std::ptr::null(),
            vk::BufferUsageFlags::VERTEX_BUFFER,
            flags
        );

        self.indices_buffer.create(&mut self.app,
            BufferArena::MB as usize,
            std::ptr::null(),
            vk::BufferUsageFlags::INDEX_BUFFER,
            flags
        );

        self.profiles.get_mut(0).buffer.create(&mut self.app,
            BufferArena::MB as usize,
            std::ptr::null(),
            vk::BufferUsageFlags::INDIRECT_BUFFER,
            BufferFlags::RAM
        );
    }

    pub fn destroy(&mut self) {
        self.vertices_buffer.destroy(&mut self.app);
        self.indices_buffer.destroy(&mut self.app);

        match &mut self.profiles {
            Profiles::Single(profile) => {
                profile.buffer.destroy(&mut self.app);
            }
            Profiles::Multi(profiles) => {
                for pf in profiles {
                    pf.buffer.destroy(&mut self.app);
                }
            }
        }
    }

    pub fn create_profile(&mut self, flags: BufferFlags) -> usize {
        let mut buffer = RawBuffer::new();

        buffer.create(&mut self.app,
            BufferArena::KB as usize,
            std::ptr::null(),
            vk::BufferUsageFlags::INDIRECT_BUFFER,
            flags
        );

        match &mut self.profiles {
            Profiles::Single(profile) => {
                let profiles = vec![
                    profile.clone(),
                    ProfileInfo { indirect_data: Vec::new(), buffer, draw_count: 0 }
                ];

                self.profiles = Profiles::Multi(profiles);

                return 1;
            }
            Profiles::Multi(profiles) => {
                let idx = profiles.len();

                profiles.push(ProfileInfo { indirect_data: Vec::new(), buffer, draw_count: 0 });

                return idx;
            }
        }
    }

    pub fn add_mesh<T>(&mut self, vertices: &[T], indices: &[u32]) -> MultiMeshInfo {
        if vertices.is_empty() {
            return MultiMeshInfo::new();
        }

        let vertices_size = vertices.len() * size_of::<T>();
        let indices_size = indices.len() * size_of::<u32>();


        let vertices_range = match self.vertices_arena.find_range(vertices_size as u32) {
            Some(range) => range,
            None => {
                self.vertices_arena.grow(vertices_size as u32);
                self.vertices_buffer.resize(&mut self.app, self.vertices_arena.get_capacity() as usize, BufferResizeMode::Preserve);

                self.vertices_arena.find_range(vertices_size as u32).unwrap()
            }
        };
        let indices_range = match self.indices_arena.find_range(indices_size as u32) {
            Some(range) => range,
            None => {
                self.indices_arena.grow(indices_size as u32);
                self.indices_buffer.resize(&mut self.app, self.indices_arena.get_capacity() as usize, BufferResizeMode::Preserve);

                self.indices_arena.find_range(indices_size as u32).unwrap()
            }
        };


        self.vertices_buffer.update(&mut self.app,
            vertices_size,
            vertices_range.start as usize,
            vertices.as_ptr() as _,
        );

        self.indices_buffer.update(&mut self.app,
            indices_size,
            indices_range.start as usize,
            indices.as_ptr() as _,
        );

        MultiMeshInfo {
            vertices_range,
            indices_range,
            index_count: indices.len() as u32,
        }
    }

    pub fn remove_mesh(&mut self, mesh_info: &mut MultiMeshInfo) {
        self.vertices_arena.restore_range(&mut mesh_info.vertices_range);
        self.indices_arena.restore_range(&mut mesh_info.indices_range);

        mesh_info.index_count = 0;
    }

    pub fn record_mesh_info(&mut self, mesh_info: MultiMeshInfo, profile_idx: usize) {
        if mesh_info.index_count == 0 { return }

        let indirect_command = vk::DrawIndexedIndirectCommand {
           index_count: mesh_info.index_count,
           instance_count: 1,
           first_index: mesh_info.indices_range.start / 4,
           vertex_offset: (mesh_info.vertices_range.start / self.vertices_size) as i32,
           first_instance: 0,
        };

        self.profiles.get_mut(profile_idx).indirect_data.push(indirect_command);
    }

    pub fn update_profile(&mut self, profile_idx: usize) {
        let profile = &mut self.profiles.get_mut(profile_idx);

        if profile.indirect_data.is_empty() { return }

        profile.draw_count = profile.indirect_data.len() as u32;
        profile.buffer.update_and_resize(&mut self.app,
            profile.indirect_data.len() * size_of::<vk::DrawIndexedIndirectCommand>(),
            0,
            profile.indirect_data.as_ptr() as _,
            BufferResizeMode::Discard,
        );

        profile.indirect_data.clear();
    }
}
