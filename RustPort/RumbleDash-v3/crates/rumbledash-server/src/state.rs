use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use rumbledash_core::cache::DashboardCache;
use rumbledash_core::config::Config;
use rumbledash_core::RumbleClient;
use tokio::sync::RwLock;

use crate::{net, routes};

/// Everything the HTTP handlers need in order to answer a request.
#[derive(Clone)]
pub struct AppState {
    /// The live configuration, shared with the GUI.
    pub config: Arc<RwLock<Config>>,
    /// Short lived cache for the upstream Rumble API response.
    pub cache: Arc<DashboardCache>,
    /// HTTP client used to talk to the Rumble API.
    pub client: Arc<RumbleClient>,
    /// Where `config.json` lives on disk.
    pub config_path: Arc<std::path::PathBuf>,
    /// Port the server listens on.
    pub port: u16,
}

impl AppState {
    /// Creates a new state container.
    pub fn new(config_path: impl Into<std::path::PathBuf>, port: u16) -> Self {
        Self {
            config: Arc::new(RwLock::new(Config::default())),
            cache: Arc::new(DashboardCache::default()),
            client: Arc::new(RumbleClient::new()),
            config_path: Arc::new(config_path.into()),
            port,
        }
    }

    /// Loads `config.json` into the shared configuration.
    pub async fn load_config(&self) -> rumbledash_core::Result<Config> {
        let config = Config::load(self.config_path.as_ref())?;
        *self.config.write().await = config.clone();
        Ok(config)
    }

    /// Loads `config.json` without needing an async context.
    pub fn load_config_blocking(&self) -> rumbledash_core::Result<Config> {
        let config = Config::load(self.config_path.as_ref())?;
        if let Ok(mut guard) = self.config.try_write() {
            *guard = config.clone();
        }
        Ok(config)
    }

    /// Persists the shared configuration to disk.
    pub async fn persist_config(&self) -> rumbledash_core::Result<()> {
        let config = self.config.read().await.clone();
        config.save(self.config_path.as_ref())
    }

    /// Returns the currently known dashboard payload, fetching it when the
    /// cache is cold or stale.
    pub async fn dashboard(&self) -> Result<DashboardPayload, rumbledash_core::Error> {
        if let Some(cached) = self.cache.get() {
            return Ok(DashboardPayload {
                followers: cached.followers,
                subscribers: cached.subscribers,
                goals: cached.goals,
            });
        }

        let config = self.config.read().await.clone();
        if !config.has_api_url() {
            return Err(rumbledash_core::Error::Config(
                "API URL not set. Please set your Rumble API URL in the dashboard.".into(),
            ));
        }

        let stats = self.client.fetch_stats(&config.api_url).await?;
        let data = rumbledash_core::model::DashboardData::new(stats, config.goals);
        self.cache.put(data);

        Ok(DashboardPayload {
            followers: data.followers,
            subscribers: data.subscribers,
            goals: data.goals,
        })
    }
}

/// The JSON body returned by `/api/dashboard` – identical to the v2 payload.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DashboardPayload {
    /// Current follower count.
    pub followers: u64,
    /// Current subscriber count.
    pub subscribers: u64,
    /// The configured goals.
    pub goals: rumbledash_core::config::Goals,
}

/// Handle to a running server, used to shut it down again.
pub struct ServerHandle {
    /// The address the server actually listens on.
    pub addr: SocketAddr,
    /// LAN address that should be handed out to OBS.
    pub lan_ip: IpAddr,
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
}

impl ServerHandle {
    /// Stops the server.
    pub fn shutdown(mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
    }
}

/// Spawns the HTTP server on its own background thread and runtime.
///
/// Returns the handle plus the address it bound to.
pub fn spawn(state: AppState, port: u16) -> std::io::Result<ServerHandle> {
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let (ready_tx, ready_rx) = std::sync::mpsc::channel::<std::io::Result<SocketAddr>>();

    std::thread::Builder::new()
        .name("rumbledash-http".into())
        .spawn(move || {
            let runtime = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(error) => {
                    let _ = ready_tx.send(Err(error));
                    return;
                }
            };

            runtime.block_on(async move {
                let listener = match tokio::net::TcpListener::bind(("0.0.0.0", port)).await {
                    Ok(listener) => listener,
                    Err(error) => {
                        let _ = ready_tx.send(Err(error));
                        return;
                    }
                };

                let local = match listener.local_addr() {
                    Ok(local) => local,
                    Err(error) => {
                        let _ = ready_tx.send(Err(error));
                        return;
                    }
                };

                let _ = ready_tx.send(Ok(local));

                let router = routes::router(state);
                let server = axum::serve(listener, router).with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                });

                if let Err(error) = server.await {
                    tracing::error!(%error, "http server stopped");
                }
            });
        })?;

    let addr = match ready_rx.recv() {
        Ok(Ok(addr)) => addr,
        Ok(Err(error)) => return Err(error),
        Err(_) => {
            return Err(std::io::Error::other(
                "the http server thread stopped before it was ready",
            ))
        }
    };

    Ok(ServerHandle {
        addr,
        lan_ip: net::advertised_ip(),
        shutdown: Some(shutdown_tx),
    })
}
