

use turbo_core::prelude::trace::*;
use ecs::*;

pub struct App {
    pub state: String,
    pub world: ecs::world::World
}

impl App {
    pub fn new() -> Self {

        // a builder for `FmtSubscriber`.
        let subscriber = FmtSubscriber::builder()
            // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
            // will be written to stdout.
            .with_max_level(Level::TRACE)
            // completes the builder.
            .finish();

        subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        Self {
            state: "new".to_string(),
            world: world::World::new()
        }
    }

    pub fn run(&mut self) {
        self.state = "running".to_string();

        while self.state == "running" {
            // self.world.entities_components.insert(0, Position{x: 0.0, y: 0.0});
            let entity1 = self.world.add_entity();

            let pos = Position {x: 0.0, y: 0.0};
            self.world.add_component_by_entity_id(entity1,pos);
            warn!("App is running");
        }
    }
}