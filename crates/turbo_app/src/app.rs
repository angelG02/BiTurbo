use crate::plugin::*;
use turbo_core::prelude::trace::*;

use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnStartup;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnEvent;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnMainUpdate;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnMainRender;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnShutdown;

static mut APP: Option<Box<App>> = None;

pub fn app() -> &'static mut App {
    unsafe { APP.as_mut().unwrap() }
}

pub fn create_v0_engine() -> &'static mut App {
    unsafe {
        APP = Some(Box::new(App::new()));
        APP.as_mut().unwrap()
    }
}

pub struct App {
    pub world: World,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        // Logging initialization
        let subscriber = FmtSubscriber::builder()
            .with_line_number(true)
            .without_time()
            .with_max_level(Level::TRACE)
            .finish();

        subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

        let mut world = World::default();

        // Initialize resoureces
        world.init_resource::<Schedules>();

        let mut schedules = world.resource_mut::<Schedules>();

        // Startup schedule (ran once on program/game startup)
        let on_startup = Schedule::new();
        schedules.insert(OnStartup, on_startup);

        // Event schedule (Window's poll_events is called here)
        let on_event = Schedule::new();
        schedules.insert(OnEvent, on_event);

        // Update schedule
        let update = Schedule::new();
        schedules.insert(OnMainUpdate, update);

        // Update schedule
        let render = Schedule::new();
        schedules.insert(OnMainRender, render);

        // Program shutdown schedule that will run when the program is closed
        let on_shutdown = Schedule::new();
        schedules.insert(OnShutdown, on_shutdown);

        Self {
            world,
            running: true,
        }
    }

    pub fn run(&mut self) {
        // ---------Timer for frame time and render time---------
        let mut current_time = std::time::Instant::now();

        self.world.run_schedule(OnStartup);

        while self.running {
            // Calculate frame time (delta time)
            let new_time = std::time::Instant::now();
            let frame_time = (new_time - current_time).as_nanos();
            let _delta_time = frame_time as f32 * 0.000000001;
            current_time = new_time;

            self.world.run_schedule(OnEvent);

            self.world.run_schedule(OnMainUpdate);

            self.world.run_schedule(OnMainRender);
        }

        self.world.run_schedule(OnShutdown);
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        let mut schedules = self.world.resource_mut::<Schedules>();

        if let Some(schedule) = schedules.get_mut(&schedule) {
            schedule.add_systems(systems);
        } else {
            let mut new_schedule = Schedule::new();
            new_schedule.add_systems(systems);
            schedules.insert(schedule, new_schedule);
        }
        self
    }

    pub fn init_resource<R: Resource + FromWorld>(&mut self) -> &mut Self {
        self.world.init_resource::<R>();
        self
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }

    // TODO: Handle this somewhere else xd

    // fn on_window_resize(event: &Event) -> bool {
    //     match event {
    //         Event::WindowResize(width, height) => warn!("Renderer Should Have a Function \"OnWindowResize()\" with width: {width}, height: {height} "),
    //         _ => {}
    //     }
    //     false
    // }

    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        plugin.build(self);
        self
    }
}
