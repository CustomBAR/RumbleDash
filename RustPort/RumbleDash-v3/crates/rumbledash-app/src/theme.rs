//! Design tokens for the RumbleDash desktop application.
//!
//! The palette is deliberately narrow: a cool, low-chroma **charcoal** base and
//! exactly one warm accent – **ember orange**. Everything else (surfaces,
//! borders, status colours) is derived from those two ideas so that the
//! interface reads as one coherent system instead of a pile of coloured boxes.
//!
//! The scales (spacing, radii, type) are intentionally tiny and explicit – a
//! 4 px spacing grid, three corner radii and six type sizes. Layout code should
//! only ever pick from these, which is what keeps the app looking deliberate
//! rather than improvised.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use egui::{Color32, CornerRadius, FontFamily, FontId, Margin, RichText, Stroke};

/// The name of the font family that holds the bold face.
const BOLD_FAMILY: &str = "rumbledash-bold";
/// The name the bold font is registered under.
const BOLD_FONT_NAME: &str = "rumbledash-bold-face";

/// Whether a bold system font could be registered.
static BOLD_AVAILABLE: AtomicBool = AtomicBool::new(false);

// ---------------------------------------------------------------------------
// Surfaces
// ---------------------------------------------------------------------------

/// The window background – the darkest surface in the app.
pub const BACKDROP: Color32 = Color32::from_rgb(0x13, 0x14, 0x17);
/// Cards, panels and the top bar.
pub const SURFACE: Color32 = Color32::from_rgb(0x1c, 0x1e, 0x23);
/// Raised elements: inputs, chips, hover rows.
pub const SURFACE_RAISED: Color32 = Color32::from_rgb(0x25, 0x28, 0x2e);
/// Hover state of raised elements.
pub const SURFACE_HOVER: Color32 = Color32::from_rgb(0x31, 0x34, 0x3c);
/// Hairline borders.
pub const BORDER: Color32 = Color32::from_rgb(0x2f, 0x32, 0x3a);
/// Borders of focused or active elements.
pub const BORDER_STRONG: Color32 = Color32::from_rgb(0x43, 0x47, 0x51);

// ---------------------------------------------------------------------------
// Ember – the single accent
// ---------------------------------------------------------------------------

/// The primary accent.
pub const EMBER: Color32 = Color32::from_rgb(0xff, 0x6b, 0x1a);
/// Hover state of the primary accent.
pub const EMBER_HOVER: Color32 = Color32::from_rgb(0xff, 0x83, 0x3d);
/// Pressed state and the tail of the ember gradient.
pub const EMBER_DEEP: Color32 = Color32::from_rgb(0xc9, 0x44, 0x06);
/// The head of the ember gradient and text on tinted surfaces.
pub const EMBER_LIGHT: Color32 = Color32::from_rgb(0xff, 0xa3, 0x66);
/// A dark ember wash – used behind active rows and informational banners.
pub const EMBER_WASH: Color32 = Color32::from_rgb(0x33, 0x1f, 0x12);

// ---------------------------------------------------------------------------
// Text
// ---------------------------------------------------------------------------

/// Primary text.
pub const TEXT: Color32 = Color32::from_rgb(0xf3, 0xf4, 0xf6);
/// Secondary text.
pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x9a, 0xa0, 0xaa);
/// Tertiary text – hints, timestamps, footnotes.
pub const TEXT_FAINT: Color32 = Color32::from_rgb(0x67, 0x6d, 0x77);
/// Text placed on top of the ember accent.
pub const TEXT_ON_EMBER: Color32 = Color32::from_rgb(0x14, 0x0c, 0x06);

// ---------------------------------------------------------------------------
// Status
// ---------------------------------------------------------------------------

/// Positive status.
pub const SUCCESS: Color32 = Color32::from_rgb(0x4c, 0xc3, 0x8b);
/// Negative status.
pub const DANGER: Color32 = Color32::from_rgb(0xe5, 0x48, 0x4d);
/// Neutral status.
pub const INFO: Color32 = Color32::from_rgb(0x6a, 0xa6, 0xf0);

/// PayPal blue – the one place a second brand colour is allowed.
pub const PAYPAL: Color32 = Color32::from_rgb(0x00, 0x70, 0xba);
/// PayPal blue on hover.
pub const PAYPAL_HOVER: Color32 = Color32::from_rgb(0x08, 0x5c, 0x9b);

