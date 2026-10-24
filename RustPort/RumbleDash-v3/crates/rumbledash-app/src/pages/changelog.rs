//! The update history.

use egui::{Rect, Ui};

use crate::theme;
use crate::widgets;

/// A single changelog entry.
pub struct Entry {
    /// The version headline.
    pub title: &'static str,
    /// The bullet points of the entry.
    pub points: &'static [&'static str],
}

/// The changelog, newest first.
pub const ENTRIES: &[Entry] = &[
    Entry {
        title: "RumbleDash v3.1.1 - AppImage & Config Fix",
        points: &[
            "Settings now save correctly in read-only AppImages (falls back to your user config directory and writes atomically).",
            "The app searches for config.json next to the executable first, so portable installs keep working as expected.",
        ],
    },
    Entry {
        title: "RumbleDash v3.1.0 - The Polish Update",
        points: &[
            "Goal labels shorten themselves once they get big: 1,000 reads 1k, 1,500 reads 1.5k, and the same carries through millions and billions.",
            "Completed progress bars now animate - a sweep of light crosses the fill and the bar flashes once when the goal is reached.",
            "The desktop dashboard previews how each goal will read on the overlay and celebrates a full bar too.",
        ],
    },
    Entry {
        title: "RumbleDash v3.0.0 - The Rust Port",
        points: &[
            "Complete rewrite in Rust with a native egui desktop dashboard.",
            "The OBS widgets are now served by a built-in HTTP server, so no Node.js runtime is needed.",
            "Configuration is stored in the same config.json format as v2.",
            "The feedback form, update history and contributor pages are part of the desktop app.",
            "Widgets keep their gradients and can be themed from the app.",
        ],
    },
    Entry {
        title: "RumbleDash v2.28 - November 11th 2024",
        points: &[
            "Added a new contributor check: the contributors page.",
            "Contributor and author added some new changes; the backend was optimized and the CSS was fine-tuned.",
        ],
    },
    Entry {
        title: "RumbleDash v2.27 - November 11th 2024",
        points: &["Updated the interactive custom banner ads."],
    },
    Entry {
        title: "RumbleDash v2.26 - November 11th 2024",
        points: &[
            "Organised the CSS files with CSS variables to keep a consistent style across the entire project.",
            "Added interactive affiliate ad banners.",
        ],
    },
    Entry {
        title: "RumbleDash v2.25 - November 10th 2024",
        points: &[
            "Redesigned the dashboard UI.",
            "Fixed backend bugs that would stop the project.",
            "Added the \"Made with love from South Africa\" branding.",
            "Added two new pages: \"Update History\" and \"Submit Your Feedback\".",
            "Added PayPal buttons for supporting the project.",
        ],
    },
    Entry {
        title: "RumbleDash v2.24 - November 8th 2024",
        points: &[
            "New re-skin.",
            "Widget overlays are re-skinned with a gradient added.",
            "Monetisation methods were added; the ads were replaced by PayPal buttons.",
            "Fixed a bug that made the project stop running after about an hour.",
        ],
    },
];

/// Draws the changelog page.
pub fn changelog(ui: &mut Ui) {
    widgets::page_header(
        ui,
        "Update history",
        "Here is a list of all the updates and changes made to the project. Stay up to date \
         with the latest enhancements, bug fixes and improvements.",
    );

    let mut previous_bottom: Option<f32> = None;

    for entry in ENTRIES {
        ui.horizontal_top(|ui| {
            // The gutter the timeline is drawn in.
            ui.add_sized([28.0, 0.0], egui::Label::new(""));

            let response = egui::Frame::new()
                .fill(theme::SURFACE)
                .stroke(egui::Stroke::new(1.0, theme::BORDER))
                .corner_radius(theme::radius())
                .inner_margin(egui::Margin::symmetric(20, 18))
                .show(ui, |ui| {
                    ui.label(theme::subtitle(entry.title));
                    ui.add_space(theme::SPACE_MD);
                    for point in entry.points {
                        ui.horizontal_top(|ui| {
                            ui.label(
                                egui::RichText::new("\u{2022}")
                                    .size(16.0)
                                    .color(theme::EMBER),
                            );
                            ui.label(theme::body(*point));
                        });
                        ui.add_space(theme::SPACE_XS);
                    }
                });

            let rect = response.response.rect;
            let rail_x = rect.left() - 14.0;

            if let Some(bottom) = previous_bottom {
                ui.painter().rect_filled(
                    Rect::from_min_max(
                        egui::pos2(rail_x, bottom - 8.0),
                        egui::pos2(rail_x + 2.0, rect.top() + 18.0),
                    ),
                    1.0,
                    theme::BORDER,
                );
            }

            ui.painter().circle_filled(
                egui::pos2(rail_x + 1.0, rect.top() + 18.0),
                4.5,
                theme::EMBER,
            );

            previous_bottom = Some(rect.bottom());
        });

        ui.add_space(theme::SPACE_MD);
    }

    ui.add_space(theme::SPACE_LG);
    widgets::branding_badge(ui);
}
