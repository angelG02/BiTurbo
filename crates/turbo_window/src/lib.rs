pub mod window;

pub mod prelude {
    pub use crate::window::*;
}

use bevy_ecs::prelude::*;
use turbo_app::prelude::*;
use turbo_core::event::Event;
use window::Window;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        let window = Window::new(1080, 720, "BiTurbo x Chronlicle".into());

        app.world.insert_non_send_resource(window);

        let mut schedules = app.world.resource_mut::<Schedules>();

        if let Some(schedule) = schedules.get_mut(&OnEvent) {
            schedule.add_system(poll_events);
        }

        if let Some(schedule) = schedules.get_mut(&OnEvent) {
            schedule.add_system(close_window);
        }
    }
}

fn poll_events(world: &mut World) {
    let mut window = world.get_non_send_resource_mut::<Window>().unwrap();

    let events = window.poll_events();

    for event in events {
        world.insert_resource::<Event>(event);
    }
}

fn close_window(world: &mut World) {
    let window = world.get_non_send_resource::<Window>().unwrap();
    app().running = !window.should_close();
}
