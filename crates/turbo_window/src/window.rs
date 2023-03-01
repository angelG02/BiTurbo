use std::sync::mpsc::Receiver;
use glfw::{Action, Context, Key, Window as GLFWWindow, WindowEvent, Glfw};

use crate::event::Event;

pub struct Window<'a> {
    pub width: u32,
    pub height: u32,
    pub resized: bool,
    pub name: String,
    context: Glfw,
    window: GLFWWindow,
    events: Receiver<(f64, WindowEvent)>,
    event_callback: Option<&'a dyn Fn(&Event)>
}

impl<'a> Window<'a> {
    pub fn new(width: u32, height: u32, name: String) -> Self {

        let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();

        let (mut window, events) = 
            glfw.create_window(
                width, 
                height, 
                &name, 
                glfw::WindowMode::Windowed)
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
            event_callback: None
        }
    }

    pub fn poll_events(&mut self) {
        self.context.poll_events();
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
                    if let Some(event_callback) = self.event_callback {
                        event_callback(&event);
                    }
                }

                glfw::WindowEvent::Close => {
                    let event = Event::WindowClose;
                    if let Some(event_callback) = self.event_callback {
                        event_callback(&event);
                    }
                }

                glfw::WindowEvent::Key(key, _scancode, action, _modifiers) => {
                    match action {
                        Action::Press => {
                            let event = Event::KeyPressed(key, 0);
                            if let Some(event_callback) = self.event_callback {
                                event_callback(&event);
                            }
                        }

                        Action::Release => {
                            let event = Event::KeyReleased(key);
                            if let Some(event_callback) = self.event_callback {
                                event_callback(&event);
                            }
                        }

                        Action::Repeat => {
                            let event = Event::KeyPressed(key, 1);
                            if let Some(event_callback) = self.event_callback {
                                event_callback(&event);
                            }
                        }
                    }
                }

                glfw::WindowEvent::MouseButton(key, action, _mods) => {
                    match action {
                        Action::Press => {
                            let event = Event::MouseButtonPressed(key);
                            if let Some(event_callback) = self.event_callback {
                                event_callback(&event);
                            }
                        }

                        Action::Release => {
                            let event = Event::MouseButtonReleased(key);
                            if let Some(event_callback) = self.event_callback {
                                event_callback(&event);
                            }
                        }

                        Action::Repeat => {
                            let event = Event::MouseButtonPressed(key);
                            if let Some(event_callback) = self.event_callback {
                                event_callback(&event);
                            }
                        }
                    }
                }

                glfw::WindowEvent::Scroll(xoffset, yoffset) => {
                    let event = Event::MouseScrolled(xoffset as f32, yoffset as f32);
                    if let Some(event_callback) = self.event_callback {
                        event_callback(&event);
                    }
                }

                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let event = Event::MouseMoved(xpos as f32, ypos as f32);
                    if let Some(event_callback) = self.event_callback {
                        event_callback(&event);
                    }
                }

                _ => {}
            }
        }
    }

    pub fn get_glfw_window(&self) -> &GLFWWindow {
        &self.window
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn set_event_callback(&mut self, callback: &'a dyn Fn(&Event)) {
        self.event_callback = Some(callback);
    }
}