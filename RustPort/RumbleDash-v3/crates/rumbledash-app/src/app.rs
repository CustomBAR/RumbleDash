//! The RumbleDash v3 desktop application.
//!
//! The window is split into three areas:
//!
//! * a **top bar** with the brand, the navigation and the live status,
//! * a **right rail** with the live snapshot, the widget URLs and the
//!   secondary actions,
//! * a **content area** that holds the page itself.

use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};

use egui::{Color32, TextureHandle};
use rumbledash_core::config::{Config, Goals};
use rumbledash_server::state::DashboardPayload;
use rumbledash_server::{AppState, ServerHandle};

use crate::assets::{self, Advertisement};
use crate::pages;
use crate::sys;
use crate::theme;
use crate::widgets;

/// How often the upstream Rumble API is polled.
const REFRESH_INTERVAL: Duration = Duration::from_secs(5);
/// How often the advertisement banner rotates.
const AD_INTERVAL: Duration = Duration::from_secs(30);
/// The width of the right rail, including its gutter.
const RAIL_WIDTH: f32 = 356.0;

/// The pages of the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    /// The configuration panel – the equivalent of the v2 dashboard.
    Dashboard,
    /// The feedback form.
    Feedback,
    /// The update history.
    Changelog,
    /// The contributor wall.
    Contributors,
}

impl Page {
    /// Every page, in navigation order.
    pub const ALL: [Page; 4] = [
        Page::Dashboard,
        Page::Feedback,
        Page::Changelog,
        Page::Contributors,
    ];

    /// The label shown in the top navigation.
    pub fn title(self) -> &'static str {
        match self {
            Page::Dashboard => "Dashboard",
            Page::Feedback => "Feedback",
            Page::Changelog => "Update History",
            Page::Contributors => "Contributors",
        }
    }

    /// The heading of the page itself.
    pub fn heading(self) -> &'static str {
        match self {
            Page::Dashboard => "Dashboard",
            Page::Feedback => "Send feedback",
            Page::Changelog => "Update history",
            Page::Contributors => "Contributors",
        }
    }

    /// The one line explanation underneath the heading.
    pub fn description(self) -> &'static str {
        match self {
            Page::Dashboard => {
                "Set your counters, connect RumbleDash to your stream and copy the widget \
                 URLs straight into OBS."
            }
            Page::Feedback => {
                "Found a bug or have an idea? Let the maintainers know – every message is \
                 read."
            }
            Page::Changelog => "Everything that changed in RumbleDash, newest first.",
            Page::Contributors => "The people who built and maintain RumbleDash.",
        }
    }
}

/// The editable copy of the configuration.
#[derive(Debug, Clone, Default)]
pub struct Draft {
    /// The Rumble API URL.
    pub api_url: String,
    /// The follower goal.
    pub followers_goal: u64,
    /// The subscriber goal.
    pub subscribers_goal: u64,
}

impl Draft {
    /// Builds a draft from a saved configuration.
    pub fn from_config(config: &Config) -> Self {
        Self {
            api_url: config.api_url.clone(),
            followers_goal: config.goals.followers,
            subscribers_goal: config.goals.subscribers,
        }
    }
}

/// The kind of a status message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
    /// A neutral note.
    Info,
    /// Everything went well.
    Success,
    /// Something went wrong.
    Error,
}

impl MessageKind {
    /// The colour the message is drawn in.
    pub fn color(self) -> egui::Color32 {
        match self {
            MessageKind::Info => theme::INFO,
            MessageKind::Success => theme::SUCCESS,
            MessageKind::Error => theme::DANGER,
        }
    }
}

/// A status message shown above the page content.
#[derive(Debug, Clone)]
pub struct Message {
    /// How the message should be coloured.
    pub kind: MessageKind,
    /// The text of the message.
    pub text: String,
}

/// The feedback form state.
#[derive(Debug, Clone, Default)]
pub struct FeedbackForm {
    /// The sender's name.
    pub name: String,
    /// The sender's e-mail address.
    pub email: String,
    /// The feedback itself.
    pub message: String,
}

/// Messages sent from the background tasks to the GUI.
enum Event {
    /// A new dashboard payload (or an error message).
    Dashboard(Result<DashboardPayload, String>),
    /// The configuration was written to disk.
    Saved(Result<(), String>),
    /// The feedback form was submitted.
    Feedback(Result<(), String>),
    /// A contributor logo finished downloading.
    ContributorLogo(Vec<u8>),
}

