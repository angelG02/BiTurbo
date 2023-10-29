//#![warn(missing_docs)]
//! This module is separated into its own crate to enable simple dynamic linking for BiTurbo, and should not be used directly

pub mod turbo_app {
    //! Build biTurbo apps, create plugins, and read events.
    pub use turbo_app::app;
    pub use turbo_app::cmd_queue;
    pub use turbo_app::plugin;
}

pub mod turbo_core {
    //! Core components and bundles for biTurbo.
    pub use turbo_core;
}

pub mod turbo_window {
    pub use turbo_window;
}

/// `use bi_turbo::prelude::*;` to import common components, bundles, and plugins.
pub mod prelude;
