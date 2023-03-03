use ecs::*;
use glam::*;
use turbo_core::prelude::trace::*;
use turbo_window::prelude::*;

pub struct App {
    pub state: String,
    pub world: ecs::world::World,
    //pub window: Window,
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

            trace!("Frame time: {delta_time}s");
            let entity1 = self.world.add_entity();
            let transform_comp: Transform = Transform::new(None, None);

            self.world
                .add_component_by_entity_id(&entity1, transform_comp);

            self.world
                .remove_component_by_entity_id::<Transform>(&entity1);

            self.world.add_component_by_entity_id(
                &entity1,
                Transform::new(Some(glam::Vec3::new(1.0, 1.0, 1.0)), None),
            );

            let new_comp: Option<&Transform> = self.world.get_component_by_entity_id(&entity1);

            println!("This is my comp trans {:?}", new_comp);

            let entity2 = self.world.add_entity();
            let entity3 = self.world.add_entity();

            self.world.add_component_by_entity_id(
                &entity2,
                Transform::new(Some(glam::Vec3::new(2.0, 2.0, 2.0)), None),
            );
            self.world.add_component_by_entity_id(
                &entity3,
                Transform::new(Some(glam::Vec3::new(3.0, 3.0, 3.0)), None),
            );

            let comps = self
                .world
                .get_all_components_of_type::<Transform>()
                .unwrap();

            println!("Here are my components {:?}", comps);

            //self.window.poll_events();
            self.world.remove_entity(entity1);
            self.world.remove_entity(entity2);
            self.world.remove_entity(entity3);

            warn!("App is running")
            // }
        }
    }
}
