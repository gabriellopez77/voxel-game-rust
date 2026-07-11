use crate::render::material::MaterialType;
use crate::render::{GlobalRenderer, Material, SPRITES_INDICES, SPRITES_VERTICES, SpritesVertices, TextVertices};
use crate::render::raw_buffer::BufferFlags;


pub struct UiRenderer {
    sprites_instance_data: Vec<SpritesVertices>,
    text_instance_data: Vec<TextVertices>,

    sprites_material: Option<Material>,
    text_material: Option<Material>,

    layer: i32,
}

impl UiRenderer {
    pub const MAX_SPRITES_COUNT: usize = 750;

    pub fn new() -> Self {
        Self {
            sprites_instance_data: Vec::new(),
            text_instance_data: Vec::new(),

            sprites_material: None,
            text_material: None,

            layer: 0,
        }
    }

    pub fn start(&mut self, global_renderer: &mut GlobalRenderer) {
        let mut sprites_material = global_renderer.create_material("ui_sprites", MaterialType::Ui);
        sprites_material.set_mesh(&SPRITES_VERTICES, &SPRITES_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        sprites_material.create_instance_buffer(size_of::<SpritesVertices>() * Self::MAX_SPRITES_COUNT, None, BufferFlags::RAM);
        self.sprites_material = Some(sprites_material);

        let mut text_material = global_renderer.create_material("ui_text", MaterialType::Ui);
        text_material.set_mesh(&SPRITES_VERTICES, &SPRITES_INDICES, BufferFlags::VRAM | BufferFlags::ONCE);
        text_material.create_instance_buffer(size_of::<TextVertices>() * Self::MAX_SPRITES_COUNT, None, BufferFlags::RAM);
        self.text_material = Some(text_material);
    }

    pub fn draw(&mut self, global_renderer: &mut GlobalRenderer) {
        let sprites_material = self.sprites_material.as_mut().unwrap();
        global_renderer.draw_obj_instanced_with_buffer(sprites_material, &mut self.sprites_instance_data);

        let text_material = self.text_material.as_mut().unwrap();
        global_renderer.draw_obj_instanced_with_buffer(text_material, &mut self.text_instance_data);
    }

    pub fn cleanup(&mut self) {
        self.sprites_material.as_mut().unwrap().destroy();
        self.text_material.as_mut().unwrap().destroy();
    }

    pub fn get_sprites_count(&self) -> usize { self.sprites_instance_data.len() }
    pub fn get_text_count(&self) -> usize { self.text_instance_data.len() }

    pub fn add_sprite(&mut self, data: SpritesVertices) { self.sprites_instance_data.push(data) }
    pub fn add_text(&mut self, data: TextVertices) { self.text_instance_data.push(data) }

    /// increment current layer value and return last value
    pub fn inc_layer(&mut self) -> i32 {
        let last = self.layer;

        self.layer += 1;

        return last;
    }
}
