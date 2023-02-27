use turbo_core::prelude::trace::*;
use turbo_window::prelude::*;
use ecs::*;

pub struct App {
    pub state: String,
    pub world: ecs::world::World,
    pub window: Window,
}

impl App {
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

    pub fn run(&mut self) {
        self.state = "running".to_string();

        while !self.window.should_close() {
            // self.world.entities_components.insert(0, Position{x: 0.0, y: 0.0});
            let entity1 = self.world.add_entity();

            let pos = Position {x: 0.0, y: 0.0};
            self.world.add_component_by_entity_id(entity1,pos);
            warn!("App is running");

            self.window.poll_events();
        }
    }
}