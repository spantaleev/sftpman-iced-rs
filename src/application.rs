use std::process::Command;

use iced::clipboard;
use iced::theme::Mode as ThemeMode;
use iced::window::settings::PlatformSpecific;
use iced::{Element, Font, Subscription, Task, Theme};

use libsftpman::Manager;

use crate::messages::Message;
use crate::pages::Home;
use crate::pages::HomeMessage;
use crate::ui_config::{APP_HEIGHT, APP_WIDTH};

const APPLICATION_ID: &str = env!("CARGO_PKG_NAME");

/// The theme to use when we have nothing better to go by
/// (no `ICED_THEME` override and no color-scheme preference reported by the system).
const DEFAULT_THEME: Theme = Theme::Light;

#[derive(Debug, Clone)]
pub enum ApplicationMessage {
    PutContentInClipboard(String),
    OpenLink(String),
    SystemThemeModeChanged(ThemeMode),
}

struct Application {
    pages: Vec<Box<dyn Page>>,

    /// An explicit theme, as requested via the `ICED_THEME` environment variable.
    /// When set, it wins over whatever color scheme the system reports.
    theme_override: Option<Theme>,

    /// The theme we are currently rendering with.
    theme: Theme,
}

impl Application {
    fn new() -> (Self, Task<Message>) {
        let manager = Manager::new().unwrap();

        let theme_override = Self::theme_override_from_env();

        let mut tasks = vec![
            iced::widget::operation::focus_next(),
            Task::perform(async {}, |_| Message::Home(HomeMessage::RunPreflightCheck)),
        ];

        // Without an explicit override, we start out with the default theme and ask the system
        // about its color-scheme preference. The answer arrives as a message and may replace the theme.
        if theme_override.is_none() {
            tasks.push(iced::system::theme().map(|mode| {
                Message::Application(ApplicationMessage::SystemThemeModeChanged(mode))
            }));
        }

        (
            Self {
                pages: vec![Box::new(Home::new(manager.clone()))],
                theme: theme_override.clone().unwrap_or(DEFAULT_THEME),
                theme_override,
            },
            Task::batch(tasks),
        )
    }

    /// Returns the theme explicitly requested via the `ICED_THEME` environment variable, if any.
    fn theme_override_from_env() -> Option<Theme> {
        let Ok(env_theme) = std::env::var("ICED_THEME") else {
            log::debug!(
                "No theme specified in the ICED_THEME environment variable. Following the system color scheme (falling back to {:?}). Available themes: {:?}",
                DEFAULT_THEME,
                Theme::ALL
            );

            return None;
        };

        let found = Theme::ALL
            .iter()
            .find(|theme| format!("{:?}", theme).eq_ignore_ascii_case(&env_theme))
            .cloned();

        match found {
            Some(theme) => {
                log::info!(
                    "Using the {:?} theme, as requested via the ICED_THEME environment variable",
                    theme
                );

                Some(theme)
            }
            None => {
                log::warn!(
                    "Theme '{}' not found. Following the system color scheme instead (falling back to {:?}). Available themes: {:?}",
                    env_theme,
                    DEFAULT_THEME,
                    Theme::ALL
                );

                None
            }
        }
    }

