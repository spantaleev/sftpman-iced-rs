use std::ops::Deref;

pub const APP_WIDTH: f32 = 800.0;
pub const APP_HEIGHT: f32 = 700.0;

pub const SCROLLBAR_RESERVED_SPACE: f32 = 15.0;

pub const WIDGET_HORIZONTAL_SPACING: f32 = 10.0;
pub const WIDGET_VERTICAL_SPACING: f32 = 10.0;

pub const ICON_TO_TEXT_SPACING: f32 = 5.0;

pub const FS_LIST_SPACING_BETWEEN_ROW_AND_OPTIONS: f32 = 5.0;
pub const FS_LIST_SPACING_BETWEEN_ROWS: f32 = 5.0;
pub const FS_LIST_EMPTY_LIST_ICON_SIZE: f32 = 48.0;

pub const MODAL_VERTICAL_SPACING: f32 = 20.0;
pub const MODAL_PADDING: f32 = 10.0;
pub const MODAL_MAX_HEIGHT: f32 = 480.0;
pub const MODAL_WIDTH: f32 = 450.0;
pub const MODAL_TITLE_SIZE: f32 = 24.0;
pub const MODAL_MESSAGE_SIZE: f32 = 16.0;

// This is intentionally larger than the default modal width,
// because it contains additional buttons which take space.
pub const MOUNT_ERROR_MODAL_WIDTH: f32 = 600.0;

pub const ABOUT_MODAL_WIDTH: f32 = 550.0;

pub const ICON_SIZE: f32 = 16.0;

pub const RECORD_LABEL_WIDTH: f32 = 230.0;

pub const ABOUT_LABEL_WIDTH: f32 = 110.0;

pub fn home_action_button_width() -> f32 {
    // This abuses the translation system to store the width of the home action buttons.
    // Languages that require more space for the buttons can use a larger value.
    let width = t!("home_action_button_width");

    width.parse::<f32>().unwrap_or_else(|_| panic!("Failed to parse home_action_button_width (`{}`) into a float for the current locale ({})",
        width,
        rust_i18n::locale().deref()))
}
