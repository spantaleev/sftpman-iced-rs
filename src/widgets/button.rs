use iced::widget::{button as iced_button, container, horizontal_space, row, text, tooltip};
use iced::{Center, Element, Length};

#[cfg(feature = "icons")]
use iced::widget::svg;

use crate::messages::Message;
use crate::ui_config::{ICON_SIZE, ICON_TO_TEXT_SPACING};

#[derive(Debug, Clone)]
pub struct Button {
    label: String,
    style: ButtonStyle,
    on_press: Option<Message>,
    tooltip: Option<String>,
    #[cfg(feature = "icons")]
    svg_icon_handle: Option<svg::Handle>,
    icon_position: ButtonIconPosition,
    width: Length,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonIconPosition {
    Left,
    Right,
}

impl Button {
    pub fn new(label: String) -> Self {
        Button {
            label,
            style: ButtonStyle::Primary,
            on_press: None,
            tooltip: None,
            #[cfg(feature = "icons")]
            svg_icon_handle: None,
            icon_position: ButtonIconPosition::Left,
            width: Length::Shrink,
        }
    }

    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_on_press(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press;
        self
    }

    pub fn with_tooltip(mut self, tooltip: String) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    #[cfg(feature = "icons")]
    pub fn with_svg_icon_handle(mut self, svg_icon_handle: svg::Handle) -> Self {
        self.svg_icon_handle = Some(svg_icon_handle);
        self
    }

    pub fn with_icon_position(mut self, icon_position: ButtonIconPosition) -> Self {
        self.icon_position = icon_position;
        self
    }

    pub fn with_width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    #[cfg(feature = "icons")]
    pub fn build_icon_text(&self) -> Option<Element<'static, Message>> {
        let button_style = self.style;

        if let Some(svg_icon_handle) = &self.svg_icon_handle {
            let svg_icon = svg(svg_icon_handle.clone())
                .style(move |theme: &iced::theme::Theme, _status| {
                    let button_style =
                        button_style.to_iced_style(theme, iced_button::Status::Active);

                    svg::Style {
                        color: Some(button_style.text_color),
                    }
                })
                .width(ICON_SIZE)
                .height(ICON_SIZE);

            let svg_icon_container = container(svg_icon);

            Some(svg_icon_container.into())
        } else {
            None
        }
    }

    pub fn build(self) -> Element<'static, Message> {
        let mut button_content = row![].spacing(ICON_TO_TEXT_SPACING).align_y(Center);

        #[cfg(feature = "icons")]
        {
            if let ButtonIconPosition::Left = self.icon_position {
                let icon_text = self.build_icon_text();
                if let Some(icon_text) = icon_text {
                    button_content = button_content.push(icon_text);
                }
            }
        }

        button_content = button_content.push(text(self.label.clone()));

        #[cfg(feature = "icons")]
        {
            if let ButtonIconPosition::Right = self.icon_position {
                let icon_text = self.build_icon_text();
                if let Some(icon_text) = icon_text {
                    button_content = button_content.push(horizontal_space());
                    button_content = button_content.push(icon_text);
                }
            }
        }

        let mut btn = iced_button(button_content);

        if let Some(on_press) = self.on_press {
            btn = btn.on_press(on_press);
        }

        btn = btn.style(self.style.style_fn());

        if let ButtonStyle::Link = self.style {
            btn = btn.padding(0);
        }

        btn = btn.width(self.width);

        match self.tooltip {
            Some(tooltip_text) => {
                let tooltip_value = text(tooltip_text);

                tooltip(btn, tooltip_value, tooltip::Position::FollowCursor)
                    .style(container::rounded_box)
                    .into()
            }
            None => btn.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonStyle {
    Primary,
    Success,
    Danger,
    Secondary,
    // SecondaryOutlined is a version of Secondary with a border, so that it's more legible
    // when the Secondary button is used in a modal window with a background color matching the secondary button's color.
    SecondaryOutlined,
    Link,
}

impl ButtonStyle {
    pub fn to_iced_style(
        self,
        theme: &iced::theme::Theme,
        active_status: iced_button::Status,
    ) -> iced_button::Style {
        match self {
            ButtonStyle::Primary => iced_button::primary(theme, active_status),
            ButtonStyle::Danger => iced_button::danger(theme, active_status),
            ButtonStyle::Success => iced_button::success(theme, active_status),
            ButtonStyle::Secondary => iced_button::secondary(theme, active_status),
            ButtonStyle::SecondaryOutlined => {
                let mut style = iced_button::secondary(theme, active_status);
                style.border = style.border.color(theme.palette().text).width(1);
                style
            }
            ButtonStyle::Link => {
                let mut style = iced_button::text(theme, active_status);
                style.text_color = theme.palette().primary;
                style
            }
        }
    }

    pub fn style_fn(
        self,
    ) -> impl Fn(&iced::theme::Theme, iced_button::Status) -> iced_button::Style {
        move |theme: &iced::theme::Theme, status: iced_button::Status| {
            self.to_iced_style(theme, status)
        }
    }
}
