//! The RumbleDash component library.
//!
//! egui's stock widgets do not allow per-button hover colours and they do not
//! share a common visual language, so the handful of components the app needs
//! are painted by hand here. Everything is built from the tokens in
//! [`crate::theme`], which is what keeps the interface looking designed rather
//! than improvised.
//!
//! The components are grouped into:
//!
//! * **Containers** – [`card`], [`section`], [`page_header`]
//! * **Data display** – [`stat_tile`], [`progress_bar`], [`chip`], [`kv_row`]
//! * **Actions** – [`button`], [`button_block`], [`nav_tab`], [`copy_field`]
//! * **Input** – [`text_field`], [`number_field`], [`multiline_field`]
//! * **Feedback** – [`status_banner`], [`divider`], [`branding_badge`]

use egui::{
    Color32, FontFamily, FontId, Margin, Rect, Sense, Stroke, StrokeKind, TextEdit, Ui, Vec2,
};

use crate::theme;

// ---------------------------------------------------------------------------
// Containers
// ---------------------------------------------------------------------------

/// A card – a raised surface with a hairline border.
pub fn card<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Frame::new()
        .fill(theme::SURFACE)
        .stroke(Stroke::new(1.0, theme::BORDER))
        .corner_radius(theme::radius())
        .inner_margin(Margin::symmetric(20, 18))
        .outer_margin(Margin::same(0))
        .show(ui, add)
        .inner
}

/// An uppercase section label with a hairline rule underneath.
pub fn section(ui: &mut Ui, label: &str) {
    ui.label(theme::micro(label));
    ui.add_space(theme::SPACE_XS);
    rule(ui);
    ui.add_space(theme::SPACE_MD);
}

/// A page title with a one line explanation underneath.
pub fn page_header(ui: &mut Ui, title: &str, subtitle: &str) {
    ui.label(theme::title(title));
    ui.add_space(theme::SPACE_XS);
    ui.label(theme::body(subtitle));
    ui.add_space(theme::SPACE_LG);
}

/// A hairline separator.
pub fn rule(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, theme::BORDER);
}

/// A hairline separator with breathing room around it.
pub fn divider(ui: &mut Ui) {
    rule(ui);
    ui.add_space(theme::SPACE_MD);
}

// ---------------------------------------------------------------------------
// Data display
// ---------------------------------------------------------------------------

/// A single headline number with a label and a caption.
pub struct StatTile<'a> {
    /// The uppercase label above the number.
    pub label: &'a str,
    /// The headline value.
    pub value: String,
    /// The small line underneath.
    pub caption: String,
    /// The colour of the accent bar and the progress bar.
    pub accent: Color32,
    /// An optional progress bar between the value and the caption.
    pub progress: Option<f32>,
}

/// Draws a [`StatTile`].
pub fn stat_tile(ui: &mut Ui, tile: StatTile<'_>) {
    let response = egui::Frame::new()
        .fill(theme::SURFACE)
        .stroke(Stroke::new(1.0, theme::BORDER))
        .corner_radius(theme::radius())
        .inner_margin(Margin::symmetric(18, 16))
        .show(ui, |ui| {
            ui.label(theme::micro(tile.label));
            ui.add_space(theme::SPACE_SM);
            ui.label(theme::display(&tile.value));
            if let Some(fraction) = tile.progress {
                ui.add_space(theme::SPACE_MD);
                progress_bar(ui, fraction, tile.accent, 6.0);
            }
            ui.add_space(theme::SPACE_SM);
            ui.label(theme::caption(&tile.caption));
        });

    // The accent bar on the left edge of the card.
    let rect = response.response.rect;
    if ui.is_rect_visible(rect) {
        let bar = Rect::from_min_size(
            rect.min + Vec2::new(0.0, 14.0),
            Vec2::new(3.0, (rect.height() - 28.0).max(0.0)),
        );
        ui.painter()
            .rect_filled(bar, theme::radius_sm(), tile.accent);
    }
}

