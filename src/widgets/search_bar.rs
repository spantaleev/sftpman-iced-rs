use iced::widget::{row, text_input, Row};
use iced::Center;

use crate::messages::Message;
use crate::pages::HomeMessage;
use crate::strings;
use crate::ui_config::WIDGET_HORIZONTAL_SPACING;

pub const SEARCH_BAR_INPUT_FIELD_ID: &str = "search-bar-input-field";

pub fn search_bar(input_value: &str) -> Row<'static, Message> {
    let widget_text_input = text_input(&strings::search_input_placeholder(), input_value)
        .id(SEARCH_BAR_INPUT_FIELD_ID)
        .on_input(|v| Message::Home(HomeMessage::SearchInputChanged(v)));

    row![widget_text_input]
        .spacing(WIDGET_HORIZONTAL_SPACING)
        .align_y(Center)
}
