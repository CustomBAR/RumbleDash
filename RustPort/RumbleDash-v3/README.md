# RumbleDash v3 – Rust Port

A native rewrite of the [RumbleDash](https://github.com/CustomBAR/RumbleDash) streamer
dashboard. The v2 project was an Express + HTML/CSS web app; v3 keeps the same look
and the same HTTP contract, but replaces the web frontend with a **Rust + egui desktop
application** and the Node backend with a small embedded **axum** server.

```
RustPort/RumbleDash-v3/
├── Cargo.toml                    # workspace
└── crates/
    ├── rumbledash-core/          # config, Rumble API client, cache, data model
    ├── rumbledash-server/        # axum HTTP server + embedded OBS widget assets
    └── rumbledash-app/           # egui desktop application (binary: rumbledash)
```

---

## Quick start

```bash
cd RustPort/RumbleDash-v3

# desktop app (starts the widget server in the background)
cargo run --bin rumbledash

# headless widget server only
cargo run --bin rumbledash-server
```

The release build is stripped and LTO-optimised:

```bash
cargo build --release
# -> target/release/rumbledash.exe
# -> target/release/rumbledash-server.exe
```

### Configuration

`config.json` is read from the current working directory – exactly like v2 – so an
existing v2 installation can be dropped straight in:

```json
{
  "apiUrl": "",
  "goals": {
    "followers": 50,
    "subscribers": 2
  }
}
```

| Environment variable    | Meaning                                              | Default      |
|-------------------------|------------------------------------------------------|--------------|
| `RUMBLE_DASH_CONFIG`    | Path of the `config.json` to use                     | `./config.json` |
| `RUMBLE_DASH_PORT`      | TCP port for the widget server                        | `9000`       |
| `PORT`                  | Fallback for `RUMBLE_DASH_PORT`                      | `9000`       |
| `RUST_LOG`              | Log filter for the `tracing` subscriber              | `info`       |

---

## Using the widgets in OBS

1. Start RumbleDash (the desktop app or the headless server).
2. Copy the widget URL shown in the right rail of the desktop app, e.g.
   `http://192.168.1.20:9000/followers`.
3. In OBS add a **Browser** source and paste the URL.

| Widget           | URL                  | Shows                                  |
|------------------|----------------------|----------------------------------------|
| Followers        | `/followers`         | follower count and goal progress bar   |
| Subscribers      | `/subscribers`       | subscriber count and goal progress bar |

The desktop app also shows a small browser status page at `/` with the live numbers.

---

## The desktop interface

v3 was re-laid-out from scratch: the old v2-style sidebar and stacked card column
are gone. The window is now split into three zones that stay visible at all times.

```
┌──────────────────────────────────────────────────────────────────────┐
│ ▣ RumbleDash v3   Dashboard│Feedback│History│Team        ● Live 4s  │  top bar
├──────────────────────────────────────────────┬───────────────────────┤
│ Dashboard                            [ad]     │ LIVE SNAPSHOT         │
│ Set your counters, connect …                 │  Followers    4,242   │
│ ┌────────────┐┌────────────┐┌────────────┐   │  ▓▓▓▓▓▓▓▓▓▓░░  84%   │
│ │ Followers  ││ Subscribers││ Widget     │   │  Subscribers     17   │
│ │ 4,242      ││ 17         ││ server     │   │  ▓▓▓░░░░░░░░░░░  85%   │
│ └────────────┘└────────────┘└────────────┘   │ WIDGET URLS           │
│ GOALS                                         │  http://…/followers   │
│ ┌──────────────────────────────────────────┐  │  [Copy]               │
│ │ Counter goals                            │  │ TOOLS                 │
│ │ Follower goal [5000]  Subscriber [20]    │  │  [Open dashboard]     │
│ └──────────────────────────────────────────┘  │ SUPPORT               │
│ CONNECTION                                   │  [Donate via PayPal]   │
│ ┌──────────────────────────────────────────┐  │                       │
│ │ Rumble API URL [••••••]      [Show]       │  │                       │
│ │ [Clear API URL]  [Save configuration]     │  │                       │
│ └──────────────────────────────────────────┘  │                       │
└──────────────────────────────────────────────┴───────────────────────┘
```

* **Top bar** – brand, segmented navigation (Dashboard / Feedback / Update History /
  Contributors) and a live status chip that shows how fresh the numbers are.
* **Right rail** – the live snapshot with progress bars, the three widget URLs with a
  one-click *Copy* button, secondary tools and the PayPal donate button.
* **Content area** – the page itself, built from a page header, a status banner, the
  rotating ad banner and a stack of labelled sections.

### Design system

The palette is deliberately narrow: a cool **charcoal** base with exactly one warm
accent – **ember orange**. All tokens live in `crates/rumbledash-app/src/theme.rs`.

| Token         | Value     | Used for                              |
|---------------|-----------|---------------------------------------|
| `BACKDROP`    | `#131417` | the window background                  |
| `SURFACE`     | `#1c1e23` | cards, panels, the top bar             |
| `SURFACE_RAISED` | `#25282e` | inputs, chips, hover rows           |
| `BORDER`      | `#2f323a` | hairline borders                       |
| `EMBER`       | `#ff6b1a` | the primary accent – buttons, progress |
| `EMBER_LIGHT` | `#ffa366` | gradient heads, highlights             |
| `TEXT`        | `#f3f4f6` | primary text                           |
| `TEXT_MUTED`  | `#9aa0aa` | secondary text                         |
| `SUCCESS` / `DANGER` / `INFO` | – | status colours                   |

Layout only ever picks from a **4 px spacing grid**, three corner radii (14 / 9 / pill)
and a six step type scale (display 30 → micro 10.5), which is what keeps the interface
looking deliberate instead of improvised.

Every column of the interface – the top bar, the content area and the right rail – is
inset by the same **`GUTTER` of 24 px**, so the three zones start and end on exactly the
same vertical lines. Inside the rail the section rules, the copy fields, the *Copy*
buttons and the full width action buttons all share one content box: the copy field
takes whatever width is left after the fixed 62 px *Copy* button and the 8 px gap have
been subtracted, and long URLs shrink their font instead of pushing the button out of
line.

egui has no notion of a bold weight, so the app probes the operating system for a real
bold face (Segoe UI Bold, Arial Bold, DejaVu Sans Bold, …) and registers it as its own
font family. When no bold face can be found the design still holds together – hierarchy
is carried by size, colour and spacing as well.

The OBS widgets and the browser status page use the same ember palette, so the whole
product reads as one thing.

---

## Pages

* **Dashboard** – overview tiles (followers, subscribers, widget server), the goal
  card, and the connection card with a show/hide toggle for the API URL.
* **Feedback** – posts to the same Formspree endpoint as v2.
* **Update History** – the v2 changelog rendered as a timeline plus the v3.0.0 entry.
* **Contributors** – the v2 contributor wall, with the channel logo fetched at runtime.

The rotating ad banner (the same five promotional banners as v2) sits under the page
header and refreshes every 30 seconds.

---

## HTTP API

All v2 endpoints are preserved so existing OBS sources and scripts keep working.

| Method | Route                  | Description                                    |
|--------|------------------------|------------------------------------------------|
| GET    | `/api/dashboard`       | `{"followers":N,"subscribers":N,"goals":{…}}`   |
| GET    | `/api/config`          | the current configuration                      |
| POST   | `/api/update-config`   | persists a new configuration                   |
| POST   | `/api/clear-api-url`   | wipes the stored Rumble API URL                |
| GET    | `/api/server-ip`       | LAN address + port for the widget URLs         |
| GET    | `/api/health`          | readiness probe (new)                          |
| GET    | `/followers`           | OBS widget – followers                         |
| GET    | `/subscribers`         | OBS widget – subscribers                       |
| GET    | `/styles/{name}.css`   | bundled stylesheet                             |
| GET    | `/scripts/{name}.js`   | bundled script                                 |
| GET    | `/`                    | browser status page                            |
| GET    | `/form`, `/changelog`, `/contributors` | v2 browser pages – now inside the desktop app |

The widget assets are compiled into the binary with `include_str!`, so the server has
no runtime file dependencies.

### Rumble API payload

The private livestream API is parsed leniently – all of these work:

```jsonc
{ "followers": { "num_followers": 12 }, "subscribers": { "num_subscribers": 3 } }
{ "followers": 12, "subscribers": 3 }
{ "followers": { "num_followers": "12" }, "subscribers": { "count": 3 } }
```

---

## v2 → v3 mapping

| v2 (Node)                              | v3 (Rust)                              |
|----------------------------------------|-----------------------------------------|
| `server.js` (Express)                  | `rumbledash-server` (axum)               |
| `public/index.html` + `dashboardui.css` | `rumbledash-app` (egui, `theme.rs` + `widgets.rs`) |
| `pages/form.html`                      | `pages/feedback.rs`                      |
| `pages/changelog.html`                 | `pages/changelog.rs`                     |
| `pages/contributors.html`              | `pages/contributors.rs`                  |
| `widgets/*.html` + `public/*.css`      | `crates/rumbledash-server/assets/`       |
| `config.json`                          | `crates/rumbledash-core/src/config.rs`   |
| `axios` calls to the Rumble API        | `rumbledash-core/src/api.rs` (reqwest)   |
| `pages/js/script.js` (ad rotator)      | `RumbleDashApp::rotate_ads`              |
| –                                      | `rumbledash-core/src/cache.rs` (5 s TTL) |

---

## Development

```bash
cargo build --workspace         # build everything
cargo test --workspace          # unit tests for config, api, cache and model
cargo clippy --workspace --all-targets
cargo fmt --all
```

### Toolchain note

On Windows the MSVC toolchain needs the Visual Studio Build Tools linker. If
`link.exe` is missing, use the GNU toolchain instead:

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
cargo build --workspace
```

---

## License

MIT – same as the original project.
