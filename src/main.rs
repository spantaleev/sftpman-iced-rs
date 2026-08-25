// Various widget builders are only mutated further when the optional `icons` / `file-picker`
// features are enabled (e.g. to attach an icon handle to a button).
// In builds without these features, the `mut` bindings are unnecessary, but we'd rather
// keep the code readable than sprinkle `cfg_attr` attributes over every such call site.
#![cfg_attr(
    not(all(feature = "icons", feature = "file-picker")),
    allow(unused_mut)
)]

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
