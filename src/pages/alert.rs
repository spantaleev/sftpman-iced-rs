use iced::Alignment::Center;
use iced::widget::{Container, Space, column, container, row, scrollable, text};
use iced::{Element, Length};

#[cfg(feature = "icons")]
use iced::widget::svg;

use crate::messages::Message;
use crate::strings;
use crate::ui_config::{
    MODAL_MAX_HEIGHT, MODAL_MESSAGE_SIZE, MODAL_PADDING, MODAL_TITLE_SIZE, MODAL_VERTICAL_SPACING,
    MODAL_WIDTH, SCROLLBAR_RESERVED_SPACE, WIDGET_HORIZONTAL_SPACING,
};
use crate::widgets::{Button, ButtonStyle, modal};

#[cfg(feature = "icons")]
use crate::widgets::{IconColor, icon};

pub fn alert(
    inner_content_container: Container<'static, Message>,
    payload: &AlertConfig,
) -> Element<'static, Message> {
    // Intentionally not using ICON_TO_TEXT_SPACING here, because the heading is large and needs more spacing.
    let mut heading_row = row![].spacing(WIDGET_HORIZONTAL_SPACING).align_y(Center);

    #[cfg(feature = "icons")]
    {
        if let Some(svg_icon_handle) = &payload.svg_icon_handle {
            heading_row =
                heading_row.push(icon(svg_icon_handle, MODAL_TITLE_SIZE, IconColor::Text));
        }
    }

    heading_row = heading_row.push(text(payload.title.clone()).size(MODAL_TITLE_SIZE));

    let btn_close = payload
        .close_button
        .clone()
        .with_on_press(Some(*payload.on_close.clone()));

    let mut buttons_row = row![].spacing(WIDGET_HORIZONTAL_SPACING);
    for btn in payload.additional_buttons.iter() {
        buttons_row = buttons_row.push(btn.clone().build());
    }
    buttons_row = buttons_row.push(Space::new().width(Length::Fill));
    buttons_row = buttons_row.push(btn_close.build());

    let dialog = container(
        column![
            heading_row,
            container(scrollable(
                row![
                    text(payload.message.clone()).size(MODAL_MESSAGE_SIZE),
                    Space::new().width(SCROLLBAR_RESERVED_SPACE)
                ]
                .width(Length::Fill)
                .spacing(WIDGET_HORIZONTAL_SPACING),
            ),)
            .max_height(MODAL_MAX_HEIGHT),
            buttons_row,
        ]
        .spacing(MODAL_VERTICAL_SPACING),
    )
    .width(payload.width)
    .padding(MODAL_PADDING)
    .style(container::rounded_box);

    modal(inner_content_container, dialog, *payload.on_close.clone())
}

#[derive(Debug, Clone)]
pub struct AlertConfig {
    pub title: String,
    pub message: String,
    pub on_close: Box<Message>,
    pub close_button: Box<Button>,
    pub additional_buttons: Vec<Button>,
    pub width: f32,

    #[cfg(feature = "icons")]
    pub svg_icon_handle: Option<svg::Handle>,
}

impl AlertConfig {
    pub fn new(title: String, message: String, on_close: Box<Message>) -> Self {
        let mut btn_close =
            Button::new(strings::alert_close_button_label()).with_style(ButtonStyle::Primary);

        #[cfg(feature = "icons")]
        {
            btn_close = btn_close.with_svg_icon_handle(svg::Handle::from_memory(
                crate::assets::bootstrap_icons::CHECK_CIRCLE,
            ));
        }

        Self {
            title,
            message,
            on_close,
            close_button: Box::new(btn_close),
            additional_buttons: vec![],
            width: MODAL_WIDTH,

            #[cfg(feature = "icons")]
            svg_icon_handle: Some(svg::Handle::from_memory(
                crate::assets::bootstrap_icons::INFO_CIRCLE,
            )),
        }
    }

    pub fn with_additional_buttons(mut self, buttons: Vec<Button>) -> Self {
        self.additional_buttons = buttons;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    #[cfg(feature = "icons")]
    pub fn with_svg_icon_handle(mut self, svg_icon_handle: svg::Handle) -> Self {
        self.svg_icon_handle = Some(svg_icon_handle);
        self
    }
}
