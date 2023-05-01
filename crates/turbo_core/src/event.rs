use std::path::PathBuf;

use bevy_ecs::prelude::*;

pub enum EventCategory {
    None = 0,
    Application = (1 << 0),
    Input = (1 << 1),
    Keyboard = (1 << 2),
    Mouse = (1 << 3),
    MouseButton = (1 << 4),
}

#[derive(Resource, PartialEq, Debug)]
#[repr(i32)]
pub enum Event {
    None = 0,
    WindowClose,
    WindowResize(i32, i32),
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    KeyPressed(glfw::Key, i32),
    KeyReleased(glfw::Key),
    KeyTyped(glfw::Key),
    MouseButtonPressed(glfw::MouseButton),
    MouseButtonReleased(glfw::MouseButton),
    MouseMoved(f32, f32),
    MouseScrolled(f32, f32),
    FileDropped(Vec<PathBuf>),
    Handled,
}

impl Event {
    pub fn get_category_flags(&self) -> u8 {
        match *self {
            Self::WindowClose => EventCategory::Application as u8,
            Self::WindowResize(_, _) => EventCategory::Application as u8,
            Self::WindowFocus => EventCategory::Application as u8,
            Self::WindowLostFocus => EventCategory::Application as u8,
            Self::KeyPressed(_, _) => EventCategory::Input as u8 | EventCategory::Keyboard as u8,
            Self::KeyReleased(_) => EventCategory::Input as u8 | EventCategory::Keyboard as u8,
            Self::KeyTyped(_) => EventCategory::Input as u8 | EventCategory::Keyboard as u8,
            Self::MouseButtonPressed(_) => {
                EventCategory::Input as u8
                    | EventCategory::Mouse as u8
                    | EventCategory::MouseButton as u8
            }
            Self::MouseButtonReleased(_) => {
                EventCategory::Input as u8
                    | EventCategory::Mouse as u8
                    | EventCategory::MouseButton as u8
            }
            Self::MouseMoved(_, _) => EventCategory::Input as u8 | EventCategory::Mouse as u8,
            Self::MouseScrolled(_, _) => EventCategory::Input as u8 | EventCategory::Mouse as u8,

            _ => EventCategory::None as u8,
        }
    }

    pub fn is_in_category(&self, category: EventCategory) -> bool {
        (self.get_category_flags() & category as u8) != 0
    }
}

pub struct EventDispatcher<'a> {
    event: &'a Event,
}

impl<'a> EventDispatcher<'a> {
    pub fn new(event: &'a Event) -> Self {
        EventDispatcher { event: &event }
    }

    pub fn dispatch(&mut self, callback: &dyn Fn(&Event) -> bool) {
        if *self.event != Event::Handled {
            callback(&self.event);
            self.event = &Event::Handled;
        }
    }
}
