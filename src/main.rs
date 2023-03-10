use bi_turbo::{prelude::*, turbo_ecs::systems};

fn main() {
    let mut app = App::new();
    let test_system = systems::MovementTestSystem::new();
    app.add_system(test_system);
    app.run();
}
