use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Goals for the follower / subscriber counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goals {
    /// Target number of followers.
    #[serde(default)]
    pub followers: u64,
    /// Target number of subscribers.
    #[serde(default)]
    pub subscribers: u64,
}

impl Default for Goals {
    fn default() -> Self {
        // Mirrors the defaults shipped in the v2 `config.json`.
        Self {
            followers: 50,
            subscribers: 2,
        }
    }
}

/// The on-disk configuration, kept byte-compatible with the v2 `config.json`
/// so existing installs can be dropped straight into the Rust build.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// The private Rumble livestream API URL.
    #[serde(rename = "apiUrl", default)]
    pub api_url: String,
    /// Counter goals.
    #[serde(default)]
    pub goals: Goals,
}

impl Config {
    /// Builds a config from raw JSON, tolerating a missing/empty document.
    pub fn from_json(raw: &str) -> crate::Result<Self> {
        if raw.trim().is_empty() {
            return Ok(Self::default());
        }
        let config: Self = serde_json::from_str(raw)?;
        Ok(config)
    }

    /// Serialises the config back into the v2 compatible JSON layout.
    pub fn to_json(&self) -> crate::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// `true` when a Rumble API URL has been configured.
    pub fn has_api_url(&self) -> bool {
        !self.api_url.trim().is_empty()
    }

    /// Loads the config from `path`, falling back to the defaults when the
    /// file does not exist yet.
    pub fn load(path: impl AsRef<Path>) -> crate::Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(path)?;
        Self::from_json(&raw)
    }

    /// Writes the config to `path`, creating the parent directory if needed.
    pub fn save(&self, path: impl AsRef<Path>) -> crate::Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(path, self.to_json()?)?;
        Ok(())
    }
}

/// Resolves the location of `config.json`.
///
/// * `RUMBLE_DASH_CONFIG` overrides everything (useful for portable installs)
/// * otherwise `config.json` next to the current working directory is used,
///   which keeps the behaviour of the v2 Node build.
pub fn default_config_path() -> PathBuf {
    if let Ok(custom) = std::env::var("RUMBLE_DASH_CONFIG") {
        if !custom.trim().is_empty() {
            return PathBuf::from(custom);
        }
    }
    PathBuf::from("config.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The configuration shipped with the v2 release.
    const V2_CONFIG: &str = r#"{
  "apiUrl": "",
  "goals": {
    "followers": 50,
    "subscribers": 2
  }
}"#;

    #[test]
    fn reads_the_v2_configuration() {
        let config = Config::from_json(V2_CONFIG).unwrap();

        assert_eq!(config.api_url, "");
        assert_eq!(config.goals.followers, 50);
        assert_eq!(config.goals.subscribers, 2);
        assert!(!config.has_api_url());
    }

    #[test]
    fn writes_the_v2_configuration_back() {
        let config = Config::from_json(V2_CONFIG).unwrap();
        let json = config.to_json().unwrap();

        assert!(json.contains("\"apiUrl\""));
        assert!(json.contains("\"followers\""));
        assert!(json.contains("\"subscribers\""));

        let round_trip = Config::from_json(&json).unwrap();
        assert_eq!(round_trip, config);
    }

    #[test]
    fn an_empty_document_yields_the_defaults() {
        let config = Config::from_json("   ").unwrap();

        assert_eq!(config.api_url, "");
        assert_eq!(config.goals, Goals::default());
    }

    #[test]
    fn a_missing_file_yields_the_defaults() {
        let path = std::env::temp_dir().join("rumbledash-does-not-exist.json");
        let config = Config::load(&path).unwrap();

        assert_eq!(config, Config::default());
    }

    #[test]
    fn saving_and_loading_round_trips() {
        let path = std::env::temp_dir().join("rumbledash-config-round-trip.json");
        let config = Config {
            api_url: "https://example.com/api".into(),
            goals: Goals {
                followers: 10,
                subscribers: 20,
            },
        };

        config.save(&path).unwrap();
        let loaded = Config::load(&path).unwrap();

        assert_eq!(loaded, config);
        let _ = std::fs::remove_file(&path);
    }
}
