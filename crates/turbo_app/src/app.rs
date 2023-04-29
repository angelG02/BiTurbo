use crate::plugin::*;
use turbo_core::event::Event;
use turbo_core::layer::{Layer, LayerStack};
use turbo_core::prelude::*;

use bevy_ecs::{prelude::*, schedule::ScheduleLabel};

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnMainPreUpdate;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnMainUpdate;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnMainPostUpdate;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnStartup;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnEvent;

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
            .with_max_level(Level::TRACE)
            .finish();

        subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

        let mut world = World::default();

        // Initialize resourece TODO: Maybe move all this to an init app system?
        world.init_resource::<Schedules>();
        world.init_resource::<LayerStack>();

        let mut schedules = world.resource_mut::<Schedules>();

        // Pre update schedule
        let pre_update = Schedule::new();
        schedules.insert(OnMainPreUpdate, pre_update);

        // Post update schedule
        let mut post_update = Schedule::new();
        post_update.add_systems((apply_system_buffers, World::clear_trackers).chain());
        schedules.insert(OnMainPostUpdate, post_update);

        // On Event schedule
        let mut on_event = Schedule::new();
        on_event.add_system(App::on_event);
        schedules.insert(OnEvent, on_event);

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
            self.world.run_schedule(OnMainPreUpdate);

            // Calculate frame time (delta time)
            let new_time = std::time::Instant::now();
            let frame_time = (new_time - current_time).as_nanos();
            let _delta_time = frame_time as f32 * 0.000000001;
            current_time = new_time;

            //trace!("Frame time: {delta_time}s");

            self.world.run_schedule(OnEvent);

            self.world.run_schedule(OnMainUpdate);

            self.world.run_schedule(OnMainPostUpdate);
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

    fn on_event(world: &mut World) {
        if let Some(event) = world.get_resource::<Event>() {
            let mut _dispatcher = EventDispatcher::new(&event);
            let layer_stack = world.get_resource::<LayerStack>().unwrap();

            match event {
                // Event::WindowResize(_, _) => dispatcher.dispatch(&App::on_window_resize),
                _ => {
                    for layer in layer_stack.into_iter().rev() {
                        layer.on_event(&event)
                    }
                }
            }
            world.remove_resource::<Event>();
        }
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

    pub fn push_layer(&mut self, layer_name: &str, layer: Box<dyn Layer>) -> &mut Self {
        let mut layer_stack = self.world.get_resource_mut::<LayerStack>().unwrap();
        layer_stack.push_layer(layer_name, layer);
        self
    }

    pub fn pop_layer(&mut self, layer_name: &str) -> &mut Self {
        let mut layer_stack = self.world.get_resource_mut::<LayerStack>().unwrap();
        layer_stack.pop_layer(layer_name);
        self
    }

    /// Overlays will always be pushed to the back of the Layer Stack (Will always be on top of the layers)
    pub fn push_overlay(&mut self, overlay_name: &str, overlay: Box<dyn Layer>) -> &mut Self {
        let mut layer_stack = self.world.get_resource_mut::<LayerStack>().unwrap();
        layer_stack.push_overlay(overlay_name, overlay);
        self
    }

    pub fn pop_overlay(&mut self, overlay_name: &str) -> &mut Self {
        let mut layer_stack = self.world.get_resource_mut::<LayerStack>().unwrap();
        layer_stack.pop_overlay(overlay_name);
        self
    }
}
