use iced::Alignment::Center;
use iced::widget::{Column, Container, Row, container, text};

use iced::Length;
#[cfg(feature = "icons")]
use iced::widget::svg;

use crate::messages::Message;
use crate::pages::HomeMessage;
use crate::strings;
use crate::ui_config::{
    ICON_SIZE, ICON_TO_TEXT_SPACING, WIDGET_HORIZONTAL_SPACING, WIDGET_VERTICAL_SPACING,
    home_action_button_width,
};
use crate::widgets::{Button, IconColor, icon};

pub fn preflight_check_errors_bar(
    errors: Vec<String>,
    theme: &iced::theme::Theme,
) -> Container<'static, Message> {
    let mut row = Row::new()
        .spacing(WIDGET_HORIZONTAL_SPACING)
        .align_y(Center)
        .width(Length::Fill);

    row = row.push(preflight_check_errors_bar_text(errors, theme));

    let mut button = Button::new(strings::button_retry())
        .with_on_press(Some(Message::Home(HomeMessage::RunPreflightCheck)))
        .with_width(Length::Fixed(home_action_button_width()));

    #[cfg(feature = "icons")]
    {
        button = button.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::ARROW_CLOCKWISE,
        ));
    }

    row = row.push(button.build());

    container(row)
}

fn preflight_check_errors_bar_text(
    errors: Vec<String>,
    theme: &iced::theme::Theme,
) -> Row<'static, Message> {
    let mut row = Row::new()
        .spacing(ICON_TO_TEXT_SPACING)
        .align_y(Center)
        .width(Length::Fill);

    #[cfg(feature = "icons")]
    {
        let icon = icon(
            &svg::Handle::from_memory(crate::assets::bootstrap_icons::EXCLAMATION_TRIANGLE),
            ICON_SIZE,
            IconColor::Danger,
        );

        row = row.push(icon);
    }

    let mut column_errors = Column::new().spacing(WIDGET_VERTICAL_SPACING);

    for error in errors {
        column_errors = column_errors.push(text(error).color(theme.palette().danger));
    }

    row = row.push(column_errors);

    row
}
