use axum::extract::Path;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use rumbledash_core::config::Config;

use crate::assets;
use crate::state::AppState;

/// Builds the router with every route the v2 server exposed.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/followers", get(followers))
        .route("/subscribers", get(subscribers))
        .route("/form", get(legacy))
        .route("/changelog", get(legacy))
        .route("/contributors", get(legacy))
        .route("/styles/{name}", get(stylesheet))
        .route("/scripts/{name}", get(script))
        .route("/api/dashboard", get(dashboard))
        .route("/api/config", get(get_config))
        .route("/api/update-config", post(update_config))
        .route("/api/clear-api-url", post(clear_api_url))
        .route("/api/server-ip", get(server_ip))
        .route("/api/health", get(health))
        .with_state(state)
}

/// `GET /api/server-ip` – the address OBS should connect to.
async fn server_ip(state: axum::extract::State<AppState>) -> Json<ServerIp> {
    Json(ServerIp {
        ip: crate::net::advertised_ip().to_string(),
        port: state.port,
    })
}

/// Payload of `/api/server-ip`.
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerIp {
    /// LAN address of this machine.
    pub ip: String,
    /// TCP port the server listens on.
    pub port: u16,
}

/// `GET /api/health` – a tiny readiness probe.
async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// Payload of `/api/health`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Health {
    /// Always `ok` when the server is reachable.
    pub status: &'static str,
    /// Version of the running binary.
    pub version: &'static str,
}

/// `GET /api/dashboard` – counters plus goals.
async fn dashboard(state: axum::extract::State<AppState>) -> Response {
    match state.dashboard().await {
        Ok(payload) => Json(payload).into_response(),
        Err(error) => {
            let status = match error {
                rumbledash_core::Error::Config(_) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, error.to_string()).into_response()
        }
    }
}

/// `GET /api/config` – the current configuration.
async fn get_config(state: axum::extract::State<AppState>) -> Json<Config> {
    Json(state.config.read().await.clone())
}

/// `POST /api/update-config` – persists a new configuration.
async fn update_config(
    state: axum::extract::State<AppState>,
    Json(config): Json<Config>,
) -> Json<UpdateResult> {
    {
        let mut guard = state.config.write().await;
        *guard = config;
    }
    state.cache.invalidate();

    match state.persist_config().await {
        Ok(()) => Json(UpdateResult {
            success: true,
            message: "Configuration updated successfully!".into(),
        }),
        Err(error) => {
            tracing::error!(%error, "could not persist configuration");
            Json(UpdateResult {
                success: false,
                message: format!("Error updating configuration: {error}"),
            })
        }
    }
}

/// `POST /api/clear-api-url` – wipes the stored Rumble API URL.
async fn clear_api_url(state: axum::extract::State<AppState>) -> Json<UpdateResult> {
    {
        let mut guard = state.config.write().await;
        guard.api_url.clear();
    }
    state.cache.invalidate();

    match state.persist_config().await {
        Ok(()) => Json(UpdateResult {
            success: true,
            message: "API URL cleared successfully!".into(),
        }),
        Err(error) => {
            tracing::error!(%error, "could not persist configuration");
            Json(UpdateResult {
                success: false,
                message: format!("Error clearing configuration: {error}"),
            })
        }
    }
}

/// Payload returned by the mutating endpoints.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResult {
    /// Whether the change was persisted.
    pub success: bool,
    /// A human readable status message.
    pub message: String,
}

/// `GET /` – the browser status page.
async fn index() -> Response {
    html(assets::INDEX_HTML)
}

/// `GET /followers` – the followers overlay.
async fn followers() -> Response {
    html(assets::FOLLOWERS_HTML)
}

/// `GET /subscribers` – the subscribers overlay.
async fn subscribers() -> Response {
    html(assets::SUBSCRIBERS_HTML)
}

/// `GET /form`, `/changelog`, `/contributors` – the v2 browser pages now live in
/// the desktop application, so the server only points the visitor there.
async fn legacy() -> Response {
    html(assets::LEGACY_HTML)
}

/// `GET /styles/{name}` – a bundled stylesheet.
async fn stylesheet(Path(name): Path<String>) -> Response {
    match assets::stylesheet(name.trim_end_matches(".css")) {
        Some(css) => ([(header::CONTENT_TYPE, "text/css; charset=utf-8")], css).into_response(),
        None => (StatusCode::NOT_FOUND, "stylesheet not found").into_response(),
    }
}

/// `GET /scripts/{name}` – a bundled script.
async fn script(Path(name): Path<String>) -> Response {
    match assets::script(name.trim_end_matches(".js")) {
        Some(js) => (
            [(
                header::CONTENT_TYPE,
                "application/javascript; charset=utf-8",
            )],
            js,
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND, "script not found").into_response(),
    }
}

/// Helper that renders a document with the correct content type.
fn html(body: &'static str) -> Response {
    ([(header::CONTENT_TYPE, "text/html; charset=utf-8")], body).into_response()
}
