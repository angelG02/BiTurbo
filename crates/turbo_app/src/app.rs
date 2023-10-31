use std::sync::{Arc, RwLock};

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

            //turbo_core::trace::tracing::trace!("Frame time: {delta_time}s");

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

pub fn run(context: Arc<RwLock<App>>) {
    let mut current_time = std::time::Instant::now();

    {
        context.write().unwrap().world.run_schedule(StartupSchedule);
    }
    // TODO: replace with event loop and recive command events
    // if the CmdEvent is Sync (meaning commands are recived from cmd thread) then prevent the loop from executing
    while context
        .read()
        .unwrap()
        .world
        .get_resource::<GState>()
        .unwrap()
        .running
    {
        let mut ctx_write_lock = context.write().unwrap();
        ctx_write_lock.world.run_schedule(CommandSchedule);

        // Calculate frame time (delta time)
        let new_time = std::time::Instant::now();
        let frame_time = (new_time - current_time).as_nanos();
        let _delta_time = frame_time as f32 * 0.000000001;
        current_time = new_time;

        //turbo_core::trace::tracing::trace!("Frame time: {delta_time}s");

        ctx_write_lock.world.run_schedule(UpdateSchedule);
        ctx_write_lock.world.run_schedule(EndFrameSchedule);
    }
    // event_loop
    //     .run(move |event, elwt| {
    //         let mut context = context.write().unwrap();
    //         let _res = executor::block_on(context.command_queue.execute());

    //         elwt.set_control_flow(winit::event_loop::ControlFlow::Poll);
    //         if !context.running {
    //             elwt.exit();
    //         }

    //         match event {
    //             winit::event::Event::WindowEvent {
    //                 event,
    //                 window_id: _,
    //             } => match event {
    //                 winit::event::WindowEvent::CloseRequested => elwt.exit(), // TODO: Drop window only
    //                 winit::event::WindowEvent::RedrawRequested => {}
    //                 _ => (),
    //             },
    //             winit::event::Event::UserEvent(event) => match event {
    //                 WindowCommandEvent::Open(props) => {
    //                     let window = winit::window::WindowBuilder::new()
    //                         .with_inner_size(winit::dpi::Size::Physical(props.size))
    //                         .with_title(props.name)
    //                         .build(elwt)
    //                         .expect("Could not create new window T-T");

    //                     context.windows.insert(window.id(), window);
    //                     info!("Created new window!")
    //                 }
    //                 WindowCommandEvent::Close(_win_id) => {}
    //                 WindowCommandEvent::Exit() => elwt.exit(),
    //             },
    //             winit::event::Event::AboutToWait => {
    //                 //window.request_redraw();
    //             }

    //             _ => (),
    //         }
    //     })
    //     .unwrap();
}
