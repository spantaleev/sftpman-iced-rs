mod application;
mod assets;
mod locale;
mod messages;
mod pages;
mod strings;
mod ui_config;
mod utils;
mod widgets;

#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

use crate::application::run_application;
use crate::locale::detect_locale;

pub fn main() -> iced::Result {
    let mut builder = env_logger::Builder::new();

    let logging_directive = match std::env::var("RUST_LOG") {
        Ok(rust_log) => rust_log,
        Err(_) => format!("warn,{}=info,libsftpman=info", env!("CARGO_CRATE_NAME")),
    };

    builder.parse_filters(&logging_directive);
    builder.init();

    rust_i18n::set_locale(&detect_locale());

    run_application()
}
