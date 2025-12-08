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

pub fn confirmation(
    inner_content_container: Container<'static, Message>,
    payload: &ConfirmationConfig,
) -> Element<'static, Message> {
    // Intentionally not using ICON_TO_TEXT_SPACING here, because the heading is large and needs more spacing.
    let mut heading_row = row![].spacing(WIDGET_HORIZONTAL_SPACING).align_y(Center);

    #[cfg(feature = "icons")]
    {
        if let Some(handle) = &payload.svg_icon_handle {
            heading_row = heading_row.push(icon(handle, MODAL_TITLE_SIZE, IconColor::Text));
        }
    }

    heading_row = heading_row.push(text(payload.title.clone()).size(MODAL_TITLE_SIZE));

    let btn_confirm = payload
        .confirmation_button
        .clone()
        .with_on_press(Some(*payload.on_confirm.clone()));

    let btn_cancel = payload
        .cancellation_button
        .clone()
        .with_on_press(Some(*payload.on_cancel.clone()));

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
            row![btn_confirm.build(), Space::new().width(Length::Fill), btn_cancel.build()]
                .spacing(WIDGET_HORIZONTAL_SPACING)
        ]
        .spacing(MODAL_VERTICAL_SPACING),
    )
    .width(MODAL_WIDTH)
    .padding(MODAL_PADDING)
    .style(container::rounded_box);

    modal(inner_content_container, dialog, *payload.on_cancel.clone())
}

#[derive(Debug, Clone)]
pub struct ConfirmationConfig {
    pub title: String,
    pub message: String,

    pub on_confirm: Box<Message>,
    pub confirmation_button: Box<Button>,

    pub on_cancel: Box<Message>,
    pub cancellation_button: Box<Button>,

    #[cfg(feature = "icons")]
    pub svg_icon_handle: Option<svg::Handle>,
}

impl ConfirmationConfig {
    pub fn new(
        title: String,
        message: String,
        on_confirm: Box<Message>,
        on_cancel: Box<Message>,
    ) -> Self {
        let mut btn_confirm = Button::new(strings::confirmation_confirmation_button_label())
            .with_style(ButtonStyle::Primary);

        #[cfg(feature = "icons")]
        {
            btn_confirm = btn_confirm.with_svg_icon_handle(svg::Handle::from_memory(
                crate::assets::bootstrap_icons::CHECK_CIRCLE,
            ));
        }

        let mut btn_cancel = Button::new(strings::confirmation_cancellation_button_label())
            .with_style(ButtonStyle::SecondaryOutlined);

        #[cfg(feature = "icons")]
        {
            btn_cancel = btn_cancel.with_svg_icon_handle(svg::Handle::from_memory(
                crate::assets::bootstrap_icons::ARROW_RETURN_LEFT,
            ));
        }

        Self {
            title,
            message,

            on_confirm,
            confirmation_button: Box::new(btn_confirm),

            on_cancel,
            cancellation_button: Box::new(btn_cancel),

            #[cfg(feature = "icons")]
            svg_icon_handle: Some(svg::Handle::from_memory(
                crate::assets::bootstrap_icons::QUESTION_CIRCLE,
            )),
        }
    }

    pub fn with_confirmation_button(mut self, button: Button) -> Self {
        self.confirmation_button = Box::new(button);
        self
    }

    pub fn with_cancellation_button(mut self, button: Button) -> Self {
        self.cancellation_button = Box::new(button);
        self
    }
}
