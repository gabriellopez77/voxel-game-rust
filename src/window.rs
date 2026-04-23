use glfw::Context;
use crate::inputs;
use crate::game::Game;


static mut WINDOW_INSTANCE: *mut Window = std::ptr::null_mut();

pub struct Window {
    glfw_instance: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,

    game: Game,

    width: i32,
    height: i32,

    last_frame: f32,
}

impl Window {
    pub fn init(width: i32, height: i32, title: &str) -> Self {
        use glfw::fail_on_errors;
        let mut glfw_instance = glfw::init(glfw::fail_on_errors!()).unwrap();
        let (mut window, events) =
            glfw_instance.create_window(width as u32, height as u32, title, glfw::WindowMode::Windowed).unwrap();


        window.make_current();
        glfw_instance.set_swap_interval(glfw::SwapInterval::Sync(1));

        // init opengl functions
        gl::load_with(|s| window.get_proc_address(s).unwrap() as *const std::ffi::c_void);

        // set callbacks
        window.set_framebuffer_size_callback(|_window, width, height| Self::resize_callback(width, height));
        window.set_key_callback(|_window, key, _scancode, action, _modifiers| Self::key_callback(key, action));
        window.set_mouse_button_callback(|_window, button, action, _modifiers| Self::mouse_button_callback(button, action));
        window.set_cursor_pos_callback(|_window, x, y| Self::mouse_move_callback(x, y));

        window.set_cursor_mode(glfw::CursorMode::Disabled);
        
        return Window { glfw_instance, window, events, width, height, game: Game::new(), last_frame: 0.0 };
    }

    pub fn set_window_instance(window: &mut Window) {
        unsafe {
            WINDOW_INSTANCE = window;
        }
    }

    pub fn run(&mut self) {
        self.game.start();

        unsafe {gl::ClearColor(0.0, 0.0, 0.0, 0.0)}

        self.game.resize(self.width, self.height);

        while !self.window.should_close() {
            inputs::new_frame();

            self.glfw_instance.poll_events();

            // calculate delta time
            let time = self.glfw_instance.get_time() as f32;
            let dt = time - self.last_frame;
            self.last_frame = time;

            self.game.update(dt);

            unsafe {gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT)}

            self.game.render();

            // checks for opengl erros
            unsafe {
                if gl::GetError() != gl::NO_ERROR { panic!("OpenGL error!") }
            }

            self.window.swap_buffers();
        }
    }

    fn resize_callback(width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
            
            let w = &mut *WINDOW_INSTANCE;

            w.width = width;
            w.height = height;

            w.game.resize(width, height);
        }

    }

    fn key_callback(key: glfw::Key, action: glfw::Action) {
        if key == glfw::Key::Unknown { return }

        inputs::set_key_state(key as i32, action != glfw::Action::Release);
    }

    fn mouse_button_callback(button: glfw::MouseButton, action: glfw::Action) {
        inputs::set_mouse_button_state(button as i32, action != glfw::Action::Release);
    }

    fn mouse_move_callback(x: f64, y: f64) {
        inputs::set_mouse_pos(x as f32, y as f32);
    }
}