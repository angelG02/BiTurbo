use ecs::{systems::System, *};
use glam::*;
use turbo_core::prelude::trace::*;
//use turbo_window::prelude::*;

pub struct App {
    pub state: String,
    pub world: ecs::world::World,
    pub systems: Vec<Box<dyn systems::System>>, //pub window: Window,
}

impl App {
    pub fn new() -> Self {
        // Loging initialization
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();

        subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
        Self {
            state: "new".to_string(),
            world: world::World::new(),
            //window: Window::new(1080, 720, "Mercedes s500".to_owned())
            systems: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        self.state = "running".to_string();

        // Timer for frame time and render time
        let mut current_time = std::time::Instant::now();

        //while !self.window.should_close() {
        loop {
            // Calculate frame time (delta time)
            let new_time = std::time::Instant::now();
            let frame_time = (new_time - current_time).as_nanos();
            let delta_time = frame_time as f32 / 1000000000.0;
            current_time = new_time;

            self.update_systems();

            let entity1 = self.world.add_entity();

            self.world
                .add_component(&entity1, TransformComponent::new(None, None));

            let comp = self
                .world
                .get_component::<TransformComponent>(&entity1)
                .unwrap();

            //comp.serialize_transform();
            //self.world.serialize_component(&comp);

            self.world.serialize_self();
            trace!("Frame time: {delta_time}s");

            //self.window.poll_events();
            warn!("App is running")
            // }
        }
    }

    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    pub fn update_systems(&mut self) {
        for system in &mut self.systems {
            system.update();
        }
    }
}
