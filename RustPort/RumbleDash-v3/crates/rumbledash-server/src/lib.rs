//! The HTTP side of RumbleDash v3.
//!
//! It keeps the endpoints the v2 Node build exposed so that existing OBS
//! browser sources keep working:
//!
//! | Route                   | Purpose                                    |
//! |-------------------------|--------------------------------------------|
//! | `GET /api/dashboard`    | follower / subscriber counters + goals      |
//! | `GET /api/config`       | current configuration                       |
//! | `POST /api/update-config` | persists a new configuration              |
//! | `POST /api/clear-api-url` | wipes the stored Rumble API URL           |
//! | `GET /api/server-ip`    | LAN address + port, used to build widget URLs|
//! | `GET /followers`        | OBS widget – followers                      |
//! | `GET /subscribers`      | OBS widget – subscribers                    |

pub mod assets;
pub mod net;
pub mod routes;
pub mod state;

pub use state::{AppState, ServerHandle};