// ---------------------------------------------------------------------------
// Radii
// ---------------------------------------------------------------------------

/// Corner radius of cards and panels.
pub const RADIUS: f32 = 14.0;
/// Corner radius of buttons, inputs and chips.
pub const RADIUS_SM: f32 = 9.0;
/// Corner radius of fully rounded elements.
pub const RADIUS_PILL: f32 = 999.0;

/// The card radius as a `CornerRadius`.
pub fn radius() -> CornerRadius {
    CornerRadius::from(RADIUS)
}

/// The control radius as a `CornerRadius`.
pub fn radius_sm() -> CornerRadius {
    CornerRadius::from(RADIUS_SM)
}

/// A dark, opaque tint of `color` – used behind chips, banners and badges.
pub fn tint(color: Color32, strength: f32) -> Color32 {
    let mix = |channel: u8| (channel as f32 * strength).round() as u8;
    Color32::from_rgb(mix(color.r()), mix(color.g()), mix(color.b()))
}

// ---------------------------------------------------------------------------
// Spacing – a 4 px grid
// ---------------------------------------------------------------------------

/// 4 px – hairline gaps.
pub const SPACE_XS: f32 = 4.0;
/// 8 px – between related items.
pub const SPACE_SM: f32 = 8.0;
/// 12 px – between grouped items.
pub const SPACE_MD: f32 = 12.0;
/// 20 px – between blocks.
pub const SPACE_LG: f32 = 20.0;
/// 32 px – between sections.
pub const SPACE_XL: f32 = 32.0;

/// 24 px – the inset every panel keeps between its content and its edge.
///
/// The top bar, the central column and the right rail all use this value, so
/// every column of the interface starts and ends on exactly the same vertical
/// line. It is an `i8` because egui expresses frame margins that way.
pub const GUTTER: i8 = 24;

// ---------------------------------------------------------------------------
// Typography
// ---------------------------------------------------------------------------

/// The family used for headings, labels and numbers.
///
/// egui has no concept of a bold weight, so a real bold face is picked up from
/// the operating system and registered as its own family. If none can be found
/// the regular proportional family is used and the design still holds up –
/// hierarchy is carried by size, colour and spacing as well.
pub fn bold_family() -> FontFamily {
    if BOLD_AVAILABLE.load(Ordering::Relaxed) {
        FontFamily::Name(BOLD_FAMILY.into())
    } else {
        FontFamily::Proportional
    }
}

/// A font id in the heading family.
pub fn bold(size: f32) -> FontId {
    FontId::new(size, bold_family())
}

/// The big number of a stat tile.
pub fn display(text: impl Into<String>) -> RichText {
    RichText::new(text).font(bold(30.0)).color(TEXT)
}

/// A page title.
pub fn title(text: impl Into<String>) -> RichText {
    RichText::new(text).font(bold(23.0)).color(TEXT)
}

/// A card heading.
pub fn subtitle(text: impl Into<String>) -> RichText {
    RichText::new(text).font(bold(15.5)).color(TEXT)
}

/// A metric value – the number next to a label in the side rail.
pub fn metric(text: impl Into<String>) -> RichText {
    RichText::new(text).font(bold(19.0)).color(TEXT)
}

/// Body copy.
pub fn body(text: impl Into<String>) -> RichText {
    RichText::new(text).size(14.0).color(TEXT_MUTED)
}

/// A caption or helper line.
pub fn caption(text: impl Into<String>) -> RichText {
    RichText::new(text).size(12.5).color(TEXT_MUTED)
}

/// An uppercase section label.
pub fn micro(text: impl Into<String>) -> RichText {
    RichText::new(text.into().to_uppercase())
        .font(bold(10.5))
        .color(TEXT_FAINT)
}

/// A form label.
pub fn field_label(text: impl Into<String>) -> RichText {
    RichText::new(text).font(bold(12.0)).color(TEXT_MUTED)
}

/// A clickable link.
pub fn link(text: impl Into<String>) -> RichText {
    RichText::new(text).size(13.5).color(EMBER)
}

/// Accented text.
pub fn accent(text: impl Into<String>) -> RichText {
    RichText::new(text).font(bold(14.0)).color(EMBER)
}

/// A monospaced value – used for URLs and paths.
pub fn mono(text: impl Into<String>) -> RichText {
    RichText::new(text)
        .font(FontId::new(12.5, FontFamily::Monospace))
        .color(TEXT_MUTED)
}