/// The RumbleDash desktop application.
pub struct RumbleDashApp {
    /// Shared state with the widget server.
    server: AppState,
    /// Handle used to stop the widget server on exit.
    handle: Option<ServerHandle>,
    /// Runtime used for the background tasks.
    runtime: tokio::runtime::Runtime,
    /// Channel from the background tasks to the GUI.
    tx: Sender<Event>,
    rx: Receiver<Event>,

    /// The page that is currently shown.
    pub(crate) page: Page,
    /// The editable configuration.
    pub(crate) draft: Draft,
    /// The last known dashboard payload.
    pub(crate) stats: Option<DashboardPayload>,
    /// The last status message.
    pub(crate) message: Option<Message>,
    /// The rotating advertisements.
    pub(crate) ads: Vec<Advertisement>,
    /// Index of the advertisement currently shown.
    pub(crate) ad_index: usize,
    /// When the advertisement was last rotated.
    ad_rotated: Instant,
    /// The RumbleDash logo.
    pub(crate) logo: Option<TextureHandle>,
    /// The artwork of the feedback page.
    pub(crate) feedback_art: Option<TextureHandle>,
    /// The logo of the contributor whose channel is shown.
    pub(crate) contributor_logo: Option<TextureHandle>,
    /// The feedback form.
    pub(crate) feedback: FeedbackForm,
    /// When the dashboard was last refreshed.
    last_refresh: Instant,
    /// When the last numbers arrived.
    last_update: Option<Instant>,
    /// Whether a refresh is currently in flight.
    refreshing: bool,
    /// Whether the Rumble API URL is shown in clear text.
    reveal_api_url: bool,
}

impl RumbleDashApp {
    /// Creates the application and loads the persisted configuration.
    pub fn new(
        creation: &eframe::CreationContext<'_>,
        server: AppState,
        handle: ServerHandle,
    ) -> Self {
        theme::install_fonts(&creation.egui_ctx);
        theme::apply(&creation.egui_ctx);

        let config = server
            .config
            .try_read()
            .map(|guard| guard.clone())
            .unwrap_or_default();

        let (tx, rx) = channel();
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap_or_else(|_| tokio::runtime::Runtime::new().expect("tokio runtime"));

        let mut ads = assets::advertisements();
        for (index, ad) in ads.iter_mut().enumerate() {
            ad.texture = assets::load_texture(&creation.egui_ctx, &format!("ad-{index}"), ad.bytes);
        }

        let mut app = Self {
            server,
            handle: Some(handle),
            runtime,
            tx,
            rx,
            page: Page::Dashboard,
            draft: Draft::from_config(&config),
            stats: None,
            message: None,
            ads,
            ad_index: 0,
            ad_rotated: Instant::now(),
            logo: assets::logo_texture(&creation.egui_ctx),
            feedback_art: assets::feedback_texture(&creation.egui_ctx),
            contributor_logo: None,
            feedback: FeedbackForm::default(),
            last_refresh: Instant::now() - REFRESH_INTERVAL,
            last_update: None,
            refreshing: false,
            reveal_api_url: false,
        };

        app.request_dashboard();
        app.request_contributor_logo();
        app.set_message(
            MessageKind::Info,
            "Add your Rumble API URL to start tracking your counters.",
        );
        app
    }

    // -- Addresses ---------------------------------------------------------

    /// The base URL that OBS should point at.
    pub fn base_url(&self) -> String {
        match &self.handle {
            Some(handle) => format!("http://{}:{}", handle.lan_ip, handle.addr.port()),
            None => format!("http://127.0.0.1:{}", self.server.port),
        }
    }

    /// The URL of the followers widget.
    pub fn followers_url(&self) -> String {
        format!("{}/followers", self.base_url())
    }

    /// The URL of the subscribers widget.
    pub fn subscribers_url(&self) -> String {
        format!("{}/subscribers", self.base_url())
    }

