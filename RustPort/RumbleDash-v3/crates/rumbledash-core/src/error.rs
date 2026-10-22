use thiserror::Error;

/// Errors produced by the core crate.
#[derive(Debug, Error)]
pub enum Error {
    /// The `config.json` payload could not be understood.
    #[error("invalid configuration: {0}")]
    Config(String),

    /// A request to the Rumble API failed.
    #[error("network error: {0}")]
    Http(#[from] reqwest::Error),

    /// The Rumble API answered with something we could not parse.
    #[error("could not parse Rumble API response: {0}")]
    Parse(#[from] serde_json::Error),

    /// Reading or writing `config.json` failed.
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    /// Anything else.
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Convenience helper for ad-hoc messages.
    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }
}

/// Result alias used across the workspace.
pub type Result<T> = std::result::Result<T, Error>;