// ---------------------------------------------------------------------------
// Fonts
// ---------------------------------------------------------------------------

/// Registers a bold system font so headings can carry real weight.
///
/// egui renders every glyph with the same weight, so a bold face has to be
/// supplied explicitly. A handful of well known system fonts are probed and
/// the first one that exists wins; when none is found the app silently falls
/// back to the bundled proportional font.
pub fn install_fonts(ctx: &egui::Context) {
    for path in bold_font_candidates() {
        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };

        ctx.add_font(egui::FontInsert {
            name: BOLD_FONT_NAME.to_owned(),
            data: egui::FontData::from_owned(bytes),
            families: vec![
                egui::InsertFontFamily {
                    family: FontFamily::Name(BOLD_FAMILY.into()),
                    priority: egui::FontPriority::Highest,
                },
                // Also usable as a last resort for regular text.
                egui::InsertFontFamily {
                    family: FontFamily::Proportional,
                    priority: egui::FontPriority::Lowest,
                },
            ],
        });

        BOLD_AVAILABLE.store(true, Ordering::Relaxed);
        break;
    }
}

/// The system fonts that are probed for a bold face, in order of preference.
fn bold_font_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(windir) = std::env::var("WINDIR") {
        for name in [
            "segoeuib.ttf",
            "arialbd.ttf",
            "tahomabd.ttf",
            "calibrib.ttf",
            "verdanab.ttf",
        ] {
            paths.push(PathBuf::from(&windir).join("Fonts").join(name));
        }
    }

    for path in [
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf",
        "/usr/share/fonts/TTF/DejaVuSans-Bold.ttf",
        "/System/Library/Fonts/Supplemental/Arial Bold.ttf",
    ] {
        paths.push(PathBuf::from(path));
    }

    paths
}

// ---------------------------------------------------------------------------
// Global styling
// ---------------------------------------------------------------------------

/// Applies the RumbleDash look and feel to the egui context.
pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);

    let mut style = (*ctx.style_of(egui::Theme::Dark)).clone();

    style.visuals.window_fill = BACKDROP;
    style.visuals.panel_fill = BACKDROP;
    style.visuals.extreme_bg_color = BACKDROP;
    style.visuals.faint_bg_color = SURFACE;
    style.visuals.hyperlink_color = EMBER;
    style.visuals.selection.bg_fill = EMBER_WASH;
    style.visuals.selection.stroke = Stroke::new(1.0, EMBER);
    style.visuals.override_text_color = None;

    // Inputs are flat raised surfaces – the focus ring is drawn by the field
    // widget itself so it can be ember tinted.
    style.visuals.widgets.inactive.bg_fill = SURFACE_RAISED;
    style.visuals.widgets.inactive.weak_bg_fill = SURFACE_RAISED;
    style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
    style.visuals.widgets.inactive.corner_radius = radius_sm();

    style.visuals.widgets.hovered.bg_fill = SURFACE_HOVER;
    style.visuals.widgets.hovered.weak_bg_fill = SURFACE_HOVER;
    style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    style.visuals.widgets.hovered.corner_radius = radius_sm();

    style.visuals.widgets.active.bg_fill = SURFACE_HOVER;
    style.visuals.widgets.active.weak_bg_fill = SURFACE_HOVER;
    style.visuals.widgets.active.bg_stroke = Stroke::NONE;
    style.visuals.widgets.active.corner_radius = radius_sm();

    style.visuals.widgets.open.bg_fill = SURFACE_RAISED;
    style.visuals.widgets.open.bg_stroke = Stroke::NONE;
    style.visuals.widgets.open.corner_radius = radius_sm();

    style.spacing.item_spacing = egui::vec2(SPACE_SM, SPACE_SM);
    style.spacing.button_padding = egui::vec2(14.0, 9.0);
    style.spacing.window_margin = Margin::same(16);
    style.spacing.interact_size.y = 30.0;
    style.spacing.scroll.bar_width = 8.0;
    style.spacing.scroll.floating = true;

    style
        .text_styles
        .insert(egui::TextStyle::Heading, bold(23.0));
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(14.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(13.5, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(12.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(12.5, egui::FontFamily::Monospace),
    );

    ctx.set_style_of(egui::Theme::Dark, style);
}
