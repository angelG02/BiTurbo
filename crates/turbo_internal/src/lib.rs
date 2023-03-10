//#![warn(missing_docs)]
//! This module is separated into its own crate to enable simple dynamic linking for BiTurbo, and should not be used directly

/// `use bi_turbo::prelude::*;` to import common components, bundles, and plugins.
pub mod prelude;

pub mod app {
    //! Build biTurbo apps, create plugins, and read events.
    pub use turbo_app::*;
}

pub mod core {
    //! Core components and bundles for biTurbo.
    pub use turbo_core::*;
}

pub mod window {
    pub use turbo_window::*;
}

pub mod render {
    pub use turbo_render::*;
}

pub mod ecs {
    pub use ecs::*;
}
