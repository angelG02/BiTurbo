pub use turbo_window::Event;

pub trait Layer {
    fn on_attach();
    fn on_detach();
    fn on_update(delta_time: f32);
    fn on_event(event: &Event);
}

pub struct Kur {
    pub debug_name: String
}

impl Layer for Kur {
    fn on_attach() {
        todo!()
    }

    fn on_detach() {
        todo!()
    }

    fn on_event(_event: &Event) {
        todo!()
    }

    fn on_update(_delta_time: f32) {
        todo!()
    }
}