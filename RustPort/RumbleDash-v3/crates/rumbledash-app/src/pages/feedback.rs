//! The feedback form.

use egui::Ui;

use crate::app::RumbleDashApp;
use crate::theme;
use crate::widgets;

/// Draws the feedback page.
pub fn feedback(app: &mut RumbleDashApp, ui: &mut Ui) {
    widgets::page_header(ui, app.page.heading(), app.page.description());

    if let Some(message) = app.message.as_ref() {
        let color = app.message_color().unwrap_or(theme::TEXT_MUTED);
        widgets::status_banner(ui, color, &message.text);
        ui.add_space(theme::SPACE_LG);
    }

    ui.columns(2, |cols| {
        let (left, right) = cols.split_at_mut(1);
        form(app, &mut left[0]);
        artwork(app, &mut right[0]);
    });

    ui.add_space(theme::SPACE_XL);
    widgets::branding_badge(ui);
}

/// The form itself.
fn form(app: &mut RumbleDashApp, ui: &mut Ui) {
    widgets::card(ui, |ui| {
        ui.label(theme::subtitle("Your feedback is greatly appreciated"));
        ui.add_space(theme::SPACE_XS);
        ui.label(theme::body(
            "Tell us what works, what does not, or what you would like to see next.",
        ));
        ui.add_space(theme::SPACE_LG);

        widgets::text_field(
            ui,
            "Your name",
            &mut app.feedback.name,
            "Enter your name",
            false,
        );
        ui.add_space(theme::SPACE_MD);

        widgets::text_field(
            ui,
            "Your e-mail",
            &mut app.feedback.email,
            "Enter your email",
            false,
        );
        ui.add_space(theme::SPACE_MD);

        widgets::multiline_field(
            ui,
            "Your message",
            &mut app.feedback.message,
            "Type your message here",
            6,
        );
        ui.add_space(theme::SPACE_LG);

        if widgets::button(ui, "Send feedback", widgets::ButtonStyle::Primary).clicked() {
            app.send_feedback();
        }
    });
}

/// The artwork next to the form.
fn artwork(app: &mut RumbleDashApp, ui: &mut Ui) {
    widgets::card(ui, |ui| {
        if let Some(art) = app.feedback_art.clone() {
            ui.add(
                egui::Image::from_texture(&art)
                    .max_width(ui.available_width())
                    .corner_radius(theme::radius_sm()),
            );
        }

        ui.add_space(theme::SPACE_MD);
        ui.label(theme::subtitle("RumbleDash v3"));
        ui.add_space(theme::SPACE_XS);
        ui.label(theme::body(
            "A native desktop dashboard for streamers – no Node.js, no browser tab, \
             just the widgets you need.",
        ));
    });
}
