use iced::widget::container::rounded_box;
use iced::widget::{Container, container, row, text, tooltip, Space};
use iced::Length;

use crate::messages::Message;
use crate::strings;

pub fn field_control_label(
    label: String,
    is_required: bool,
    theme: &iced::theme::Theme,
) -> Container<'static, Message> {
    let mut row_label = row![text(label)];

    if is_required {
        row_label = row_label.push(Space::new().width(Length::Fill));

        row_label = row_label.push(
            tooltip(
                text(strings::field_control_label_required_label()).color(theme.palette().danger),
                text(strings::field_control_label_required_tooltip()),
                tooltip::Position::FollowCursor,
            )
            .style(rounded_box),
        );
    }

    container(row_label)
}
