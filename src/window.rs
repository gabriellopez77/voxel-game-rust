use glfw::WindowEvent;
use crate::inputs::Inputs;
use crate::game::Game;
use crate::render::core::VulkanApp;


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
        let mut inputs = Inputs::new();

        let mut imgui = imgui::Context::create();
        let mut first_time = true;
        let mut last_frame = 0.0f32;

        while !self.window.should_close() {
            // update keyboard and mouse inupts
            inputs.new_frame();

            // poll window events
            self.glfw_instance.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                inputs.roll_event(&event);
                let imgui_io = imgui.io_mut();

                match event {
                    WindowEvent::FramebufferSize(width, height) => {
                        vulkan_app.resize(width, height, &mut self.glfw_instance, &self.window);
                        game.resize(width as f32, height as f32);
                        self.width = width as f32;
                        self.height = height as f32;
                        imgui_io.display_size = [self.width, self.height];
                    }
                    WindowEvent::MouseButton(button, action, _) => {
                        let imgui_button = match button {
                            glfw::MouseButton::Button1 => imgui::MouseButton::Left,
                            glfw::MouseButton::Button2 => imgui::MouseButton::Right,
                            _ => imgui::MouseButton::Middle,
                        };

                        imgui_io.add_mouse_button_event(imgui_button, action != glfw::Action::Release);
                    }
                    WindowEvent::CursorPos(x, y) => {
                        imgui_io.mouse_pos = [x as f32, y as f32];
                    }
                    _ => {}
                }
            }

            // calculate delta time
            let time = self.glfw_instance.get_time() as f32;
            let dt = time - last_frame;
            last_frame = time;


            vulkan_app.begin_frame(&self.window);

            if first_time {
                game.start(&mut vulkan_app, &mut imgui);
                game.resize(self.width, self.height);
                first_time = false;
            }

            game.update(dt, time, self, &mut inputs);
            game.render(dt, &mut imgui);

            vulkan_app.end_frame();
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
}
