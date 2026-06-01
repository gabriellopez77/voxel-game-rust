use std::mem::offset_of;
use std::rc::Rc;
use crate::math::Matrix4;
use crate::render::{render_utils, sprites_renderer, SpritesRenderer, SpritesVertices, TextVertices, Ubo, Vao, SPRITES_INDICES, SPRITES_VERTICES};
use crate::render::render_utils::RenderCap;
use crate::render::vao::VaoBuffers;
use crate::resources::ResourceManager;


pub struct UiRenderer {
    pub sprites: SpritesRenderer<SpritesVertices>,
    pub text: SpritesRenderer<TextVertices>,
    pub icons: SpritesRenderer<SpritesVertices>,

    pub ubo: Option<Rc<Ubo>>,

    layer: i32,
}

impl UiRenderer {
    pub fn new() -> Self {
        Self {
            sprites: SpritesRenderer::new(),
            text: SpritesRenderer::new(),
            icons: SpritesRenderer::new(),

            layer: 0,
            ubo: None,
        }
    }

    pub fn start(&mut self, resource_manager: &ResourceManager) {
        let mut sprites_vao = Vao::new();
        sprites_vao.gen_vao()
            .gen_buffer(VaoBuffers::Ebo)
            .gen_buffer(VaoBuffers::Vbo)
            .gen_buffer(VaoBuffers::Instance);

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


        self.sprites.start(sprites_vao,
            resource_manager.get_shader("ui/sprites"),
            resource_manager.get_texture("ui")
        );

        let mut text_vao = Vao::new();
        text_vao.gen_vao()
            .gen_buffer(VaoBuffers::Ebo)
            .gen_buffer(VaoBuffers::Vbo)
            .gen_buffer(VaoBuffers::Instance);

        text_vao.buffer_data_from_arr(VaoBuffers::Ebo, &SPRITES_INDICES, gl::STATIC_DRAW);

        text_vao.buffer_data_from_arr(VaoBuffers::Vbo, &SPRITES_VERTICES, gl::STATIC_DRAW)
            .attrib_info(0, 4, gl::FLOAT, 0, false)
            .set_stride(4 * size_of::<f32>());

        text_vao.buffer_data(VaoBuffers::Instance, size_of::<TextVertices>() * sprites_renderer::MAX_SPRITES, None, gl::DYNAMIC_DRAW)
            .attrib_info(1, 2, gl::SHORT, offset_of!(TextVertices, position), true)
            .attrib_info(2, 2, gl::UNSIGNED_BYTE, offset_of!(TextVertices, size), true)
            .attrib_info(3, 4, gl::FLOAT, offset_of!(TextVertices, uv), true)
            .attrib_info(4, 2, gl::SHORT, offset_of!(TextVertices, advance), true)
            .attrib_info(5, 4, gl::UNSIGNED_BYTE, offset_of!(TextVertices, color), true)
            .set_stride(size_of::<TextVertices>());

        self.text.start(text_vao,
            resource_manager.get_shader("ui/text"),
            resource_manager.get_texture("fonts")
        );

        let mut icons_vao = Vao::new();
        icons_vao.gen_vao()
            .gen_buffer(VaoBuffers::Ebo)
            .gen_buffer(VaoBuffers::Vbo)
            .gen_buffer(VaoBuffers::Instance);

        icons_vao.buffer_data_from_arr(VaoBuffers::Ebo, &SPRITES_INDICES, gl::STATIC_DRAW);

        icons_vao.buffer_data_from_arr(VaoBuffers::Vbo, &SPRITES_VERTICES, gl::STATIC_DRAW)
            .attrib_info(0, 4, gl::FLOAT, 0, false)
            .set_stride(4 * size_of::<f32>());

        icons_vao.buffer_data(VaoBuffers::Instance, size_of::<SpritesVertices>() * sprites_renderer::MAX_SPRITES, None, gl::DYNAMIC_DRAW)
            .attrib_info(1, 2, gl::SHORT, offset_of!(SpritesVertices, position), true)
            .attrib_info(2, 2, gl::SHORT, offset_of!(SpritesVertices, size), true)
            .attrib_info(3, 4, gl::FLOAT, offset_of!(SpritesVertices, uv), true)
            .attrib_info(4, 4, gl::UNSIGNED_BYTE, offset_of!(SpritesVertices, color), true)
            .set_stride(size_of::<SpritesVertices>());


        self.icons.start(icons_vao,
                           resource_manager.get_shader("ui/sprites"),
                           resource_manager.get_texture("blocks")
        );
        
        self.ubo = resource_manager.get_ubo("globalData");
    }

    pub fn draw(&mut self) {
        render_utils::disable(RenderCap::DepthTest);
        render_utils::enable(RenderCap::Blend);
        self.sprites.draw();
        self.icons.draw();
        self.text.draw();
        render_utils::disable(RenderCap::Blend);
        render_utils::enable(RenderCap::DepthTest);
    }

    pub fn resize(&mut self, width: f32, height: f32, pixel_scale: f32) {
        let projection = Matrix4::orthographic(0.0, width, height, 0.0);

        self.ubo.as_ref().unwrap().update("uiProj", projection.as_ptr());
        self.ubo.as_ref().unwrap().update("uiPixelScale", &pixel_scale);
    }

    /// increment current layer value and return last value
    pub fn inc_layer(&mut self) -> i32 {
        let last = self.layer;

        self.layer += 1;

        return last;
    }
}