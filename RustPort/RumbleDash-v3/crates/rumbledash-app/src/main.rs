//! RumbleDash v3 – the native desktop dashboard.
//!
//! The application shows the configuration panel in an egui window and runs a
//! small HTTP server in the background that serves the OBS widgets.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod assets;
mod pages;
mod sys;
mod theme;
mod widgets;

use rumbledash_core::config::default_config_path;
use rumbledash_server::AppState;

/// The default port, matching the v2 Node build.
const DEFAULT_PORT: u16 = 9000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config_path = default_config_path();
    let port = port_from_env();

    let server = AppState::new(config_path.clone(), port);
    if let Err(error) = server.load_config_blocking() {
        tracing::warn!(%error, "could not read the configuration, starting with the defaults");
    }

    let handle = rumbledash_server::state::spawn(server.clone(), port)?;
    tracing::info!(
        "widget server listening on http://{}:{}",
        handle.lan_ip,
        handle.addr.port()
    );

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("RumbleDash v3")
            .with_inner_size([1280.0, 820.0])
            .with_min_inner_size([960.0, 620.0]),
        ..Default::default()
    };

    eframe::run_native(
        "RumbleDash",
        options,
        Box::new(move |creation| Ok(Box::new(app::RumbleDashApp::new(creation, server, handle)))),
    )?;

    Ok(())
}

/// Reads the listening port from the environment, defaulting to `9000`.
fn port_from_env() -> u16 {
    std::env::var("RUMBLE_DASH_PORT")
        .or_else(|_| std::env::var("PORT"))
        .ok()
        .and_then(|value| value.trim().parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT)
}
