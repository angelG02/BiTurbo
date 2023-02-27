use std::sync::mpsc::Receiver;

use glfw::{Action, Context, Key, Window as GLFWWindow, WindowEvent, Glfw};

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub resized: bool,
    pub name: String,
    context: Glfw,
    window: GLFWWindow,
    events: Receiver<(f64, WindowEvent)>
}

impl Window {
    pub fn new(width: u32, height: u32, name: String) -> Self {

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw.create_window(width, height, &name, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        Window {
            width,
            height,
            resized: false,
            name,
            context: glfw,
            window,
            events
        }
    }

    pub fn poll_events(&mut self) {
        self.context.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                }
                _ => {}
            }
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
}