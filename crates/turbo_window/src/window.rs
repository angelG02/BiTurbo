use glfw::{Action, Context, Glfw, Key, Window as GLFWWindow, WindowEvent};
use std::sync::mpsc::Receiver;
use turbo_app::prelude::*;

use turbo_core::event::Event;

use bevy_ecs::prelude::*;

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub resized: bool,
    pub name: String,
    context: Glfw,
    window: GLFWWindow,
    events: Receiver<(f64, WindowEvent)>,
}

impl Window {
    /// Creates a new Window with the specified dimensions and name.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the window.
    /// * `height` - The height of the window.
    /// * `name` - The name of the window.
    ///
    /// # Returns
    ///
    /// A new [`Window`] instance.
    pub fn new(width: u32, height: u32, name: String) -> Self {
        let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, &name, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_all_polling(true);
        window.make_current();

        Window {
            width,
            height,
            resized: false,
            name,
            context: glfw,
            window,
            events,
        }
    }

    /// Polls for pending events happening on the Window (Key, Mouse, Resize, Close...)
    /// See [`Event`]
    ///
    /// # Returns
    ///
    /// A [`Vec<Event>`] representing the input events that
    /// occurred since the last call to this method.
    pub fn poll_events(&mut self) -> Vec<Event> {
        self.context.poll_events();

        let mut turbo_events: Vec<Event> = Vec::new();

        for (_time, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                }

                glfw::WindowEvent::Size(width, height) => {
                    self.width = width as u32;
                    self.height = height as u32;
                    self.resized = true;
                    let event = Event::WindowResize(width, height);
                    turbo_events.push(event);
                }

                glfw::WindowEvent::Close => {
                    let event = Event::WindowClose;
                    turbo_events.push(event);
                }

                glfw::WindowEvent::Key(key, _scancode, action, _modifiers) => match action {
                    Action::Press => {
                        let event = Event::KeyPressed(key, 0);
                        turbo_events.push(event);
                    }

                    Action::Release => {
                        let event = Event::KeyReleased(key);
                        turbo_events.push(event);
                    }

                    Action::Repeat => {
                        let event = Event::KeyPressed(key, 1);
                        turbo_events.push(event);
                    }
                },

                glfw::WindowEvent::MouseButton(key, action, _mods) => match action {
                    Action::Press => {
                        let event = Event::MouseButtonPressed(key);
                        turbo_events.push(event);
                    }

                    Action::Release => {
                        let event = Event::MouseButtonReleased(key);
                        turbo_events.push(event);
                    }

                    Action::Repeat => {
                        let event = Event::MouseButtonPressed(key);
                        turbo_events.push(event);
                    }
                },

                glfw::WindowEvent::Scroll(xoffset, yoffset) => {
                    let event = Event::MouseScrolled(xoffset as f32, yoffset as f32);
                    turbo_events.push(event);
                }

                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let event = Event::MouseMoved(xpos as f32, ypos as f32);
                    turbo_events.push(event);
                }

                glfw::WindowEvent::FileDrop(path) => {
                    let event = Event::FileDropped(path);
                    turbo_events.push(event);
                }

                _ => {}
            }
        }
        turbo_events
    }

    /// Returns a reference to the underlying GLFW window object.
    pub fn get_glfw_window(&self) -> &GLFWWindow {
        &self.window
    }

    /// Returns whether the user has requested that the window be closed.
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
}

unsafe impl Send for Window {}

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        let window = Window::new(1080, 720, "BiTurbo x Chronlicle".into());

        app.world.insert_non_send_resource(window);

        let mut schedules = app.world.resource_mut::<Schedules>();

        if let Some(schedule) = schedules.get_mut(&OnEvent) {
            schedule.add_system(poll_events);
        }

        if let Some(schedule) = schedules.get_mut(&OnEvent) {
            schedule.add_system(close_window);
        }
    }
}

fn poll_events(world: &mut World) {
    let mut window = world.get_non_send_resource_mut::<Window>().unwrap();

    let events = window.poll_events();

    for event in events {
        world.insert_resource::<Event>(event);
    }
}

fn close_window(world: &mut World) {
    let window = world.get_non_send_resource::<Window>().unwrap();
    app().running = !window.should_close();
}
