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

        let json = self.to_json()?;
        // Write to a temp file first, then rename it. This avoids leaving a
        // truncated config.json behind if the process is killed mid-write
        // (and is more reliable on read-only overlay filesystems like AppImages).
        let mut tmp_path = path.to_path_buf();
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("config.json");
        tmp_path.set_file_name(format!(".{file_name}.tmp-{}", std::process::id()));
        std::fs::write(&tmp_path, &json)?;
        std::fs::rename(&tmp_path, path).or_else(|_| {
            // Fall back to a direct write if rename isn't possible (e.g. cross-device).
            let _ = std::fs::remove_file(path);
            std::fs::write(path, &json)
        })?;
        let _ = std::fs::remove_file(&tmp_path);
        Ok(())
    }
}

/// Resolves the location of `config.json`.
///
/// * `RUMBLE_DASH_CONFIG` overrides everything (useful for portable installs)
/// * otherwise we try a few sensible locations in order:
///   1. `config.json` next to the executable (portable installs)
///   2. `config.json` next to the current working directory (matches v2 behaviour)
///   3. The user's configuration directory (e.g. `~/.config/rumbledash/config.json`
///      on Linux/macOS or `%APPDATA%\\RumbleDash\\config.json` on Windows)
///      so AppImages and read-only installs can still persist settings.
pub fn default_config_path() -> PathBuf {
    if let Ok(custom) = std::env::var("RUMBLE_DASH_CONFIG") {
        if !custom.trim().is_empty() {
            return PathBuf::from(custom);
        }
    }

    // Try next to the executable first (works well for portable zips/AppDirs).
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let path = dir.join("config.json");
            // Prefer an existing config next to the exe, otherwise fall through
            // to other locations (we may not be able to write next to the exe in
            // read-only AppImages, but we still want to discover it there).
            if path.exists() {
                return path;
            }
            // If the exe directory looks writable, use it.
            if is_writable_dir(dir) {
                return path;
            }
        }
    }

    // Fall back to the current working directory (v2 compatibility).
    let cwd_path = PathBuf::from("config.json");
    if cwd_path.exists() {
        return cwd_path;
    }
    if let Ok(cwd) = std::env::current_dir() {
        if is_writable_dir(&cwd) {
            return cwd_path;
        }
    }

    // Last resort: use the OS user configuration directory.
    if let Some(mut dir) = user_config_dir() {
        dir.push("RumbleDash");
        let path = dir.join("config.json");
        return path;
    }

    cwd_path
}

fn is_writable_dir(dir: &Path) -> bool {
    if !dir.exists() {
        return false;
    }
    let test_path = dir.join(".rumbledash-write-test");
    match std::fs::File::create(&test_path) {
        Ok(_) => {
            let _ = std::fs::remove_file(&test_path);
            true
        }
        Err(_) => false,
    }
}

fn user_config_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            if !appdata.trim().is_empty() {
                return Some(PathBuf::from(appdata));
            }
        }
        None
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            if !xdg.trim().is_empty() {
                return Some(PathBuf::from(xdg));
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            if !home.trim().is_empty() {
                let mut dir = PathBuf::from(home);
                dir.push(".config");
                return Some(dir);
            }
        }
        None
    }

    #[cfg(not(any(
        target_os = "windows",
        target_os = "linux",
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    )))]
    {
        std::env::current_dir().ok()
    }
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