    /// The folder that holds `config.json`.
    pub fn config_folder(&self) -> std::path::PathBuf {
        self.server
            .config_path
            .parent()
            .map(std::path::Path::to_path_buf)
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| std::path::PathBuf::from("."))
    }

    /// The number of followers last reported by Rumble.
    pub fn followers(&self) -> u64 {
        self.stats
            .as_ref()
            .map(|stats| stats.followers)
            .unwrap_or(0)
    }

    /// The number of subscribers last reported by Rumble.
    pub fn subscribers(&self) -> u64 {
        self.stats
            .as_ref()
            .map(|stats| stats.subscribers)
            .unwrap_or(0)
    }

    /// Whether the app has received numbers at least once.
    pub fn is_live(&self) -> bool {
        self.stats.is_some()
    }

    /// How long ago the numbers were refreshed, in seconds.
    pub fn seconds_since_update(&self) -> Option<u64> {
        self.last_update.map(|instant| instant.elapsed().as_secs())
    }

    // -- Background work ---------------------------------------------------

    /// Asks the server for fresh numbers.
    fn request_dashboard(&mut self) {
        if self.refreshing {
            return;
        }
        self.refreshing = true;

        let server = self.server.clone();
        let tx = self.tx.clone();
        self.runtime.spawn(async move {
            let result = server.dashboard().await.map_err(|error| error.to_string());
            let _ = tx.send(Event::Dashboard(result));
        });
    }

    /// Downloads the contributor channel logo in the background.
    fn request_contributor_logo(&mut self) {
        let url = pages::contributors::CUSTOMER_LOGO_URL;
        let tx = self.tx.clone();
        self.runtime.spawn(async move {
            let client = reqwest::Client::new();
            if let Ok(response) = client.get(url).send().await {
                if let Ok(bytes) = response.bytes().await {
                    let _ = tx.send(Event::ContributorLogo(bytes.to_vec()));
                }
            }
        });
    }

    /// Handles everything the background tasks produced.
    fn drain_events(&mut self, ctx: &egui::Context) {
        while let Ok(event) = self.rx.try_recv() {
            match event {
                Event::Dashboard(Ok(payload)) => {
                    self.refreshing = false;
                    self.stats = Some(payload);
                    self.last_update = Some(Instant::now());
                }
                Event::Dashboard(Err(error)) => {
                    self.refreshing = false;
                    self.set_message(MessageKind::Error, error);
                }
                Event::Saved(Ok(())) => {
                    self.set_message(MessageKind::Success, "Configuration saved.");
                }
                Event::Saved(Err(error)) => {
                    self.set_message(
                        MessageKind::Error,
                        format!("Error updating configuration: {error}"),
                    );
                }
                Event::Feedback(Ok(())) => {
                    self.set_message(MessageKind::Success, "Thank you for your feedback!");
                    self.feedback = FeedbackForm::default();
                }
                Event::Feedback(Err(error)) => {
                    self.set_message(
                        MessageKind::Error,
                        format!("Could not send feedback: {error}"),
                    );
                }
                Event::ContributorLogo(bytes) => {
                    self.contributor_logo = assets::load_texture(ctx, "contributor-logo", &bytes);
                }
            }
        }
    }

    /// Stores a status message.
    pub fn set_message(&mut self, kind: MessageKind, text: impl Into<String>) {
        self.message = Some(Message {
            kind,
            text: text.into(),
        });
    }

    /// The colour of the current status message.
    pub(crate) fn message_color(&self) -> Option<egui::Color32> {
        self.message.as_ref().map(|message| message.kind.color())
    }

    // -- Actions -----------------------------------------------------------

    /// Saves the current draft to disk and to the shared server state.
    pub fn save_config(&mut self) {
        let config = Config {
            api_url: self.draft.api_url.trim().to_string(),
            goals: Goals {
                followers: self.draft.followers_goal,
                subscribers: self.draft.subscribers_goal,
            },
        };

        let server = self.server.clone();
        let tx = self.tx.clone();
        let path = server.config_path.clone();

        self.runtime.spawn(async move {
            {
                let mut guard = server.config.write().await;
                *guard = config.clone();
            }
            server.cache.invalidate();

            let result = config
                .save(path.as_ref())
                .map_err(|error| error.to_string());
            let _ = tx.send(Event::Saved(result));
        });
    }

    /// Removes the stored Rumble API URL.
    pub fn clear_api_url(&mut self) {
        self.draft.api_url.clear();

        let server = self.server.clone();
        let tx = self.tx.clone();
        let path = server.config_path.clone();

        self.runtime.spawn(async move {
            {
                let mut guard = server.config.write().await;
                guard.api_url.clear();
            }
            server.cache.invalidate();

            let result = Config {
                api_url: String::new(),
                goals: server.config.read().await.goals,
            }
            .save(path.as_ref())
            .map_err(|error| error.to_string());

            let _ = tx.send(Event::Saved(result));
        });
    }

    /// Sends the feedback form to Formspree.
    pub fn send_feedback(&mut self) {
        if self.feedback.name.trim().is_empty()
            || self.feedback.email.trim().is_empty()
            || self.feedback.message.trim().is_empty()
        {
            self.set_message(
                MessageKind::Error,
                "Please fill in your name, e-mail address and message.",
            );
            return;
        }

        let form = self.feedback.clone();
        let tx = self.tx.clone();

        self.runtime.spawn(async move {
            let client = reqwest::Client::new();
            let result = client
                .post("https://formspree.io/f/xeoqkoqa")
                .form(&[
                    ("name", form.name.as_str()),
                    ("email", form.email.as_str()),
                    ("message", form.message.as_str()),
                ])
                .send()
                .await
                .map_err(|error| error.to_string())
                .and_then(|response| {
                    if response.status().is_success() {
                        Ok(())
                    } else {
                        Err(format!("the server replied with {}", response.status()))
                    }
                });

            let _ = tx.send(Event::Feedback(result));
        });
    }

    /// Rotates the advertisement banner.
    fn rotate_ads(&mut self) {
        if self.ad_rotated.elapsed() < AD_INTERVAL || self.ads.is_empty() {
            return;
        }
        self.ad_rotated = Instant::now();
        self.ad_index = (self.ad_index + 1) % self.ads.len();
    }

    // -- Layout ------------------------------------------------------------

    /// Draws the top bar: brand, navigation and the live status.
    fn top_bar(&mut self, ui: &mut egui::Ui) {
        let logo = self.logo.clone();
        let page = self.page;
        let live = self.is_live();
        let age = self.seconds_since_update();
        let mut target: Option<Page> = None;

        ui.horizontal(|ui| {
            if let Some(logo) = logo {
                ui.add(egui::Image::from_texture(&logo).fit_to_exact_size(egui::vec2(148.0, 30.0)));
            } else {
                ui.label(theme::title("RumbleDash"));
            }

            ui.add_space(theme::SPACE_MD);
            widgets::chip(
                ui,
                &format!("v{}", rumbledash_core::VERSION),
                theme::TEXT_FAINT,
            );

            ui.add_space(theme::SPACE_XL);

            for candidate in Page::ALL {
                if widgets::nav_tab(ui, candidate.title(), page == candidate).clicked() {
                    target = Some(candidate);
                }
                ui.add_space(theme::SPACE_XS);
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let (color, label) = if live {
                    (
                        theme::SUCCESS,
                        match age {
                            Some(0) => "Live".to_owned(),
                            Some(seconds) => format!("Live \u{b7} {seconds}s ago"),
                            None => "Live".to_owned(),
                        },
                    )
                } else {
                    (theme::TEXT_FAINT, "Waiting for data".to_owned())
                };

                widgets::chip(ui, &label, color);
                if live {
                    widgets::dot(ui, theme::SUCCESS, 8.0);
                }
            });
        });

        if let Some(target) = target {
            self.page = target;
        }
    }

    /// Draws the right rail: live snapshot, widget URLs and secondary actions.
    fn rail(&mut self, ui: &mut egui::Ui) {
        let followers = self.followers();
        let subscribers = self.subscribers();
        let followers_goal = self.draft.followers_goal;
        let subscribers_goal = self.draft.subscribers_goal;
        let live = self.is_live();

        let followers_url = self.followers_url();
        let subscribers_url = self.subscribers_url();
        let base_url = self.base_url();
        let config_folder = self.config_folder();
        let local_url = format!("http://127.0.0.1:{}/", self.server.port);

        ui.add_space(theme::SPACE_LG);

        widgets::section(ui, "Live snapshot");
        widgets::metric_row(
            ui,
            "Followers",
            &followers.to_string(),
            theme::EMBER,
            Some(ratio(followers, followers_goal)),
        );
        widgets::metric_row(
            ui,
            "Subscribers",
            &subscribers.to_string(),
            theme::EMBER_LIGHT,
            Some(ratio(subscribers, subscribers_goal)),
        );
        if live {
            ui.label(theme::accent("Connected to the Rumble API"));
        } else {
            ui.label(theme::caption(
                "Waiting for the Rumble API to answer\u{2026}",
            ));
        }
        ui.add_space(theme::SPACE_MD);

        widgets::section(ui, "Widget URLs");
        ui.label(theme::caption(
            "Add one of these as a browser source in OBS.",
        ));
        ui.add_space(theme::SPACE_MD);
        widgets::copy_field(ui, &followers_url);
        ui.add_space(theme::SPACE_SM);
        widgets::copy_field(ui, &subscribers_url);

        widgets::section(ui, "Tools");
        if widgets::button_block(
            ui,
            "Open dashboard in browser",
            widgets::ButtonStyle::Secondary,
        )
        .clicked()
        {
            sys::open_url(&local_url);
        }
        ui.add_space(theme::SPACE_SM);
        if widgets::button_block(ui, "Open config folder", widgets::ButtonStyle::Secondary)
            .clicked()
        {
            sys::open_folder(&config_folder);
        }

        widgets::section(ui, "Support");
        if widgets::button_block(ui, "Donate via PayPal", widgets::ButtonStyle::PayPal).clicked() {
            sys::open_url("https://www.paypal.com/donate?hosted_button_id=4J4ZYEGVSMNJN");
        }

        ui.add_space(theme::SPACE_LG);
        widgets::divider(ui);
        widgets::kv_row(ui, "Server", &base_url);
        ui.add_space(theme::SPACE_SM);
        widgets::kv_row(ui, "Config", "config.json");
        ui.label(theme::caption(config_folder.display().to_string()));
        ui.add_space(theme::SPACE_LG);
    }

    /// Draws the rotating advertisement banner.
    pub fn ad_banner(&mut self, ui: &mut egui::Ui) {
        if self.ads.is_empty() {
            return;
        }

        let index = self.ad_index % self.ads.len();
        let Some(ad) = self.ads.get(index) else {
            return;
        };
        let Some(texture) = ad.texture.clone() else {
            return;
        };
        let href = ad.href;

        let width = ui.available_width();
        let height = 132.0;
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());

        if !ui.is_rect_visible(rect) {
            return;
        }

        let painter = ui.painter();
        painter.rect_filled(rect, theme::radius(), theme::SURFACE);
        painter.rect_stroke(
            rect,
            theme::radius(),
            egui::Stroke::new(1.0, theme::BORDER),
            egui::StrokeKind::Inside,
        );

        // Keep the aspect ratio of the banner.
        let texture_size = egui::load::SizedTexture::from_handle(&texture).size;
        if texture_size.x > 0.0 && texture_size.y > 0.0 {
            let scale = (rect.width() / texture_size.x).min(rect.height() / texture_size.y);
            let draw_size = egui::vec2(texture_size.x * scale, texture_size.y * scale);
            let draw_rect = egui::Rect::from_center_size(rect.center(), draw_size);
            painter.image(
                texture.id(),
                draw_rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        }

        // The little "AD" badge in the corner.
        let badge =
            egui::Rect::from_min_size(rect.min + egui::vec2(12.0, 12.0), egui::vec2(34.0, 20.0));
        painter.rect_filled(
            badge,
            theme::RADIUS_PILL,
            theme::tint(theme::TEXT_FAINT, 0.55),
        );
        painter.text(
            badge.center(),
            egui::Align2::CENTER_CENTER,
            "AD",
            egui::FontId::proportional(10.5),
            theme::TEXT,
        );

        if response.clicked() {
            sys::open_url(href);
        }
    }

    /// Toggles whether the Rumble API URL is shown in clear text.
    pub fn toggle_api_url_visibility(&mut self) {
        self.reveal_api_url = !self.reveal_api_url;
    }

    /// Whether the Rumble API URL is currently revealed.
    pub fn api_url_is_visible(&self) -> bool {
        self.reveal_api_url
    }
}

