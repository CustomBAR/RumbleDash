//! Core building blocks shared by the RumbleDash v3 GUI and its HTTP server.
//!
//! * [`config`] – the on-disk `config.json` (same shape as the v2 Node build)
//! * [`api`]    – a tolerant client for the Rumble livestream API
//! * [`cache`]  – a short lived cache so widgets and GUI share one upstream request
//! * [`model`]  – the data types exchanged over the wire

pub mod api;
pub mod cache;
pub mod config;
pub mod error;
pub mod model;

pub use api::RumbleClient;
pub use cache::DashboardCache;
pub use config::{Config, Goals};
pub use error::{Error, Result};
pub use model::{DashboardData, RumbleStats};

/// Human readable version of the port, surfaced in the sidebar.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
