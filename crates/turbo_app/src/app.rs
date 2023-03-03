use ecs::*;
use turbo_core::prelude::{trace::*, Layer, LayerStack};
use turbo_window::prelude::{Event, EventDispatcher, Window};

pub struct App<'a> {
    pub world: world::World,
    pub window: Window<'a>,
    event_call: Option<&'a dyn Fn(&Event)>,
    layer_stack: LayerStack,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        // Loging initialization
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();

        subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

        Self {
            world: world::World::new(),
            window: Window::new(1080, 720, "Mercedes s500".to_owned()),
            event_call: None,
            layer_stack: LayerStack::new(),
        }
    }

    pub fn run(&mut self) {
        // ---------Setup event callback for event dispatching and layer propagation---------
        self.event_call = Some(&|event: &Event| {
            let mut dispatcher = EventDispatcher::new(event);

            match *event {
                Event::WindowResize(_, _) => dispatcher.dispatch(&App::on_window_resize),
                _ => info!("{:?}", event),
            }
        });

        if let Some(ec) = self.event_call {
            self.window.set_event_callback(ec);
        }

        // ---------Timer for frame time and render time---------
        let mut current_time = std::time::Instant::now();

        while !self.window.should_close() {
            // Calculate frame time (delta time)
            let new_time = std::time::Instant::now();
            let frame_time = (new_time - current_time).as_nanos();
            let _delta_time = frame_time as f32 * 0.000000001;
            current_time = new_time;

            //trace!("Frame time: {delta_time}s");

            self.window.poll_events();
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
