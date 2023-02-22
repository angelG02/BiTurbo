#![warn(missing_docs)]
//! This module is separated into its own crate to enable simple dynamic linking for BiTurbo, and should not be used directly

/// `use bi_turbo::prelude::*;` to import common components, bundles, and plugins.
pub mod prelude;

pub mod app {
    //! Build biTurbo apps, create plugins, and read events.
    pub use turbo_app::*;
}