    /// Adopts the color scheme (light/dark) that the system reports.
    ///
    /// Detection happens through the XDG desktop portal and requires
    /// the `system-theme-detection` feature to be enabled at build time.
    /// Without it, the system always reports [`ThemeMode::None`].
    fn apply_system_theme_mode(&mut self, mode: ThemeMode) {
        if self.theme_override.is_some() {
            // An explicit ICED_THEME value wins over whatever the system prefers.
            return;
        }

        let theme = match mode {
            ThemeMode::Light => Theme::Light,
            ThemeMode::Dark => Theme::Dark,
            ThemeMode::None => DEFAULT_THEME,
        };

        if theme == self.theme {
            return;
        }

        log::info!(
            "Switching to the {:?} theme, following the system color scheme ({:?})",
            theme,
            mode
        );

        self.theme = theme;
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        if let Message::Application(message) = message {
            return match message {
                ApplicationMessage::SystemThemeModeChanged(mode) => {
                    self.apply_system_theme_mode(mode);

                    Task::none()
                }
                ApplicationMessage::PutContentInClipboard(content) => clipboard::write(content),
                ApplicationMessage::OpenLink(link) => {
                    log::info!("Opening link: {}", link);

                    let mut cmd = Command::new("xdg-open");
                    cmd.arg(link);

                    let result = cmd.spawn();

                    match result {
                        Ok(_) => Task::none(),
                        Err(e) => {
                            log::error!("Failed to open link: {}", e);
                            Task::none()
                        }
                    }
                }
            };
        };

        let navigation = self.pages.last_mut().unwrap().update(message);
        match navigation {
            Navigation::GoTo(page, tasks) => {
                self.pages.push(page);
                tasks
            }
            Navigation::Back(tasks) => {
                if self.pages.len() > 1 {
                    self.pages.pop();
                }
                tasks
            }
            Navigation::None(tasks) => tasks,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        self.pages.last().unwrap().view(&self.theme())
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        let page_subscription = self.pages.last().unwrap().subscription();

        if self.theme_override.is_some() {
            return page_subscription;
        }

        // Keeps us in sync when the user toggles their desktop's dark mode while we're running.
        let system_theme_subscription = iced::system::theme_changes()
            .map(|mode| Message::Application(ApplicationMessage::SystemThemeModeChanged(mode)));

        Subscription::batch([page_subscription, system_theme_subscription])
    }
}

pub fn run_application() -> iced::Result {
    let window_settings_platform_specific = PlatformSpecific {
        application_id: APPLICATION_ID.to_string(),
        ..PlatformSpecific::default()
    };

    #[cfg(feature = "x11-icon")]
    let mut window_settings = iced::window::Settings {
        platform_specific: window_settings_platform_specific,
        ..iced::window::Settings::default()
    };

    #[cfg(not(feature = "x11-icon"))]
    let window_settings = iced::window::Settings {
        platform_specific: window_settings_platform_specific,
        ..iced::window::Settings::default()
    };

    // This is behind an x11-icon feature flag, because it won't work on Wayland anyway and needs to bring in iced/image as a dependency.
    //
    // On Wayland, the window setting's platform-specific application id needs to match either the `.desktop` launcher name or the `StartupWMClass` within it.
    // The `Icon` property of the `.desktop` file will be used for the application icon automatically.
    // On such systems, we don't need to set an icon manually and can avoid the iced/image dependency.
    //
    // See:
    // - https://github.com/iced-rs/iced/issues/1944
    // - https://github.com/GyulyVGC/sniffnet/issues/292#issuecomment-1691384896
    #[cfg(feature = "x11-icon")]
    {
        window_settings.icon = Some(
            iced::window::icon::from_file_data(crate::assets::APPLICATION_ICON, None)
                .expect("Failed to load icon"),
        );
    }

    let app = iced::application(Application::new, Application::update, Application::view)
        .title(Application::title)
        .subscription(Application::subscription)
        .window_size((APP_WIDTH, APP_HEIGHT))
        .window(window_settings)
        .theme(Application::theme)
        .default_font(Font::DEFAULT);

    app.run()
}

impl Application {
    fn title(&self) -> String {
        String::from(APPLICATION_ID)
    }
}

pub enum Navigation {
    GoTo(Box<dyn Page>, iced::Task<Message>),
    Back(iced::Task<Message>),
    None(iced::Task<Message>),
}

pub trait Page {
    fn update(&mut self, message: Message) -> Navigation;
    fn view(&self, theme: &iced::theme::Theme) -> Element<'_, Message>;
    fn subscription(&self) -> Subscription<Message>;
}
