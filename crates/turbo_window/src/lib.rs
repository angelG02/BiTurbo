pub mod window;
pub use window::*;

pub mod event;
pub use event::*;

pub mod prelude {
    pub use crate::window::Window;
    pub use crate::event::*;
}