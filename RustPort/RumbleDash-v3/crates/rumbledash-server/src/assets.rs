//! Static assets that are compiled into the binary.
//!
//! Embedding the widgets means the server has no runtime dependencies – the
//! OBS browser source keeps working even when the executable is moved around.

/// The followers overlay.
pub const FOLLOWERS_HTML: &str = include_str!("../assets/widgets/followers.html");
/// The subscribers overlay.
pub const SUBSCRIBERS_HTML: &str = include_str!("../assets/widgets/subscribers.html");

/// Stylesheet for the followers overlay.
pub const FOLLOWERS_CSS: &str = include_str!("../assets/styles/followers.css");
/// Stylesheet for the subscribers overlay.
pub const SUBSCRIBERS_CSS: &str = include_str!("../assets/styles/subscribers.css");
/// Stylesheet for the browser status page.
pub const DASHBOARD_CSS: &str = include_str!("../assets/styles/dashboard.css");

/// Shared widget script.
pub const WIDGET_JS: &str = include_str!("../assets/scripts/widget.js");
/// Status page script.
pub const STATUS_JS: &str = include_str!("../assets/scripts/status.js");

/// The browser status page.
pub const INDEX_HTML: &str = include_str!("../assets/index.html");
/// Placeholder for the v2 browser pages that moved into the desktop app.
pub const LEGACY_HTML: &str = include_str!("../assets/legacy.html");

/// Looks up a stylesheet by name (without the `.css` suffix).
pub fn stylesheet(name: &str) -> Option<&'static str> {
    match name {
        "followers" => Some(FOLLOWERS_CSS),
        "subscribers" => Some(SUBSCRIBERS_CSS),
        "dashboard" => Some(DASHBOARD_CSS),
        _ => None,
    }
}

/// Looks up a script by name (without the `.js` suffix).
pub fn script(name: &str) -> Option<&'static str> {
    match name {
        "widget" => Some(WIDGET_JS),
        "status" => Some(STATUS_JS),
        _ => None,
    }
}