/// A thin rounded progress bar.
pub fn progress_bar(ui: &mut Ui, fraction: f32, color: Color32, height: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), height), Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }

    let painter = ui.painter();
    painter.rect_filled(rect, pill(rect.height()), theme::SURFACE_RAISED);

    let fraction = fraction.clamp(0.0, 1.0);
    if fraction <= 0.0 {
        return;
    }

    let fill = Rect::from_min_size(rect.min, Vec2::new(rect.width() * fraction, rect.height()));
    painter.rect_filled(fill, pill(rect.height()), color);
}

/// A small pill used for statuses and metadata.
pub fn chip(ui: &mut Ui, text: &str, color: Color32) {
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), FontId::proportional(11.5), color);
    let size = galley.size() + Vec2::new(20.0, 10.0);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }

    ui.painter()
        .rect_filled(rect, theme::RADIUS_PILL, theme::tint(color, 0.18));
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, color);
}

/// A small filled circle, used for status indicators.
pub fn dot(ui: &mut Ui, color: Color32, size: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.painter().circle_filled(rect.center(), size / 2.0, color);
    }
}

/// A label on the left and a value on the right.
pub fn kv_row(ui: &mut Ui, key: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(theme::caption(key));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(egui::Label::new(theme::mono(value)).truncate());
        });
    });
}

/// A compact metric for the side rail: a label, a value and a progress bar.
pub fn metric_row(ui: &mut Ui, label: &str, value: &str, color: Color32, progress: Option<f32>) {
    ui.horizontal(|ui| {
        ui.label(theme::caption(label));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(theme::metric(value));
        });
    });

    if let Some(fraction) = progress {
        ui.add_space(theme::SPACE_XS);
        progress_bar(ui, fraction, color, 5.0);
    }

    ui.add_space(theme::SPACE_MD);
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

/// The visual variants of the app's buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    /// Filled with the ember accent – the main call to action.
    Primary,
    /// A raised neutral surface.
    Secondary,
    /// Transparent, only the label is visible.
    Ghost,
    /// A destructive action.
    Danger,
    /// The PayPal brand button.
    PayPal,
}

impl ButtonStyle {
    /// The fill, the hover fill and the label colour of the variant.
    fn colors(self) -> (Color32, Color32, Color32) {
        match self {
            ButtonStyle::Primary => (theme::EMBER, theme::EMBER_HOVER, theme::TEXT_ON_EMBER),
            ButtonStyle::Secondary => (theme::SURFACE_RAISED, theme::SURFACE_HOVER, theme::TEXT),
            ButtonStyle::Ghost => (
                Color32::TRANSPARENT,
                theme::SURFACE_RAISED,
                theme::TEXT_MUTED,
            ),
            ButtonStyle::Danger => (
                theme::tint(theme::DANGER, 0.16),
                theme::tint(theme::DANGER, 0.26),
                theme::DANGER,
            ),
            ButtonStyle::PayPal => (theme::PAYPAL, theme::PAYPAL_HOVER, Color32::WHITE),
        }
    }
}

/// A button that is only as wide as its label.
pub fn button(ui: &mut Ui, label: &str, style: ButtonStyle) -> egui::Response {
    let (fill, hover_fill, text_color) = style.colors();
    paint_button(
        ui,
        label,
        None,
        fill,
        hover_fill,
        text_color,
        style == ButtonStyle::Primary,
    )
}

/// A button that spans the whole available width.
pub fn button_block(ui: &mut Ui, label: &str, style: ButtonStyle) -> egui::Response {
    let (fill, hover_fill, text_color) = style.colors();
    paint_button(
        ui,
        label,
        Some(ui.available_width()),
        fill,
        hover_fill,
        text_color,
        style == ButtonStyle::Primary,
    )
}

/// A button with an exact width – used where a row has to line up with
/// something else.
pub fn button_sized(ui: &mut Ui, label: &str, style: ButtonStyle, width: f32) -> egui::Response {
    let (fill, hover_fill, text_color) = style.colors();
    paint_button(
        ui,
        label,
        Some(width),
        fill,
        hover_fill,
        text_color,
        style == ButtonStyle::Primary,
    )
}

/// A tab of the top navigation bar.
pub fn nav_tab(ui: &mut Ui, label: &str, active: bool) -> egui::Response {
    if active {
        paint_button(
            ui,
            label,
            None,
            theme::EMBER,
            theme::EMBER_HOVER,
            theme::TEXT_ON_EMBER,
            true,
        )
    } else {
        paint_button(
            ui,
            label,
            None,
            Color32::TRANSPARENT,
            theme::SURFACE_RAISED,
            theme::TEXT_MUTED,
            false,
        )
    }
}

