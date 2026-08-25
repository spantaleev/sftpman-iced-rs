use iced::widget::{Container, container};

use iced::widget::svg;

use crate::messages::Message;

#[cfg(feature = "icons")]
pub fn icon(handle: &svg::Handle, size: f32, color: IconColor) -> Container<'static, Message> {
    let svg_icon = svg(handle.clone()).width(size).height(size).style(
        move |theme: &iced::theme::Theme, _status| {
            let color = match color {
                IconColor::Text => theme.palette().text,
                IconColor::Primary => theme.palette().primary,
                IconColor::Danger => theme.palette().danger,
            };

            svg::Style { color: Some(color) }
        },
    );

    container(svg_icon)
}

pub enum IconColor {
    Text,
    Primary,
    Danger,
}
