use iced::widget::{container, Container};

use crate::application::ApplicationMessage;
use crate::messages::Message;
use crate::widgets::{Button, ButtonStyle};

pub fn text_link(label: String, url: String) -> Container<'static, Message> {
    container(
        Button::new(label)
            .with_style(ButtonStyle::Link)
            .with_on_press(Some(Message::Application(ApplicationMessage::OpenLink(
                url,
            ))))
            .build(),
    )
}
