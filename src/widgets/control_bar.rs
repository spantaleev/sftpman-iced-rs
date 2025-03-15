use iced::widget::{Row, horizontal_space, row};
use iced::{Center, Length};

#[cfg(feature = "icons")]
use iced::widget::svg;

use super::{Button, ButtonStyle};
use crate::messages::Message;
use crate::pages::HomeMessage;
use crate::strings;
use crate::ui_config::{WIDGET_HORIZONTAL_SPACING, home_action_button_width};

pub fn control_bar(
    are_all_mounted: bool,
    are_all_unmounted: bool,
    is_search_enabled: bool,
) -> Row<'static, Message> {
    let mut btn_new = Button::new(strings::control_bar_new_label())
        .with_style(ButtonStyle::Primary)
        .with_tooltip(strings::control_bar_new_tooltip())
        .with_on_press(Some(Message::Home(HomeMessage::New)));

    #[cfg(feature = "icons")]
    {
        btn_new = btn_new.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::PLUS_CIRCLE,
        ));
    }

    let btn_mount_all = Button::new(strings::control_bar_mount_all_label())
        .with_style(if are_all_mounted {
            ButtonStyle::Secondary
        } else {
            ButtonStyle::Success
        })
        .with_tooltip(strings::control_bar_mount_all_tooltip())
        .with_on_press((!are_all_mounted).then_some(Message::Home(HomeMessage::MountAll)));

    let btn_unmount_all = Button::new(strings::control_bar_unmount_all_label())
        .with_style(if are_all_unmounted {
            ButtonStyle::Secondary
        } else {
            ButtonStyle::Danger
        })
        .with_tooltip(strings::control_bar_unmount_all_tooltip())
        .with_on_press((!are_all_unmounted).then_some(Message::Home(HomeMessage::UnmountAll)));

    let mut btn_search = Button::new(strings::control_bar_search_label())
        .with_style(if is_search_enabled {
            ButtonStyle::Primary
        } else {
            ButtonStyle::Secondary
        })
        .with_tooltip(strings::control_bar_search_tooltip())
        .with_on_press(Some(Message::Home(HomeMessage::ToggleSearchEnabled)));

    #[cfg(feature = "icons")]
    {
        btn_search = btn_search.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::SEARCH,
        ));
    }

    let mut btn_about = Button::new(strings::control_bar_about_label())
        .with_style(ButtonStyle::Secondary)
        .with_on_press(Some(Message::Home(HomeMessage::About(true))));

    #[cfg(feature = "icons")]
    {
        btn_about = btn_about.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::INFO_CIRCLE,
        ));
    }

    let home_action_button_width = Length::Fixed(home_action_button_width());

    row![
        btn_new.build(),
        horizontal_space(),
        btn_mount_all.build(),
        btn_unmount_all.build(),
        horizontal_space(),
        btn_search.with_width(home_action_button_width).build(),
        btn_about.with_width(home_action_button_width).build(),
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING)
    .align_y(Center)
}
