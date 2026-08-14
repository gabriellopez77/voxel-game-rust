use crate::render::core::raw_buffer::BufferResizeMode;
use crate::render::material::MaterialType;
use crate::render::{GlobalRenderer, Material, Mesh, SPRITES_INDICES, SPRITES_VERTICES, SpritesVertices, TextVertices};
use super::core::raw_buffer::BufferFlags;


pub struct UiRenderer {
    sprites_instance_data: Vec<SpritesVertices>,
    text_instance_data: Vec<TextVertices>,

    sprites_renderer: Option<(Mesh, Material)>,
    text_renderer: Option<(Mesh, Material)>,
}

impl UiRenderer {
    pub fn new() -> Self {
        Self {
            sprites_instance_data: Vec::new(),
            text_instance_data: Vec::new(),

            sprites_renderer: None,
            text_renderer: None,
        }
    }

    pub fn start(&mut self, global_renderer: &mut GlobalRenderer) {
        let (mut mesh, material) = global_renderer.create_mesh_material("ui_sprites", MaterialType::Ui);
        mesh.set(&SPRITES_VERTICES, &SPRITES_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        mesh.create_instance_buffer(size_of::<SpritesVertices>() * 64, None, BufferFlags::RAM);
        self.sprites_renderer = Some((mesh, material));

        let (mut mesh, material) = global_renderer.create_mesh_material("ui_text", MaterialType::Ui);
        mesh.set(&SPRITES_VERTICES, &SPRITES_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        mesh.create_instance_buffer(size_of::<TextVertices>() * 64, None, BufferFlags::RAM);
        self.text_renderer = Some((mesh, material));
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        let sprites_renderer = self.sprites_renderer.as_mut().unwrap();
        global_renderer.draw_instanced_with_buffer(&mut sprites_renderer.0, &mut sprites_renderer.1, &mut self.sprites_instance_data, BufferResizeMode::Discard);

        let text_renderer = self.text_renderer.as_mut().unwrap();
        global_renderer.draw_instanced_with_buffer(&mut text_renderer.0, &mut text_renderer.1, &mut self.text_instance_data, BufferResizeMode::Discard);
    }

    pub fn cleanup(&mut self) {
        let sprites_renderer = self.sprites_renderer.as_mut().unwrap();
        sprites_renderer.0.destroy();
        sprites_renderer.1.destroy();

        let text_renderer = self.text_renderer.as_mut().unwrap();
        text_renderer.0.destroy();
        text_renderer.1.destroy();
    }

    pub fn add_sprite(&mut self, data: SpritesVertices) { self.sprites_instance_data.push(data) }
    pub fn add_text(&mut self, data: TextVertices) { self.text_instance_data.push(data) }
}
