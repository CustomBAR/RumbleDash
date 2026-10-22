//! Headless entry point: runs only the widget server.
//!
//! Useful when the widgets are hosted on a different machine than the GUI, or
//! when the dashboard should run on a small always-on box.

use rumbledash_core::config::default_config_path;
use rumbledash_server::{AppState, ServerHandle};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config_path = default_config_path();
    let port = port_from_env();

    let state = AppState::new(config_path.clone(), port);
    state.load_config().await?;

    let handle: ServerHandle = rumbledash_server::state::spawn(state, port)?;

    println!(
        "RumbleDash v3 is running on http://{}:{}",
        handle.lan_ip,
        handle.addr.port()
    );
    println!("  config file : {}", config_path.display());
    println!(
        "  followers   : http://{}:{}/followers",
        handle.lan_ip,
        handle.addr.port()
    );
    println!(
        "  subscribers : http://{}:{}/subscribers",
        handle.lan_ip,
        handle.addr.port()
    );

    tokio::signal::ctrl_c().await?;
    println!("\nShutting down...");
    handle.shutdown();
    Ok(())
}

/// Reads the listening port from the environment, defaulting to `9000`.
fn port_from_env() -> u16 {
    std::env::var("RUMBLE_DASH_PORT")
        .or_else(|_| std::env::var("PORT"))
        .ok()
        .and_then(|value| value.trim().parse::<u16>().ok())
        .unwrap_or(9000)
}