/// A read-only value with a copy button next to it.
///
/// The row always occupies exactly the full width of its container: the field
/// takes whatever is left over once the fixed width copy button and the gap
/// have been subtracted.
///
/// Returns `true` when the value was copied to the clipboard.
pub fn copy_field(ui: &mut Ui, value: &str) -> bool {
    /// The exact width of the copy button.
    const COPY_WIDTH: f32 = 62.0;
    /// The padding between the value and the edge of the field.
    const FIELD_PADDING: f32 = 9.0;
    /// The preferred and the smallest font size of the value.
    const SIZE_MAX: f32 = 11.0;
    const SIZE_MIN: f32 = 9.0;

    let mut copied = false;

    ui.horizontal(|ui| {
        // One gap between the field and the button – no more, no less.
        ui.spacing_mut().item_spacing.x = theme::SPACE_SM;

        let height = 34.0;
        let field_width = (ui.available_width() - COPY_WIDTH - theme::SPACE_SM).max(40.0);
        let text_width = (field_width - 2.0 * FIELD_PADDING).max(20.0);

        // Shrink the value until it fits, so long URLs never spill out of the
        // field and never push the copy button out of line.
        let mut size = SIZE_MAX;
        let mut galley = ui.painter().layout_no_wrap(
            value.to_owned(),
            FontId::new(size, FontFamily::Monospace),
            theme::TEXT_MUTED,
        );
        while galley.size().x > text_width && size > SIZE_MIN {
            size -= 0.5;
            galley = ui.painter().layout_no_wrap(
                value.to_owned(),
                FontId::new(size, FontFamily::Monospace),
                theme::TEXT_MUTED,
            );
        }

        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(field_width, height), Sense::hover());
        if ui.is_rect_visible(rect) {
            ui.painter()
                .rect_filled(rect, theme::radius_sm(), theme::SURFACE_RAISED);

            let clip = ui.painter().with_clip_rect(rect.shrink(FIELD_PADDING));
            clip.galley(
                rect.left_center() + Vec2::new(FIELD_PADDING, -galley.size().y / 2.0),
                galley,
                theme::TEXT_MUTED,
            );
        }
        response.on_hover_text(value);

        if button_sized(ui, "Copy", ButtonStyle::Secondary, COPY_WIDTH).clicked() {
            ui.ctx().copy_text(value.to_owned());
            copied = true;
        }
    });

    copied
}

/// Paints a button and returns its response.
#[allow(clippy::too_many_arguments)]
fn paint_button(
    ui: &mut Ui,
    label: &str,
    width: Option<f32>,
    fill: Color32,
    hover_fill: Color32,
    text_color: Color32,
    outlined: bool,
) -> egui::Response {
    let galley =
        ui.painter()
            .layout_no_wrap(label.to_owned(), FontId::proportional(13.5), text_color);

    let padding = Vec2::new(16.0, 9.0);
    let height = (galley.size().y + padding.y * 2.0).max(34.0);
    let width = width.unwrap_or(galley.size().x + padding.x * 2.0);

    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
    if !ui.is_rect_visible(rect) {
        return response;
    }

    let background = if response.hovered() { hover_fill } else { fill };

    let radius = theme::radius_sm();
    let painter = ui.painter();
    painter.rect_filled(rect, radius, background);

    if outlined {
        painter.rect_stroke(
            rect,
            radius,
            Stroke::new(1.0, theme::EMBER_DEEP),
            StrokeKind::Inside,
        );
    }

    if response.clicked() {
        painter.rect_filled(rect, radius, Color32::from_black_alpha(30));
    }

    painter.galley(
        rect.center() - galley.size() / 2.0,
        galley.clone(),
        text_color,
    );

    response
}

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------

