#[cfg(feature = "x11-icon")]
pub const APPLICATION_ICON: &[u8] = include_bytes!("../assets/sftpman-iced-512.png");

#[cfg(feature = "icons")]
pub const APPLICATION_ICON_SVG: &[u8] = include_bytes!("../assets/sftpman-iced.svg");

#[cfg(feature = "icons")]
pub mod bootstrap_icons {
    // We used to use iced_fonts::bootstrap for these icons, but this increases the binary size by 2MB.
    // We're now manually including only the icons we need.
    pub const ARROW_RETURN_LEFT: &[u8] =
        include_bytes!("../assets/bootstrap/arrow-return-left.svg");
    pub const ARROW_CLOCKWISE: &[u8] = include_bytes!("../assets/bootstrap/arrow-clockwise.svg");
    pub const BUG: &[u8] = include_bytes!("../assets/bootstrap/bug.svg");
    pub const CHECK_CIRCLE: &[u8] = include_bytes!("../assets/bootstrap/check-circle.svg");
    pub const CHEVRON_DOWN: &[u8] = include_bytes!("../assets/bootstrap/chevron-down.svg");
    pub const CHEVRON_UP: &[u8] = include_bytes!("../assets/bootstrap/chevron-up.svg");
    pub const CLIPBOARD_CHECK: &[u8] = include_bytes!("../assets/bootstrap/clipboard-check.svg");
    pub const CLIPBOARD_MINUS: &[u8] = include_bytes!("../assets/bootstrap/clipboard-minus.svg");
    pub const COPY: &[u8] = include_bytes!("../assets/bootstrap/copy.svg");
    pub const EXCLAMATION_TRIANGLE: &[u8] =
        include_bytes!("../assets/bootstrap/exclamation-triangle.svg");
    pub const FOLDER2_OPEN: &[u8] = include_bytes!("../assets/bootstrap/folder2-open.svg");
    pub const INFO_CIRCLE: &[u8] = include_bytes!("../assets/bootstrap/info-circle.svg");
    pub const PENCIL: &[u8] = include_bytes!("../assets/bootstrap/pencil.svg");
    pub const PLUS_CIRCLE: &[u8] = include_bytes!("../assets/bootstrap/plus-circle.svg");
    pub const QUESTION_CIRCLE: &[u8] = include_bytes!("../assets/bootstrap/question-circle.svg");
    pub const SEARCH: &[u8] = include_bytes!("../assets/bootstrap/search.svg");
    pub const TRASH: &[u8] = include_bytes!("../assets/bootstrap/trash.svg");
}
