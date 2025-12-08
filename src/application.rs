use std::process::Command;

use iced::clipboard;
use iced::window::settings::PlatformSpecific;
use iced::{Element, Font, Subscription, Task, Theme};

use libsftpman::Manager;

use crate::messages::Message;
use crate::pages::Home;
use crate::pages::HomeMessage;
use crate::ui_config::{APP_HEIGHT, APP_WIDTH};

const APPLICATION_ID: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone)]
pub enum ApplicationMessage {
    PutContentInClipboard(String),
    OpenLink(String),
}

struct Application {
    pages: Vec<Box<dyn Page>>,
}

impl Application {
    fn new() -> (Self, Task<Message>) {
        let manager = Manager::new().unwrap();

        let tasks = Task::batch([
            iced::widget::operation::focus_next(),
            Task::perform(async {}, |_| Message::Home(HomeMessage::RunPreflightCheck)),
        ]);

        (
            Self {
                pages: vec![Box::new(Home::new(manager.clone()))],
            },
            tasks,
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        if let Message::Application(message) = message {
            return match message {
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

    fn view(&self) -> Element<Message> {
        self.pages.last().unwrap().view(&self.theme())
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }

    fn subscription(&self) -> Subscription<Message> {
        self.pages.last().unwrap().subscription()
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
    fn view(&self, theme: &iced::theme::Theme) -> Element<Message>;
    fn subscription(&self) -> Subscription<Message>;
}
