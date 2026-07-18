use std::{cell::RefCell, rc::Rc};

use crate::{render::{DrawInfo, GraphicsPipeline, VerticesAttributes, VulkanApp, raw_buffer::BufferFlags, vertices_attributes::BuffersTypes, vkutl}, utils::MutSafePtr};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MaterialType {
    ChunksOpaque,
    ChunksAlpha,
    Opaque,
    Alpha,
    Ui,
    Sky,
    Particle,
}

pub struct Material {
    app: MutSafePtr<VulkanApp>,

    pipeline: Rc<RefCell<GraphicsPipeline>>,
    vao: VerticesAttributes,

    material_type: MaterialType,

    push_data: [u8; vkutl::MAX_PUSH_CONSTANT_SIZE],
    push_range: u8,
}

unsafe impl Send for Material {}

impl Material {
    pub fn new(app: &mut VulkanApp, pipeline: Rc<RefCell<GraphicsPipeline>>, material_type: MaterialType) -> Self {
        Self {
            app: MutSafePtr::new(app),

            pipeline: pipeline,
            vao: VerticesAttributes::new(),

            material_type: material_type,

            push_data: [0; vkutl::MAX_PUSH_CONSTANT_SIZE],
            push_range: 0,
        }
    }

    pub fn destroy(&mut self) {
        self.vao.destroy(&mut self.app);
    }

    pub fn get_triangles_count(&self) -> u32 { self.vao.triangles_count }
    pub fn get_push_constant_info(&self) -> (u8, &[u8; vkutl::MAX_PUSH_CONSTANT_SIZE]) { (self.push_range, &self.push_data) }
    pub fn get_type(&self) -> MaterialType { self.material_type }

    pub fn set_mesh<T2>(&mut self, vertices: &[T2], indices: &[u32], flags: BufferFlags) {
        self.vao.update_or_realloc(&mut self.app, BuffersTypes::Index, &indices, flags);
        self.vao.update_or_realloc(&mut self.app, BuffersTypes::Vertex, &vertices, flags);
    }

    pub fn create_instance_buffer_from_arr<T2>(&mut self, arr: &[T2], flags: BufferFlags) {
        self.create_instance_buffer(size_of::<T2>() * arr.len(), Some(arr.as_ptr() as _), flags);
    }

    pub fn create_instance_buffer(&mut self, size: usize, data: Option<*const u8>, flags: BufferFlags) {
        self.vao.create_buffer(&mut self.app, BuffersTypes::Instance, size, data.unwrap_or(std::ptr::null()), flags);
    }

    pub fn update_instance_data<T2>(&mut self, arr: &[T2]) {
        self.vao.update_buffer(&self.app, BuffersTypes::Instance, arr);
    }

    pub fn update_instance_data2<T2>(&mut self, arr: &[T2]) {
        self.vao.update_buffer2(&mut self.app, BuffersTypes::Instance, arr);
    }

    pub fn update_push_constant<T>(&mut self, offset: usize, data: *const T) {
        let size = size_of::<T>();

        debug_assert!(offset + size <= vkutl::MAX_PUSH_CONSTANT_SIZE, "push constant size not valid");

        unsafe { std::ptr::copy_nonoverlapping(data as _, self.push_data.as_mut_ptr().add(offset), size) };

        self.push_range = self.push_range.max((offset + size) as u8);
    }

    pub fn create_draw_info(&self, frame_index: usize) -> DrawInfo {
        let index_buffer = self.vao.get_raw_buffer(BuffersTypes::Index).get_buffer(frame_index);
        let vertex_buffer = self.vao.get_raw_buffer(BuffersTypes::Vertex).get_buffer(frame_index);
        let instance_buffer = self.vao.get_raw_buffer(BuffersTypes::Instance).get_buffer(frame_index);

        let pipeline = self.pipeline.borrow();

        DrawInfo {
            pipeline: pipeline.get_pipeline(),
            pipeline_layout: pipeline.pipeline_layout.get_layout(),
            descriptors_sets: *pipeline.pipeline_layout.get_descriptors_sets(frame_index),
            descriptors_count: pipeline.pipeline_layout.descriptors_count,

            index_buffer: index_buffer,
            vertices_buffer: [vertex_buffer, instance_buffer],

            index_count: self.vao.triangles_count,
            instance_count: 0,

            push_constant_idx: -1,
        }
    }
}
