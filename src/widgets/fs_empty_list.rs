use iced::{
    Length,
    widget::{Container, column, container, text},
};

#[cfg(feature = "icons")]
use iced::widget::svg;

#[cfg(feature = "icons")]
use crate::widgets::{IconColor, icon};

use crate::{messages::Message, ui_config::WIDGET_VERTICAL_SPACING};
use crate::{strings, ui_config::FS_LIST_EMPTY_LIST_ICON_SIZE};

pub fn fs_empty_list() -> Container<'static, Message> {
    #[cfg(feature = "icons")]
    let widget_empty_list_icon = icon(
        &svg::Handle::from_memory(crate::assets::bootstrap_icons::CLIPBOARD_MINUS),
        FS_LIST_EMPTY_LIST_ICON_SIZE,
        IconColor::Text,
    )
    .center_x(Length::Fill);

    let widget_empty_list_text = text(strings::fs_list_empty_list_label())
        .width(Length::Fill)
        .center();

    let mut col = column![].spacing(WIDGET_VERTICAL_SPACING);

    #[cfg(feature = "icons")]
    {
        col = col.push(widget_empty_list_icon);
    }

    col = col.push(widget_empty_list_text);

    container(col)
}
