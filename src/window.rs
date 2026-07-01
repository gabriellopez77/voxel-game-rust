use glfw::WindowEvent;
use crate::inputs;
use crate::game::Game;
use crate::render::VulkanApp;


pub struct Window {
    glfw_instance: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>,

    width: f32,
    height: f32,
}

impl Window {
    pub fn init(width: u32, height: u32, title: &str) -> Self {
        let mut glfw_instance = glfw::init(|error, description| glfw::fail_on_errors(error, description)).unwrap();
        glfw_instance.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let (mut window, events) =
            glfw_instance.create_window(width, height, title, glfw::WindowMode::Windowed).unwrap();

        window.set_size_limits(Some(1050), Some(650), None, None);


        // set pollings
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);

        Self {
            glfw_instance,
            window,
            events,

            width: width as f32,
            height: height as f32,
        }
    }

    pub fn run(&mut self) {
        let mut vulkan_app = VulkanApp::new();
        vulkan_app.start(&self.window);

        let mut game = Game::new(&mut vulkan_app);


        let mut first_time = true;
        let mut last_frame = 0.0f32;

        while !self.window.should_close() {
            // update keyboard and mouse inupts
            inputs::new_frame();

            // poll window events
            self.glfw_instance.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                Self::roll_events(&mut self.glfw_instance, &self.window, event, &mut vulkan_app, &mut game);
            }

            // calculate delta time
            let time = self.glfw_instance.get_time() as f32;
            let dt = time - last_frame;
            last_frame = time;


            if vulkan_app.begin_frame(&self.window) {
                if first_time {
                    game.start(&mut vulkan_app);
                    game.resize(self.width, self.height);
                    first_time = false;
                }

                game.update(dt, self);
                game.render();

                vulkan_app.end_frame();
            }
        }

        game.cleanup(&mut vulkan_app);
        vulkan_app.cleanup();
    }

    pub fn close(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn set_cursor(&mut self, cursor: glfw::CursorMode) {
        self.window.set_cursor_mode(cursor);
    }

    fn roll_events(glfw_instance: &mut glfw::Glfw, glfw_window: &glfw::PWindow,
                   event: WindowEvent, app: &mut VulkanApp, game: &mut Game) {
        inputs::roll_event(&event);

        match event {
            WindowEvent::FramebufferSize(width, height) => {
                app.resize(width, height, glfw_instance, glfw_window);

                game.resize(width as f32, height as f32);
            }
            _ => {}
        }
    }
}
