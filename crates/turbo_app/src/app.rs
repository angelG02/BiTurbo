use glam::*;

use turbo_core::prelude::{trace::*, Layer, LayerStack};
use turbo_ecs::*;
use turbo_window::prelude::{Event, EventDispatcher, Window};

pub struct App {
    pub world: world::World,
    pub window: Window,
    pub systems: Vec<Box<dyn systems::System>>,
    layer_stack: LayerStack,
}

impl App {
    pub fn new() -> Self {
        // Logging initialization
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();

        subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
        Self {
            world: world::World::new(),
            //window: Window::new(1080, 720, "Mercedes s500".to_owned())
            systems: Vec::new(),
            window: Window::new(1080, 720, "Mercedes s500".to_owned()),
            layer_stack: LayerStack::new(),
        }
    }

    pub fn on_event(&mut self, events: Vec<Event>) {
        for event in events {
            for layer in &self.layer_stack {
                let mut dispatcher = EventDispatcher::new(&event);

                match event {
                    Event::WindowResize(_, _) => dispatcher.dispatch(&App::on_window_resize),
                    _ => layer.on_event(&event),
                }
            }
        }
    }

    pub fn run(&mut self) {
        // ---------Timer for frame time and render time---------
        let mut current_time = std::time::Instant::now();

        while !self.window.should_close() {
            // Calculate frame time (delta time)
            let new_time = std::time::Instant::now();
            let frame_time = (new_time - current_time).as_nanos();
            let _delta_time = frame_time as f32 * 0.000000001;
            current_time = new_time;
            // if comp.position.x >= 4.0 {
            //     self.world.desirialize_self();
            // }

            //trace!("Frame time: {delta_time}s");

            let events = self.window.poll_events();
            self.on_event(events);
        }
    }

    pub fn add_system<T: systems::System + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    pub fn update_systems(&mut self) {
        for system in &mut self.systems {
            system.update();
        }
    }

    fn on_window_resize(event: &Event) -> bool {
        match event {
                Event::WindowResize(width, height) => warn!("Renderer Should Have a Function \"OnWindowResize()\" with width: {width}, height: {height} "),
                _ => {}
            }
        false
    }

    pub fn push_layer(&mut self, layer_name: &str, layer: Box<dyn Layer>) {
        self.layer_stack.push_layer(layer_name, layer);
    }

    pub fn pop_layer(&mut self, layer_name: &str) {
        self.layer_stack.pop_layer(layer_name);
    }

    /// Overlays will always be pushed to the back of the Layer Stack (Will always be on top of the layers)
    pub fn push_overlay(&mut self, overlay_name: &str, overlay: Box<dyn Layer>) {
        self.layer_stack.push_overlay(overlay_name, overlay);
    }

    pub fn pop_overlay(&mut self, overlay_name: &str) {
        self.layer_stack.pop_overlay(overlay_name);
    }
}
