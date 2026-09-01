use std::{cell::RefCell, collections::HashMap, mem::offset_of, rc::Rc};
use ash::{vk, vk::Handle};

use crate::{math::Vec3, render::{BlockItemVertices, ChunkVertices, CloudsVertices, DrawInfo, EntitiesCubesVertices, GlobalUboData, Material, Mesh, MultiMesh, ParticlesVertices, SkyBodiesVertices, SpritesVertices, TextVertices, Ubo, core::raw_buffer::BufferResizeMode, draw_info::DrawType, material::{MaterialType, VertexAttribInfo}, mesh::BuffersTypes}, resources::{ResourceManager, ShadersCompiler}, utils::SafePtrMut};
use super::core::{vkutl, VulkanApp, DescriptorSet, PipelineLayout, raw_buffer::BufferFlags};


pub struct GlobalRenderer {
    pub app: SafePtrMut<VulkanApp>,

    pub global_pipeline_layout: PipelineLayout,
    pub global_ubo: Ubo<GlobalUboData>,
    pub global_descriptor: DescriptorSet,

    pub shaders_compiler: ShadersCompiler,

    default_materials: HashMap<&'static str, Rc<RefCell<Material>>>,

    sky_draw_list: Vec<DrawInfo>,
    chunks_opaque_draw_list: Vec<DrawInfo>,
    chunks_alpha_draw_list: Vec<DrawInfo>,
    opaque_draw_list: Vec<DrawInfo>,
    alpha_draw_list: Vec<DrawInfo>,
    ui_draw_list: Vec<DrawInfo>,
    particle_draw_list: Vec<DrawInfo>,
    first_person_draw_list: Vec<DrawInfo>,

    push_constant_list: Vec<(u8, [u8; vkutl::MAX_PUSH_CONSTANT_SIZE])>,
    push_constant_idx: i32,

    frame_index: usize,

    //pipeline_layouts: Vec<(vk::PipelineLayout, [vk::DescriptorSetLayout; vkutl::MAX_DESCRIPTORS_BINDING_COUNT])>,
}

impl GlobalRenderer {
    pub const WORLD_TEXTURE_IDX: u8 = 0;
    pub const UI_SPRITES_TEXTURE_IDX: u8 = 1;

    pub fn new(app: &mut VulkanApp) -> Self {
        Self {
            app: SafePtrMut::new(app),

            global_pipeline_layout: PipelineLayout::new(),
            global_ubo: Ubo::new(),
            global_descriptor: DescriptorSet::new(),

            shaders_compiler: ShadersCompiler::new(),

            default_materials: HashMap::new(),

            sky_draw_list: Vec::new(),
            chunks_opaque_draw_list: Vec::new(),
            chunks_alpha_draw_list: Vec::new(),
            opaque_draw_list: Vec::new(),
            alpha_draw_list: Vec::new(),
            ui_draw_list: Vec::new(),
            particle_draw_list: Vec::new(),
            first_person_draw_list: Vec::new(),

            push_constant_list: Vec::new(),
            push_constant_idx: -1,

            frame_index: 0,

            //pipeline_layouts: Vec::new(),
        }
    }

