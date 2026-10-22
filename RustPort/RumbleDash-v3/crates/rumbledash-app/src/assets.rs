//! Branding assets, embedded into the binary at compile time.

use egui::{Context, TextureHandle, TextureOptions};

/// The RumbleDash word mark.
pub const LOGO: &[u8] = include_bytes!("../assets/logo.png");
/// The artwork shown on the feedback page.
pub const FEEDBACK: &[u8] = include_bytes!("../assets/feedback.png");

/// RumbleBot affiliate banner.
pub const AD_RUMBLEBOT: &[u8] =
    include_bytes!("../assets/promotions/Affiliate Banners RumbleBot.png");
/// Nerd Or Die affiliate banner.
pub const AD_NOD: &[u8] = include_bytes!("../assets/promotions/Affiliate Banners NoD.png");
/// Rumble for Gaming Discord banner.
pub const AD_DISCORD: &[u8] =
    include_bytes!("../assets/promotions/Discord Ad Banner RumbleForGaming.png");
/// PayPal donation banner.
pub const AD_PAYPAL: &[u8] =
    include_bytes!("../assets/promotions/Paypal Donation Encouragement Banner.png");
/// GitHub repository banner.
pub const AD_GITHUB: &[u8] =
    include_bytes!("../assets/promotions/Github Repository RumbleDash.png");

/// An advertisement: a banner image plus the link it points at.
pub struct Advertisement {
    /// Destination opened when the banner is clicked.
    pub href: &'static str,
    /// Raw PNG bytes of the banner.
    pub bytes: &'static [u8],
    /// The decoded texture, filled in once the GUI starts.
    pub texture: Option<TextureHandle>,
}

/// The rotating advertisement banner of the v2 dashboard.
pub fn advertisements() -> Vec<Advertisement> {
    vec![
        Advertisement {
            href: "https://rumble.bot/",
            bytes: AD_RUMBLEBOT,
            texture: None,
        },
        Advertisement {
            href: "https://nerdordie.com/shop/ref/tinyplayerss/",
            bytes: AD_NOD,
            texture: None,
        },
        Advertisement {
            href: "https://discord.gg/nJywsXTaFY",
            bytes: AD_DISCORD,
            texture: None,
        },
        Advertisement {
            href: "https://paypal.me/2players1Gamer/",
            bytes: AD_PAYPAL,
            texture: None,
        },
        Advertisement {
            href: "https://github.com/CustomBAR/RumbleDash",
            bytes: AD_GITHUB,
            texture: None,
        },
    ]
}

/// Decodes PNG bytes into an egui texture.
pub fn load_texture(ctx: &Context, name: &str, bytes: &[u8]) -> Option<TextureHandle> {
    let decoded = image::load_from_memory(bytes).ok()?.to_rgba8();
    let size = [decoded.width() as usize, decoded.height() as usize];
    let pixels = decoded.into_raw();
    let image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture(name, image, TextureOptions::LINEAR))
}

/// The RumbleDash logo as a texture.
pub fn logo_texture(ctx: &Context) -> Option<TextureHandle> {
    load_texture(ctx, "rumbledash-logo", LOGO)
}

/// The feedback artwork as a texture.
pub fn feedback_texture(ctx: &Context) -> Option<TextureHandle> {
    load_texture(ctx, "rumbledash-feedback", FEEDBACK)
}
