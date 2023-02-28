use turbo_core::prelude::trace::*;
use turbo_window::prelude::{Window, Event, EventDispatcher};
use ecs::*;

pub struct App<'a> {
    pub state: String,
    pub world: world::World,
    pub window: Window<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        // Loging initialization
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();

        subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        Self {
            state: "new".to_string(),
            world: world::World::new(),
            window: Window::new(1080, 720, "Mercedes s500".to_owned())
        }
    }

    pub fn on_event(event: &Event) {
        let mut dispatcher = EventDispatcher::new(event);

        match *event {
            Event::WindowResize(_, _) => dispatcher.dispatch(&App::on_window_resize),
            _ => info!("{:?}", event),
        }
    }

    pub fn run(&mut self) {
        self.state = "running".to_string();
        self.window.set_event_callback(&Self::on_event);

        // Timer for frame time and render time
        let mut current_time = std::time::Instant::now();

        while !self.window.should_close() {

            // Calculate frame time (delta time)
            let new_time = std::time::Instant::now();
            let frame_time = (new_time - current_time).as_nanos();
            let _delta_time = frame_time as f32 / 1000000000.0;
            current_time = new_time;

            //trace!("Frame time: {delta_time}s");

            self.window.poll_events();
            let pos_comp: &Position = self.world.get_component_by_entity_id(entity1).unwrap();
            println!("This is my successful comp: {:?}", pos_comp);

            warn!("App is running {:?}", pos_comp);
        }
    }

    fn on_window_resize(event: &Event) -> bool {
        match event {
            Event::WindowResize(width, height) => warn!("Renderer Should Have a Function \"OnWindowResize()\" with width: {width}, height: {height} "),
            _ => {}
        }        
        false
    }
    
}