/// A labelled single line text field.
pub fn text_field(ui: &mut Ui, label: &str, value: &mut String, hint: &str, password: bool) {
    if !label.is_empty() {
        ui.label(theme::field_label(label));
        ui.add_space(theme::SPACE_XS);
    }

    field(ui, |ui| {
        let mut edit = TextEdit::singleline(value)
            .desired_width(f32::INFINITY)
            .hint_text(hint)
            .text_color(theme::TEXT)
            .frame(egui::Frame::NONE);
        if password {
            edit = edit.password(true);
        }
        ui.add(edit)
    });
}

/// A labelled numeric field that is parsed as you type.
pub fn number_field(ui: &mut Ui, label: &str, value: &mut u64, hint: &str) {
    if !label.is_empty() {
        ui.label(theme::field_label(label));
        ui.add_space(theme::SPACE_XS);
    }

    field(ui, |ui| {
        let mut buffer = value.to_string();
        let response = ui.add(
            TextEdit::singleline(&mut buffer)
                .desired_width(f32::INFINITY)
                .hint_text(hint)
                .text_color(theme::TEXT)
                .frame(egui::Frame::NONE),
        );
        if response.changed() {
            if let Ok(parsed) = buffer.trim().parse::<u64>() {
                *value = parsed;
            }
        }
        response
    });
}

/// A labelled multi line text field.
pub fn multiline_field(ui: &mut Ui, label: &str, value: &mut String, hint: &str, rows: usize) {
    if !label.is_empty() {
        ui.label(theme::field_label(label));
        ui.add_space(theme::SPACE_XS);
    }

    field(ui, |ui| {
        ui.add(
            TextEdit::multiline(value)
                .desired_width(f32::INFINITY)
                .desired_rows(rows)
                .hint_text(hint)
                .text_color(theme::TEXT)
                .frame(egui::Frame::NONE),
        )
    });
}

/// The shared frame of every input: a raised surface plus a focus ring.
fn field(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> egui::Response) -> egui::Response {
    let margin = Margin::symmetric(12, 8);

    let mut inner = None;
    egui::Frame::new()
        .fill(theme::SURFACE_RAISED)
        .corner_radius(theme::radius_sm())
        .inner_margin(margin)
        .show(ui, |ui| {
            inner = Some(add(ui));
        });

    let response = inner.expect("the field always renders");
    let ring = response.rect.expand2(Vec2::new(12.0, 8.0));
    let stroke = if response.has_focus() {
        Stroke::new(1.5, theme::EMBER)
    } else {
        Stroke::new(1.0, theme::BORDER_STRONG)
    };
    ui.painter()
        .rect_stroke(ring, theme::radius_sm(), stroke, StrokeKind::Inside);

    response
}

// ---------------------------------------------------------------------------
// Feedback
// ---------------------------------------------------------------------------

/// A coloured banner that reports the outcome of the last action.
pub fn status_banner(ui: &mut Ui, color: Color32, text: &str) {
    let galley =
        ui.painter()
            .layout_no_wrap(text.to_owned(), FontId::proportional(13.5), theme::TEXT);

    let padding = Vec2::new(14.0, 11.0);
    let height = galley.size().y + padding.y * 2.0;
    let width = ui.available_width();

    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }

    let painter = ui.painter();
    painter.rect_filled(rect, theme::radius_sm(), theme::tint(color, 0.14));
    painter.rect_filled(
        Rect::from_min_size(rect.min, Vec2::new(3.0, rect.height())),
        theme::radius_sm(),
        color,
    );
    painter.galley(
        rect.left_center() + Vec2::new(14.0 - 6.0, -galley.size().y / 2.0),
        galley,
        theme::TEXT,
    );
}

/// The small "made with love" note.
pub fn branding_badge(ui: &mut Ui) {
    let galley = ui.painter().layout_no_wrap(
        "Made with \u{2764} from South Africa".to_owned(),
        FontId::proportional(12.0),
        theme::TEXT_FAINT,
    );
    let size = galley.size() + Vec2::new(18.0, 9.0);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.painter()
            .rect_filled(rect, theme::RADIUS_PILL, theme::SURFACE);
        ui.painter().galley(
            rect.center() - galley.size() / 2.0,
            galley,
            theme::TEXT_FAINT,
        );
    }
}

/// A corner radius that turns a rectangle into a pill.
fn pill(height: f32) -> egui::CornerRadius {
    egui::CornerRadius::from((height / 2.0).min(theme::RADIUS_PILL))
}