impl eframe::App for RumbleDashApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        self.drain_events(&ctx);
        self.rotate_ads();

        if self.last_refresh.elapsed() >= REFRESH_INTERVAL {
            self.last_refresh = Instant::now();
            self.request_dashboard();
        }

        egui::Panel::top("rumbledash-topbar")
            .exact_size(62.0)
            .resizable(false)
            .show_separator_line(false)
            .frame(
                egui::Frame::new()
                    .fill(theme::SURFACE)
                    .inner_margin(egui::Margin::symmetric(theme::GUTTER, 0))
                    .stroke(egui::Stroke::new(1.0, theme::BORDER)),
            )
            .show(ui, |ui| self.top_bar(ui));

        egui::Panel::right("rumbledash-rail")
            .exact_size(RAIL_WIDTH)
            .resizable(false)
            .show_separator_line(false)
            .frame(
                egui::Frame::new()
                    .fill(theme::BACKDROP)
                    .inner_margin(egui::Margin::symmetric(theme::GUTTER, 0)),
            )
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("rail-scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| self.rail(ui));
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(theme::BACKDROP)
                    .inner_margin(egui::Margin::symmetric(theme::GUTTER, 0)),
            )
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("page-scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add_space(theme::SPACE_XL);
                        match self.page {
                            Page::Dashboard => pages::dashboard::dashboard(self, ui),
                            Page::Feedback => pages::feedback::feedback(self, ui),
                            Page::Changelog => pages::changelog::changelog(ui),
                            Page::Contributors => pages::contributors::contributors(self, ui),
                        }
                    });
            });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Some(handle) = self.handle.take() {
            handle.shutdown();
        }
    }
}

/// `current` as a fraction of `goal`, clamped to `0.0..=1.0`.
pub fn ratio(current: u64, goal: u64) -> f32 {
    if goal == 0 {
        0.0
    } else {
        (current as f32 / goal as f32).clamp(0.0, 1.0)
    }
}
