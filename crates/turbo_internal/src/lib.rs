//#![warn(missing_docs)]
//! This module is separated into its own crate to enable simple dynamic linking for BiTurbo, and should not be used directly

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

pub mod ecs {
    pub use turbo_ecs::*;
}

pub mod render {
    pub use turbo_render::*;
}

pub mod assets {
    pub use assets_manager::*;
}

/// `use bi_turbo::prelude::*;` to import common components, bundles, and plugins.
pub mod prelude;
