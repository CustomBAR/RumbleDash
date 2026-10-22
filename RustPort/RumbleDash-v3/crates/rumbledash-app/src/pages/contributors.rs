//! The contributor wall.

use egui::{Rect, Ui};

use crate::app::RumbleDashApp;
use crate::sys;
use crate::theme;
use crate::widgets;

/// The channel logo of "Constantly Curious".
pub const CUSTOMER_LOGO_URL: &str =
    "https://1a-1791.com/video/z8/v/R/d/z/vRdza.baa-ConstantlyCurious-sgs4w0.png";

/// A person who contributed to the project.
pub struct Contributor {
    /// The name shown on the card.
    pub name: &'static str,
    /// The channel the card links to.
    pub url: &'static str,
    /// What this person did.
    pub role: &'static str,
}

/// The people who helped with RumbleDash.
pub const CONTRIBUTORS: &[Contributor] = &[
    Contributor {
        name: "Constantly Curious",
        url: "https://rumble.com/c/ConstantlyCurious",
        role: "Design, branding and the original dashboard concept.",
    },
    Contributor {
        name: "CustomBAR",
        url: "https://github.com/CustomBAR/RumbleDash",
        role: "Maintainer of RumbleDash and the author of the Rust rewrite.",
    },
];

/// Draws the contributors page.
pub fn contributors(app: &mut RumbleDashApp, ui: &mut Ui) {
    widgets::page_header(
        ui,
        "Contributors",
        "This project had help from all these amazing people!",
    );

    ui.columns(2, |cols| {
        let (left, right) = cols.split_at_mut(1);
        contributor_card(app, &mut left[0], &CONTRIBUTORS[0]);
        contributor_card(app, &mut right[0], &CONTRIBUTORS[1]);
    });

    ui.add_space(theme::SPACE_XL);
    widgets::branding_badge(ui);
}

/// A single contributor card.
fn contributor_card(app: &mut RumbleDashApp, ui: &mut Ui, contributor: &Contributor) {
    let logo = app.contributor_logo.clone();

    widgets::card(ui, |ui| {
        ui.horizontal(|ui| {
            if let Some(logo) = logo {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(64.0, 64.0), egui::Sense::click());
                if ui.is_rect_visible(rect) {
                    let painter = ui.painter();
                    painter.rect_filled(rect, theme::radius_sm(), theme::SURFACE_RAISED);
                    let size = egui::load::SizedTexture::from_handle(&logo).size;
                    if size.x > 0.0 && size.y > 0.0 {
                        let scale = (64.0 / size.x).min(64.0 / size.y);
                        let draw_size = egui::vec2(size.x * scale, size.y * scale);
                        painter.image(
                            logo.id(),
                            Rect::from_center_size(rect.center(), draw_size),
                            Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            egui::Color32::WHITE,
                        );
                    }
                }
                if response.clicked() {
                    sys::open_url(contributor.url);
                }
            } else {
                monogram(ui, contributor);
            }

            ui.vertical(|ui| {
                ui.label(theme::subtitle(contributor.name));
                ui.add_space(theme::SPACE_XS);
                ui.label(theme::caption(contributor.role));
            });
        });

        ui.add_space(theme::SPACE_LG);
        if widgets::button(ui, "Visit profile", widgets::ButtonStyle::Secondary).clicked() {
            sys::open_url(contributor.url);
        }
    });
}

/// The placeholder avatar used while the channel logo is downloading.
fn monogram(ui: &mut Ui, contributor: &Contributor) {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(64.0, 64.0), egui::Sense::click());
    if !ui.is_rect_visible(rect) {
        return;
    }

    let painter = ui.painter();
    painter.rect_filled(rect, theme::radius_sm(), theme::tint(theme::EMBER, 0.18));

    let initial = contributor
        .name
        .chars()
        .next()
        .map(|character| character.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_owned());

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        initial,
        egui::FontId::new(26.0, theme::bold_family()),
        theme::EMBER_LIGHT,
    );

    if response.clicked() {
        sys::open_url(contributor.url);
    }
}
