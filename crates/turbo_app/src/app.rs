use crate::plugin::*;
use turbo_core::trace::{
    tracing::{subscriber, Level},
    tracing_subscriber::FmtSubscriber,
};

use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UpdateSchedule;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StartupSchedule;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommandSchedule;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EndFrameSchedule;

#[derive(Resource)]
pub struct GState {
    pub running: bool,
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
            .with_thread_ids(true)
            .without_time()
            .with_max_level(Level::TRACE)
            .finish();

        subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

        let mut world = World::default();
        let global_state = GState { running: true };

        // Initialize resoureces
        world.init_resource::<Schedules>();
        world.insert_resource(global_state);

        let mut schedules = world.resource_mut::<Schedules>();

        // startup
        let startup = Schedule::new();
        schedules.insert(StartupSchedule, startup);

        // command
        let cmd = Schedule::new();
        schedules.insert(CommandSchedule, cmd);

        // update
        let update = Schedule::new();
        schedules.insert(UpdateSchedule, update);

        // end frame schedule (used for claering state, commands, etc)
        let mut end = Schedule::new();
        end.add_systems(World::clear_trackers);
        schedules.insert(EndFrameSchedule, end);

        Self {
            world,
            running: true,
        }
    }

    pub fn run(&mut self) {
        // ---------Timer for frame time and render time---------
        let mut current_time = std::time::Instant::now();

        self.world.run_schedule(StartupSchedule);

        while self.running {
            self.world.run_schedule(CommandSchedule);

            // Calculate frame time (delta time)
            let new_time = std::time::Instant::now();
            let frame_time = (new_time - current_time).as_nanos();
            let delta_time = frame_time as f32 * 0.000000001;
            current_time = new_time;

            turbo_core::trace::tracing::trace!("Frame time: {delta_time}s");

            self.world.run_schedule(UpdateSchedule);
            self.world.run_schedule(EndFrameSchedule);
        }
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

    // TODO: Serialize pugins
    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        plugin.build(self);
        self
    }
}