    pub fn start(&mut self, resources: &mut ResourceManager) {
        self.shaders_compiler.start(resources.shader_path.clone());

        self.global_ubo.create(&mut self.app, BufferFlags::RAM);

        let used_stages_flag = vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT;

        self.global_descriptor.add_indexing_textures(0, used_stages_flag, &mut [
                &mut resources.world_texture.raw_texture,
                &mut resources.ui_sprites_texture.raw_texture,
                &mut resources.ui_fonts_texture.raw_texture,
                &mut resources.sky_bodies_texture.raw_texture,
            ])
            .add_ubo(1, used_stages_flag, self.global_ubo.buffer.get_all_buffers())
            .create(&self.app);

        self.global_pipeline_layout.add_descriptor(&self.global_descriptor);
        self.global_pipeline_layout.create(&self.app, vk::PipelineLayout::null());

        // create pipelines

        {
            let mut material = self.create_material("chunk", MaterialType::ChunksOpaque);
            material.set_attributes_info(*VertexAttribInfo::default()
                .add_vertex(size_of::<ChunkVertices>(), false)
                .add_attribute(vk::Format::R32G32B32_SFLOAT, offset_of!(ChunkVertices, vertices))
                .add_attribute(vk::Format::R32G32B32_SFLOAT, offset_of!(ChunkVertices, normal))
                .add_attribute(vk::Format::R32G32_SFLOAT, offset_of!(ChunkVertices, uv))
                .add_attribute(vk::Format::R8_UINT, offset_of!(ChunkVertices, flags))
                .add_attribute(vk::Format::R8_UINT, offset_of!(ChunkVertices, light))
            );

            self.default_materials.insert("chunks", Rc::new(RefCell::new(material)));
        }
        {
            let mut material = self.create_material("chunk", MaterialType::ChunksAlpha);
            material.set_blend(true);
            material.set_cull_mode(vk::CullModeFlags::NONE);
            material.set_attributes_info(*VertexAttribInfo::default()
                .add_vertex(size_of::<ChunkVertices>(), false)
                .add_attribute(vk::Format::R32G32B32_SFLOAT, offset_of!(ChunkVertices, vertices))
                .add_attribute(vk::Format::R32G32B32_SFLOAT, offset_of!(ChunkVertices, normal))
                .add_attribute(vk::Format::R32G32_SFLOAT, offset_of!(ChunkVertices, uv))
                .add_attribute(vk::Format::R8_UINT, offset_of!(ChunkVertices, flags))
                .add_attribute(vk::Format::R8_UINT, offset_of!(ChunkVertices, light))
            );

            self.default_materials.insert("chunksAlpha", Rc::new(RefCell::new(material)));
        }
        {
            let mut material = self.create_material(r"ui\sprites", MaterialType::Ui);
            material.set_blend(true);
            material.set_depth_test(false);
            material.set_attributes_info(*VertexAttribInfo::default()
                .add_vertex(4 * size_of::<f32>(), false)
                .add_attribute( vk::Format::R32G32B32A32_SFLOAT, 0)
                .add_vertex(size_of::<SpritesVertices>(), true)
                .add_attribute(vk::Format::R16G16_SINT, offset_of!(SpritesVertices, position))
                .add_attribute(vk::Format::R16G16_SINT, offset_of!(SpritesVertices, size))
                .add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset_of!(SpritesVertices, uv))
                .add_attribute(vk::Format::R8G8B8A8_UINT, offset_of!(SpritesVertices, color))
                .add_attribute(vk::Format::R8_UINT, offset_of!(SpritesVertices, texture_idx))
            );

            self.default_materials.insert("ui_sprites", Rc::new(RefCell::new(material)));
        }
        {
            let mut material = self.create_material(r"ui\text", MaterialType::Ui);
            material.set_blend(true);
            material.set_depth_test(false);
            material.set_attributes_info(*VertexAttribInfo::default()
                .add_vertex(4 * size_of::<f32>(), false)
                .add_attribute( vk::Format::R32G32B32A32_SFLOAT, 0)
                .add_vertex(size_of::<TextVertices>(), true)
                .add_attribute(vk::Format::R16G16_SINT, offset_of!(TextVertices, position))
                .add_attribute(vk::Format::R8G8_UINT, offset_of!(TextVertices, size))
                .add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset_of!(TextVertices, uv))
                .add_attribute(vk::Format::R16G16_SINT, offset_of!(TextVertices, advance))
                .add_attribute(vk::Format::R8G8B8_UINT, offset_of!(TextVertices, color))
            );

