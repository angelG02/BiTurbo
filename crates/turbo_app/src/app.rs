use turbo_core::trace::*;

pub struct App {
    pub state: String,

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
        }
    }

    pub fn run(&mut self) {
        self.state = "running".to_string();

        while self.state == "running" {
            
            warn!("App is running");
        }
    }
}