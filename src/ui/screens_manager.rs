use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::offset_of;
use std::rc::Rc;

use crate::math::{self, Vec2};
use crate::render::sprites_renderer;
use crate::render::{SPRITES_VERTICES, SPRITES_INDICES, SpritesRenderer, SpritesVertices, TextVertices, Ubo, Vao, vao::VaoBuffers};
use crate::resources::ResourceManager;
use crate::ui::{screen_base::ScreenBase, screens::StartScreen};


pub struct ScreenManager {
    sprites_renderer: SpritesRenderer<SpritesVertices>,
    text_renderer: SpritesRenderer<TextVertices>,
    sprites_ubo: Ubo,

    resource_manager: Option<Rc<RefCell<ResourceManager>>>,

    pixel_scale: i32,
    screen_size: Vec2,

    current_screen: Option<Rc<RefCell<dyn ScreenBase>>>,
    screens: HashMap<TypeId, Rc<RefCell<dyn ScreenBase>>>,
}

impl ScreenManager {
    pub fn new() -> Self {
        Self {
            sprites_renderer: SpritesRenderer::new(),
            text_renderer: SpritesRenderer::new(),
            sprites_ubo: Ubo::new(),

            resource_manager: None,

            pixel_scale: 3,
            screen_size: Vec2::ZERO,

            screens: HashMap::new(),
            current_screen: None,
        }
    }

    pub fn start(&mut self, resource_manager: Rc<RefCell<ResourceManager>>) {
        let mut sprites_vao = Vao::new();
        sprites_vao.gen_vao()
            .gen_buffer(gl::ELEMENT_ARRAY_BUFFER, VaoBuffers::Ebo)
            .gen_buffer(gl::ARRAY_BUFFER, VaoBuffers::Vbo)
            .gen_buffer(gl::ARRAY_BUFFER, VaoBuffers::Instance);
        
        sprites_vao.buffer_data_from_arr(VaoBuffers::Ebo, &SPRITES_INDICES, gl::STATIC_DRAW);

        sprites_vao.buffer_data_from_arr(VaoBuffers::Vbo, &SPRITES_VERTICES, gl::STATIC_DRAW)
            .attrib_info(0, 4, gl::FLOAT, 0, false)
            .set_stride(4 * size_of::<f32>());

        sprites_vao.buffer_data(VaoBuffers::Instance, size_of::<SpritesVertices>() * sprites_renderer::MAX_SPRITES, None, gl::DYNAMIC_DRAW)
            .attrib_info(1, 2, gl::SHORT, offset_of!(SpritesVertices, position), true)
            .attrib_info(2, 2, gl::SHORT, offset_of!(SpritesVertices, size), true)
            .attrib_info(3, 4, gl::FLOAT, offset_of!(SpritesVertices, uv), true)
            .attrib_info(4, 4, gl::UNSIGNED_BYTE, offset_of!(SpritesVertices, color), true)
            .set_stride(size_of::<SpritesVertices>());


        self.sprites_renderer.start(
            sprites_vao,
            resource_manager.borrow().get_shader("ui/sprites"),
            resource_manager.borrow().get_texture("ui")
        );

        let mut text_vao = Vao::new();
        text_vao.gen_vao()
            .gen_buffer(gl::ELEMENT_ARRAY_BUFFER, VaoBuffers::Ebo)
            .gen_buffer(gl::ARRAY_BUFFER, VaoBuffers::Vbo)
            .gen_buffer(gl::ARRAY_BUFFER, VaoBuffers::Instance);

        text_vao.buffer_data_from_arr(VaoBuffers::Ebo, &SPRITES_INDICES, gl::STATIC_DRAW);

        text_vao.buffer_data_from_arr(VaoBuffers::Vbo, &SPRITES_VERTICES, gl::STATIC_DRAW)
            .attrib_info(0, 4, gl::FLOAT, 0, false)
            .set_stride(4 * size_of::<f32>());

        text_vao.buffer_data(VaoBuffers::Instance, size_of::<TextVertices>() * sprites_renderer::MAX_SPRITES, None, gl::DYNAMIC_DRAW)
            .attrib_info(1, 2, gl::SHORT, offset_of!(TextVertices, position), true)
            .attrib_info(2, 2, gl::SHORT, offset_of!(TextVertices, size), true)
            .attrib_info(3, 4, gl::FLOAT, offset_of!(TextVertices, uv), true)
            .attrib_info(4, 2, gl::SHORT, offset_of!(TextVertices, advance), true)
            .attrib_info(5, 4, gl::UNSIGNED_BYTE, offset_of!(TextVertices, color), true)
            .set_stride(size_of::<TextVertices>());

        self.text_renderer.start(
            text_vao,
            resource_manager.borrow().get_shader("ui/text"),
            resource_manager.borrow().get_texture("font")
        );

        self.resource_manager = Some(resource_manager.clone());
        self.sprites_ubo.add::<math::Matrix4>("projection");
        self.sprites_ubo.create(0);

        self.screens.insert(TypeId::of::<StartScreen>(), Rc::new(RefCell::new(StartScreen::new())));

        self.change::<StartScreen>();
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.screen_size = Vec2 { x: width, y: height };

        let projection = math::Matrix4::orthographic(0.0, width, height, 0.0);
        self.sprites_ubo.update("projection", projection.as_ptr() as *const ());

        self.current_screen.as_ref().unwrap().borrow_mut().resize(width, height);
    }

    pub fn update(&self, dt: f32) {
        self.current_screen.as_ref().unwrap().borrow_mut().update(dt);
    }

    pub fn draw(&mut self) {
        self.current_screen.as_ref().unwrap().borrow_mut().draw(&mut self.sprites_renderer, &mut self.text_renderer);

        self.sprites_renderer.draw()
    }

    pub fn change<T: ScreenBase>(&mut self) {
        let new_screen_id = TypeId::of::<StartScreen>();
        let new_screen = self.screens[&new_screen_id].clone();

        new_screen.borrow_mut().change_logic(self.screen_size.x, self.screen_size.y, self.resource_manager.as_ref().unwrap().clone());
        self.current_screen = Some(new_screen);
    }
}