            self.default_materials.insert("ui_text", Rc::new(RefCell::new(material)));
        }
        {
            let mut material = self.create_material(r"clouds", MaterialType::Alpha);
            material.set_blend(true);
            material.set_attributes_info(*VertexAttribInfo::default()
                .add_vertex(7 * size_of::<i8>(), false)
                .add_attribute(vk::Format::R8G8B8_SINT, 0)
                .add_attribute(vk::Format::R8G8B8_SINT, 3 * size_of::<i8>())
                .add_attribute(vk::Format::R8_SINT, 6 * size_of::<i8>())
                .add_vertex(size_of::<CloudsVertices>(), true)
                .add_attribute(vk::Format::R32G32_SFLOAT, offset_of!(CloudsVertices, position))
                .add_attribute(vk::Format::R8_UINT, offset_of!(CloudsVertices, cullface))
            );

            self.default_materials.insert("clouds", Rc::new(RefCell::new(material)));
        }
        {
            let mut material = self.create_material(r"skyDome", MaterialType::Sky);
            material.set_depth_test(false);
            material.set_attributes_info(*VertexAttribInfo::default()
                .add_vertex(size_of::<Vec3>(), false)
                .add_attribute(vk::Format::R32G32B32_SFLOAT, 0)
            );

            self.default_materials.insert("skyDome", Rc::new(RefCell::new(material)));
        }
        {
            let mut material = self.create_material(r"selectionBox", MaterialType::Alpha);
            material.set_blend(true);
            material.set_topology(vk::PrimitiveTopology::LINE_LIST);
            material.set_line_width(3.0);
            material.set_attributes_info(*VertexAttribInfo::default()
                .add_vertex(3, false)
                .add_attribute(vk::Format::R8G8B8_SINT, 0)
            );

            self.default_materials.insert("selectionBox", Rc::new(RefCell::new(material)));
        }
        {
            let mut material = self.create_material(r"skyBodies", MaterialType::Sky);
            material.set_blend(true);
            material.set_depth_test(false);
            material.set_attributes_info(*VertexAttribInfo::default()
                .add_vertex(5 * size_of::<f32>(), false)
                .add_attribute(vk::Format::R32G32B32_SFLOAT, 0)
                .add_attribute(vk::Format::R32G32_SFLOAT, 3 * size_of::<f32>())
                .add_vertex(size_of::<SkyBodiesVertices>(), true)
                .add_attribute_matrix(offset_of!(SkyBodiesVertices, matrix))
                .add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset_of!(SkyBodiesVertices, uv))
                .add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset_of!(SkyBodiesVertices, color))
            );

            self.default_materials.insert("skyBodies", Rc::new(RefCell::new(material)));
        }
        {
            let mut material = self.create_material(r"particles", MaterialType::Particle);
            material.set_blend(true);
            material.set_attributes_info(*VertexAttribInfo::default()
                .add_vertex(5 * size_of::<f32>(), false)
                .add_attribute(vk::Format::R32G32B32_SFLOAT, 0)
                .add_attribute(vk::Format::R32G32_SFLOAT, 3 * size_of::<f32>())
                .add_vertex(size_of::<ParticlesVertices>(), true)
                .add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset_of!(ParticlesVertices, position))
                .add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset_of!(ParticlesVertices, scale))
                .add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset_of!(ParticlesVertices, rotation))
                .add_attribute(vk::Format::R32G32B32A32_SFLOAT, offset_of!(ParticlesVertices, uv))
                .add_attribute(vk::Format::R8_UINT, offset_of!(ParticlesVertices, texture_idx))
            );

            self.default_materials.insert("particles", Rc::new(RefCell::new(material)));
        }
        {
            let mut material = self.create_material(r"firstPerson", MaterialType::FirstPerson);
            material.set_blend(true);
            material.set_attributes_info(*VertexAttribInfo::default()
                .add_vertex(size_of::<BlockItemVertices>(), false)
                .add_attribute(vk::Format::R32G32B32_SFLOAT, offset_of!(BlockItemVertices, vertices))
                .add_attribute(vk::Format::R32G32B32_SFLOAT, offset_of!(BlockItemVertices, normal))
                .add_attribute(vk::Format::R32G32_SFLOAT, offset_of!(BlockItemVertices, uv))
            );

            self.default_materials.insert("firstPerson", Rc::new(RefCell::new(material)));
        }
        {
            let mut material = self.create_material("entities", MaterialType::Alpha);
            material.set_blend(true);
            material.set_cull_mode(vk::CullModeFlags::NONE);
            material.set_attributes_info(*VertexAttribInfo::default()
                .add_vertex(size_of::<f32>() * 9, false)
                .add_attribute(vk::Format::R32G32B32_SFLOAT, 0)
                .add_attribute(vk::Format::R32G32B32_SFLOAT, size_of::<f32>() * 3)
                .add_attribute(vk::Format::R32G32B32_SFLOAT, size_of::<f32>() * 6)
                .add_vertex(size_of::<EntitiesCubesVertices>(), true)
                .add_attribute_array_vec4(offset_of!(EntitiesCubesVertices, up_tex_coords), 6)
                .add_attribute(vk::Format::R32G32_UINT, offset_of!(EntitiesCubesVertices, color))
                .add_attribute_matrix(offset_of!(EntitiesCubesVertices, local_matrix))
            );

            self.default_materials.insert("entities", Rc::new(RefCell::new(material)));
        }
    }

    pub fn cleanup(&mut self) {
        self.global_descriptor.destroy(&mut self.app);
        self.global_ubo.destroy(&mut self.app);
    }

    //pub fn create_pipeline_layout(&mut self, layout: &mut PipelineLayout) {
    //    let shared_layout = match self.pipeline_layouts.iter().find(|x| layout.is_same(&x.1)) {
    //        Some(layout) => layout.0,
    //        None => vk::PipelineLayout::null()
    //    };

    //    layout.create(&self.app, shared_layout);

    //    // save shared layout
    //    if shared_layout.is_null() {
    //        self.pipeline_layouts.push((layout.get_layout(), layout.descriptors_layout));
    //    }
    //}

    pub fn create_mesh_and_get_material(&self, name: &'static str) -> (Mesh, Rc<RefCell<Material>>) {
        (self.create_mesh(), self.get_material(name))
    }

    pub fn create_mesh(&self) -> Mesh {
        Mesh::new(self.app.clone())
    }

    pub fn create_multi_mesh(&self, vertices_size: usize) -> MultiMesh {
        MultiMesh::new(self.app.clone(), vertices_size)
    }

    pub fn create_material(&self, name: &'static str, material_type: MaterialType) -> Material {
        Material::new(self.app.clone(), name, material_type)
    }

    pub fn get_material(&self, name: &'static str) -> Rc<RefCell<Material>> {
        self.default_materials.get(name).unwrap().clone()
    }



    pub fn set_push_constant<T>(&mut self, offset: usize, data: *const T) {
        let size = size_of::<T>();

        debug_assert!(offset + size <= vkutl::MAX_PUSH_CONSTANT_SIZE, "push constant size not valid");


        if self.push_constant_idx == -1 {
            self.push_constant_idx = self.push_constant_list.len() as i32;
            self.push_constant_list.push((size as u8, [0u8; vkutl::MAX_PUSH_CONSTANT_SIZE]));
        }

        let (push_size, push_data) = &mut self.push_constant_list[self.push_constant_idx as usize];
        *push_size = (*push_size).max((offset + size) as u8);

        unsafe {
            std::ptr::copy_nonoverlapping(data as _, push_data.as_mut_ptr().byte_add(offset), size);
        }
    }

    pub fn draw_multi_mesh(&mut self, multi_mesh: &MultiMesh, material: &mut Material, profile_idx: usize) {
        self.prepare_draw_info(
            material,
            multi_mesh.get_buffers(self.frame_index, profile_idx),
            DrawType::Indirect(multi_mesh.get_profile_draw_count(profile_idx))
        );
    }

    pub fn draw_instanced(&mut self, mesh: &Mesh, material: &mut Material, instance_count: usize) {
        self.prepare_draw_info(
            material,
            mesh.get_buffers(self.frame_index),
            DrawType::Default(mesh.get_index_count(), instance_count as u32)
        );
    }

    pub fn draw_instanced_with_buffer<T>(&mut self,
        mesh: &mut Mesh,
        material: &mut Material,
        instance_data: &mut Vec<T>,
        resize_mode: BufferResizeMode
    ) {
        if self.prepare_draw_info(
            material,
            mesh.get_buffers(self.frame_index),
            DrawType::Default(mesh.get_index_count(), instance_data.len() as u32)
        ) {
            mesh.update_instance_buffer(&instance_data, resize_mode);
            instance_data.clear();
        }
    }

    pub fn draw(&mut self, mesh: &Mesh, material: &mut Material) {
        self.prepare_draw_info(
            material,
            mesh.get_buffers(self.frame_index),
            DrawType::Default(mesh.get_index_count(), 1)
        );
    }



    pub fn begin(&mut self) {
        self.push_constant_list.clear();

        self.sky_draw_list.clear();
        self.chunks_opaque_draw_list.clear();
        self.chunks_alpha_draw_list.clear();
        self.opaque_draw_list.clear();
        self.alpha_draw_list.clear();
        self.particle_draw_list.clear();
        self.first_person_draw_list.clear();
        self.ui_draw_list.clear();

        self.frame_index = self.app.frame_index;
    }

    pub fn end(&mut self) {
        self.app.render_pass_begin();

        //let now = std::time::Instant::now();
        self.opaque_draw_list.sort();
        self.alpha_draw_list.sort();
        //println!("{}", now.elapsed().as_micros());

        //let now = std::time::Instant::now();
        self.render(&self.sky_draw_list);
        self.render(&self.chunks_opaque_draw_list);
        self.render(&self.opaque_draw_list);
        self.render(&self.alpha_draw_list);
        self.render(&self.particle_draw_list);
        self.render(&self.chunks_alpha_draw_list);
        self.render(&self.first_person_draw_list);
        self.render(&self.ui_draw_list);
        //println!("{}", now.elapsed().as_micros());
    }

    fn render(&self, draw_list: &Vec<DrawInfo>) {
        let mut current_pipeline = vk::Pipeline::null();
        let mut current_pipeline_layout = vk::PipelineLayout::null();
        let mut current_descriptor_sets = [vk::DescriptorSet::null(); vkutl::MAX_DESCRIPTORS_BINDING_COUNT];

        let vulkan_app = &*self.app;

        let command_buffer = vulkan_app.get_graphics_cmd();


        for draw_info in draw_list {
            if current_pipeline != draw_info.pipeline {
                current_pipeline = draw_info.pipeline;

                unsafe {
                    vulkan_app.ash_device.cmd_bind_pipeline(command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        current_pipeline
                    );
                }
            }

            if current_pipeline_layout != draw_info.pipeline_layout {
                current_pipeline_layout = draw_info.pipeline_layout;

                let mut first_set = usize::MAX;

                for i in 0..draw_info.descriptors_count as usize {
                    if draw_info.descriptors_sets[i] != current_descriptor_sets[i] {
                        current_descriptor_sets[i] = draw_info.descriptors_sets[i];

                        if first_set == usize::MAX {
                            first_set = i;
                        }
                    }
                }

                unsafe {
                    vulkan_app.ash_device.cmd_bind_descriptor_sets(command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        current_pipeline_layout,
                        first_set as u32,
                        &draw_info.descriptors_sets[first_set..draw_info.descriptors_count as usize],
                        &[]
                    );
                }
            }

            if draw_info.push_constant_idx != -1 {
                let (push_size, push_data) = &self.push_constant_list[draw_info.push_constant_idx as usize];

                unsafe {
                    vulkan_app.ash_device.cmd_push_constants(command_buffer,
                        current_pipeline_layout,
                        vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
                        0,
                        &push_data[0..*push_size as usize]
                    );
                }
            }


            let offsets = [0u64; vkutl::MAX_VERTEX_BINDING_COUNT];

            unsafe {
                vulkan_app.ash_device.cmd_bind_index_buffer(command_buffer,
                    draw_info.index_buffer,
                    0,
                    vk::IndexType::UINT32
                );


                match draw_info.draw_type {
                    DrawType::Default(index_count, instance_count) => {
                        let count = if draw_info.buffers[BuffersTypes::Custom as usize].is_null() { 1 } else { 2 };

                        vulkan_app.ash_device.cmd_bind_vertex_buffers(command_buffer,
                            0,
                            &draw_info.buffers[0..count],
                            &offsets[0..count],
                        );

                        vulkan_app.ash_device.cmd_draw_indexed(command_buffer,
                            index_count,
                            instance_count,
                            0, 0, 0
                        );
                    }
                    DrawType::Indirect(draw_count) => {
                        vulkan_app.ash_device.cmd_bind_vertex_buffers(command_buffer,
                            0,
                            &draw_info.buffers[0..1],
                            &offsets[0..1],
                        );

                        vulkan_app.ash_device.cmd_draw_indexed_indirect(command_buffer,
                            draw_info.buffers[BuffersTypes::Custom as usize],
                            0,
                            draw_count,
                            size_of::<vk::DrawIndexedIndirectCommand>() as u32
                        );
                    }
                }

            }
        }
    }

    fn prepare_draw_info(&mut self,
        material: &mut Material,
        buffers: [vk::Buffer; vkutl::MAX_BUFFERS_REQUIRED_TO_DRAW_COUNT],
        draw_type: DrawType
    ) -> bool {
        let push_constant_idx = self.push_constant_idx;
        self.push_constant_idx = -1;

        match draw_type {
            DrawType::Default(index_count, instance_count) => if index_count == 0 || instance_count == 0 { return false }
            DrawType::Indirect(draw_count) => if draw_count == 0 { return false }
        }

        let (pipeline, pipeline_layout, descriptors_sets, descriptors_sets_count) =
            if let Some(info) = material.get_draw_info(self, self.frame_index) {
                info
            } else { return false };

        let draw_info = DrawInfo {
            pipeline: pipeline,
            pipeline_layout: pipeline_layout,
            descriptors_sets: descriptors_sets,
            descriptors_count: descriptors_sets_count,

            buffers: [buffers[BuffersTypes::Vertex as usize], buffers[BuffersTypes::Custom as usize]],
            index_buffer: buffers[BuffersTypes::Index as usize],

            draw_type,

            push_constant_idx: push_constant_idx,
        };


        match material.get_type() {
            MaterialType::ChunksOpaque => self.chunks_opaque_draw_list.push(draw_info),
            MaterialType::ChunksAlpha => self.chunks_alpha_draw_list.push(draw_info),
            MaterialType::Opaque => self.opaque_draw_list.push(draw_info),
            MaterialType::Alpha => self.alpha_draw_list.push(draw_info),
            MaterialType::Sky => self.sky_draw_list.push(draw_info),
            MaterialType::Ui => self.ui_draw_list.push(draw_info),
            MaterialType::Particle => self.particle_draw_list.push(draw_info),
            MaterialType::FirstPerson => self.first_person_draw_list.push(draw_info),
        }

        return true;
    }
}
