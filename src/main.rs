use bi_turbo::prelude::*;

fn main() {
    let mut app = App::new();
    let debug_event_layer = DebugLayer;
    app.push_layer("Debug", Box::new(debug_event_layer));
    app.run();
}

struct DebugLayer;

impl Layer for DebugLayer {
    fn on_attach(&self) {}
    fn on_detach(&self) {}
    fn on_event(&self, event: &Event) {
        println!("{:?}", event);
    }
    fn on_tick(&self, _delta_time: f32) {}
}
