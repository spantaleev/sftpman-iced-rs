use std::ops::Deref;

pub const APP_WIDTH: u16 = 800;
pub const APP_HEIGHT: u16 = 700;

pub const SCROLLBAR_RESERVED_SPACE: u16 = 15;

pub const WIDGET_HORIZONTAL_SPACING: u16 = 10;
pub const WIDGET_VERTICAL_SPACING: u16 = 10;

pub const ICON_TO_TEXT_SPACING: u16 = 5;

pub const FS_LIST_SPACING_BETWEEN_ROW_AND_OPTIONS: u16 = 5;
pub const FS_LIST_SPACING_BETWEEN_ROWS: u16 = 5;
pub const FS_LIST_EMPTY_LIST_ICON_SIZE: u16 = 48;

pub const MODAL_VERTICAL_SPACING: u16 = 20;
pub const MODAL_PADDING: u16 = 10;
pub const MODAL_MAX_HEIGHT: u16 = 480;
pub const MODAL_WIDTH: u16 = 450;
pub const MODAL_TITLE_SIZE: u16 = 24;
pub const MODAL_MESSAGE_SIZE: u16 = 16;

// This is intentionally larger than the default modal width,
// because it contains additional buttons which take space.
pub const MOUNT_ERROR_MODAL_WIDTH: u16 = 600;

pub const ABOUT_MODAL_WIDTH: u16 = 550;

pub const ICON_SIZE: u16 = 16;

pub const RECORD_LABEL_WIDTH: u16 = 230;

pub const ABOUT_LABEL_WIDTH: u16 = 110;

pub fn home_action_button_width() -> f32 {
    // This abuses the translation system to store the width of the home action buttons.
    // Languages that require more space for the buttons can use a larger value.
    let width = t!("home_action_button_width");

    width.parse::<f32>().unwrap_or_else(|_| panic!("Failed to parse home_action_button_width (`{}`) into a float for the current locale ({})",
        width,
        rust_i18n::locale().deref()))
}
