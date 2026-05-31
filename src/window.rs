use glfw::{Context, WindowEvent};
use crate::inputs;
use crate::game::Game;
use crate::render::render_utils;
use crate::ui::UiManager;

pub struct Window {
    glfw_instance: glfw::Glfw,
    window: glfw::PWindow,

    width: i32,
    height: i32,

    last_frame: f32,
}

impl Window {
    pub fn init(width: i32, height: i32, title: &str) -> (Self, glfw::GlfwReceiver<(f64, WindowEvent)>) {
        let mut glfw_instance = glfw::init(|error, description| glfw::fail_on_errors(error, description)).unwrap();
        let (mut window, events) =
            glfw_instance.create_window(width as u32, height as u32, title, glfw::WindowMode::Windowed).unwrap();

        window.set_size_limits(Some(1050), Some(650), None, None);

        window.make_current();

        // set pollings
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);

        glfw_instance.set_swap_interval(glfw::SwapInterval::Sync(1));

        // init opengl functions
        gl::load_with(|s| window.get_proc_address(s).unwrap() as *const std::ffi::c_void);

        window.set_cursor_mode(glfw::CursorMode::Disabled);

        return (Window {
            glfw_instance,
            window,
            width, height,
            last_frame: 0.0
        }, events);
    }

    pub fn run(&mut self, events: &glfw::GlfwReceiver<(f64, WindowEvent)>) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::CullFace(gl::BACK);
            render_utils::enable(render_utils::RenderCap::DepthTest);
            render_utils::enable(render_utils::RenderCap::CullFace);
        }
        
        let mut game = Game::new();
        //let mut ui_manager = UiManager::new();
        game.start();

        game.resize(self.width as f32, self.height as f32);


        while !self.window.should_close() {
            // update keyboard and mouse inupts
            inputs::new_frame();

            // poll window events
            self.glfw_instance.poll_events();

            for (_, event) in glfw::flush_messages(&events) {
                self.roll_events(event, &mut game);
            }

            // calculate delta time
            let time = self.glfw_instance.get_time() as f32;
            let dt = time - self.last_frame;
            self.last_frame = time;


            game.update(dt, self);

            unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }

            game.render();

            // checks for opengl erros
            unsafe {
                if gl::GetError() != gl::NO_ERROR { panic!("OpenGL error!") }
            }

            self.window.swap_buffers();
        }
    }

    pub fn set_cursor(&mut self, cursor: glfw::CursorMode) {
        self.window.set_cursor_mode(cursor);
    }

    fn roll_events(&mut self, event: WindowEvent, game: &mut Game) {
        inputs::roll_event(&event);

        match event {
            WindowEvent::FramebufferSize(width, heigth) => self.resize_callback(game, width, heigth),
            _ => {}
        }
    }

    fn resize_callback(&mut self, game: &mut Game, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);

            self.width = width;
            self.height = height;


            game.resize(width as f32, height as f32);
        }
    }
}
