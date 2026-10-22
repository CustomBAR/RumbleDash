# RustPort

This folder holds the Rust rewrite of RumbleDash.

| Folder            | Description                                                        |
|-------------------|--------------------------------------------------------------------|
| `RumbleDash-v3/`  | The v3 port – a Rust workspace with an egui desktop app and an axum widget server. |

The original Node.js project stays untouched in [`../RumbleDash-v2.28`](../RumbleDash-v2.28)
and is used as the reference for the behaviour and the HTTP contract. The v3 look and
feel is a deliberate departure: a charcoal interface with ember orange accents.

## v2 → v3 at a glance

* **v2.28** – Express server, HTML/CSS dashboard in the browser, OBS widgets served
  from `public/` and `widgets/`.
* **v3.0.0** – the dashboard becomes a **native egui desktop app**, the Express
  server becomes a small **axum** server that is embedded in the app (and is also
  available as a standalone headless binary), and the configuration format stays
  byte-compatible with v2.

See [`RumbleDash-v3/README.md`](RumbleDash-v3/README.md) for the full documentation.

## Build & run

```bash
cd RumbleDash-v3
cargo run --bin rumbledash          # desktop application
cargo run --bin rumbledash-server   # headless widget server
```
