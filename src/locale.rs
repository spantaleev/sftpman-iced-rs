use std::env;

const DEFAULT_LOCALE: &str = "en";

pub fn detect_locale() -> String {
    detect_locale_by_lang_value(&env::var("LANG").unwrap_or_default())
}

fn detect_locale_by_lang_value(lang: &str) -> String {
    let lang = if lang.is_empty() {
        DEFAULT_LOCALE
    } else {
        lang
    };

    lang.split('.')
        .next()
        .unwrap_or(DEFAULT_LOCALE)
        .replace('_', "-")
}

#[cfg(test)]
mod tests {
    use super::{detect_locale_by_lang_value, DEFAULT_LOCALE};

    #[test]
    fn test_detect_locale() {
        assert_eq!(detect_locale_by_lang_value("en"), "en");
        assert_eq!(detect_locale_by_lang_value(""), DEFAULT_LOCALE);
        assert_eq!(detect_locale_by_lang_value("en_US.UTF-8"), "en-US");

        assert_eq!(detect_locale_by_lang_value("bg"), "bg");
        assert_eq!(detect_locale_by_lang_value("bg_BG.UTF-8"), "bg-BG");
    }
}
