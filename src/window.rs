use std::cell::{Ref, RefCell};
use std::rc::Rc;

use glfw::{Context, WindowEvent};
use crate::inputs;
use crate::game::Game;


pub struct Window {
    glfw_instance: glfw::Glfw,
    window: glfw::PWindow,

    width: i32,
    height: i32,

    last_frame: f32,
}

impl Window {
    pub fn init(width: i32, height: i32, title: &str) -> (Self, glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
        let mut glfw_instance = glfw::init(|error, description| glfw::fail_on_errors(error, description)).unwrap();
        let (mut window, events) =
            glfw_instance.create_window(width as u32, height as u32, title, glfw::WindowMode::Windowed).unwrap();


        window.make_current();

        // set pollings
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);

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

    pub fn run(&mut self, game: &mut Game, events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
        game.start();

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
        }

        game.resize(self.width, self.height);


        while !self.window.should_close() {
            inputs::new_frame();
            
            self.glfw_instance.poll_events();

            for (_, event) in glfw::flush_messages(&events) {
                self.roll_events(event, game);
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
        match event {
            WindowEvent::FramebufferSize(width, heigth) => self.resize_callback(game, width, heigth),
            WindowEvent::Key(key, _, action, _) => self.key_callback(key, action),
            WindowEvent::MouseButton(button, action, _) => self.mouse_button_callback(button, action),
            WindowEvent::CursorPos(x, y) => self.mouse_move_callback(x, y),
            _ => {}
        }
    }

    fn resize_callback(&mut self, game: &mut Game, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);

            self.width = width;
            self.height = height;


            game.resize(width, height);
        }
    }

    fn key_callback(&mut self, key: glfw::Key, action: glfw::Action) {
        if key == glfw::Key::Unknown { return }

        inputs::set_key_state(key as i32, action != glfw::Action::Release);
    }

    fn mouse_button_callback(&mut self, button: glfw::MouseButton, action: glfw::Action) {
        inputs::set_mouse_button_state(button as i32, action != glfw::Action::Release);
    }

    fn mouse_move_callback(&mut self, x: f64, y: f64) {
        inputs::set_mouse_pos(x as f32, y as f32);
    }
}