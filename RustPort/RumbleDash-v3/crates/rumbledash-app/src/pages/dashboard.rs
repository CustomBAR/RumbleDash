//! The configuration panel – the native equivalent of the v2 dashboard.

use egui::Ui;

use crate::app::{ratio, RumbleDashApp};
use crate::theme;
use crate::widgets;

/// Draws the dashboard page.
pub fn dashboard(app: &mut RumbleDashApp, ui: &mut Ui) {
    widgets::page_header(ui, app.page.heading(), app.page.description());

    if let Some(message) = app.message.as_ref() {
        let color = app.message_color().unwrap_or(theme::TEXT_MUTED);
        widgets::status_banner(ui, color, &message.text);
        ui.add_space(theme::SPACE_LG);
    }

    app.ad_banner(ui);
    ui.add_space(theme::SPACE_LG);

    overview(app, ui);
    ui.add_space(theme::SPACE_LG);

    goals(app, ui);
    ui.add_space(theme::SPACE_LG);

    connection(app, ui);

    ui.add_space(theme::SPACE_XL);
    widgets::branding_badge(ui);
}

/// The row of headline numbers.
fn overview(app: &mut RumbleDashApp, ui: &mut Ui) {
    let followers = app.followers();
    let subscribers = app.subscribers();
    let followers_goal = app.draft.followers_goal;
    let subscribers_goal = app.draft.subscribers_goal;
    let live = app.is_live();
    let server = app.base_url();

    widgets::section(ui, "Overview");

    ui.columns(3, |cols| {
        let tiles = [
            widgets::StatTile {
                label: "Followers",
                value: format_number(followers),
                caption: format!("Goal {}", format_number(followers_goal)),
                accent: theme::EMBER,
                progress: Some(ratio(followers, followers_goal)),
            },
            widgets::StatTile {
                label: "Subscribers",
                value: format_number(subscribers),
                caption: format!("Goal {}", format_number(subscribers_goal)),
                accent: theme::EMBER_LIGHT,
                progress: Some(ratio(subscribers, subscribers_goal)),
            },
            widgets::StatTile {
                label: "Widget server",
                value: if live { "Running" } else { "Idle" }.to_owned(),
                caption: server,
                accent: theme::SUCCESS,
                progress: None,
            },
        ];

        for (column, tile) in cols.iter_mut().zip(tiles) {
            widgets::stat_tile(column, tile);
        }
    });
}

/// The card that holds the two counter goals.
fn goals(app: &mut RumbleDashApp, ui: &mut Ui) {
    widgets::section(ui, "Goals");

    widgets::card(ui, |ui| {
        ui.label(theme::subtitle("Counter goals"));
        ui.add_space(theme::SPACE_XS);
        ui.label(theme::body(
            "The goals drive the progress bars of your OBS overlays.",
        ));
        ui.add_space(theme::SPACE_LG);

        ui.columns(2, |cols| {
            let (left, right) = cols.split_at_mut(1);

            left[0].add_space(theme::SPACE_XS);
            widgets::number_field(
                &mut left[0],
                "Follower goal",
                &mut app.draft.followers_goal,
                "5000",
            );

            right[0].add_space(theme::SPACE_XS);
            widgets::number_field(
                &mut right[0],
                "Subscriber goal",
                &mut app.draft.subscribers_goal,
                "20",
            );
        });
    });
}

/// The card that holds the Rumble API URL.
fn connection(app: &mut RumbleDashApp, ui: &mut Ui) {
    widgets::section(ui, "Connection");

    let visible = app.api_url_is_visible();
    let toggle_label = if visible { "Hide" } else { "Show" };

    widgets::card(ui, |ui| {
        ui.label(theme::subtitle("Rumble API URL"));
        ui.add_space(theme::SPACE_XS);
        ui.label(theme::body(
            "The private URL of your Rumble livestream API. It is stored in config.json and \
             never leaves this machine.",
        ));
        ui.add_space(theme::SPACE_LG);

        widgets::text_field(
            ui,
            "",
            &mut app.draft.api_url,
            "https://rumble.com/api/v0/livestream/…",
            !visible,
        );

        ui.add_space(theme::SPACE_MD);
        ui.horizontal(|ui| {
            ui.label(theme::caption("Get your URL from the "));
            ui.hyperlink_to(
                theme::link("Rumble live streaming API page"),
                "https://rumble.com/account/livestream-api",
            );
            ui.label(theme::caption(" and paste it above."));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if widgets::button(ui, toggle_label, widgets::ButtonStyle::Ghost).clicked() {
                    app.toggle_api_url_visibility();
                }
            });
        });

        ui.add_space(theme::SPACE_LG);
        ui.horizontal(|ui| {
            if widgets::button(ui, "Clear API URL", widgets::ButtonStyle::Danger).clicked() {
                app.clear_api_url();
            }
            ui.add_space(theme::SPACE_SM);
            if widgets::button(ui, "Save configuration", widgets::ButtonStyle::Primary).clicked() {
                app.save_config();
            }
        });
    });
}

/// Formats a number with thin spaces as the thousands separator.
fn format_number(value: u64) -> String {
    let digits = value.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);

    let bytes = digits.as_bytes();
    for (index, byte) in bytes.iter().enumerate() {
        if index > 0 && (bytes.len() - index) % 3 == 0 {
            out.push(',');
        }
        out.push(*byte as char);
    }

    out
}
