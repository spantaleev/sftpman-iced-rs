use iced::widget::{Container, Space, column, container, row, scrollable, text};
use iced::{Element, Length};

#[cfg(feature = "icons")]
use iced::widget::svg;

use crate::messages::Message;
use crate::pages::HomeMessage;
use crate::strings;
use crate::ui_config::{
    ABOUT_LABEL_WIDTH, ABOUT_MODAL_WIDTH, MODAL_MAX_HEIGHT, MODAL_PADDING, MODAL_VERTICAL_SPACING,
    SCROLLBAR_RESERVED_SPACE, WIDGET_HORIZONTAL_SPACING, WIDGET_VERTICAL_SPACING,
};
use crate::widgets::{Button, field_control_label, modal, text_link};

const SFTPMAN_ICED_LINK: &str = "https://github.com/spantaleev/sftpman-iced-rs";
const SFTPMAN_LINK: &str = "https://github.com/spantaleev/sftpman-rs";
const AUTHOR_NAME: &str = "Slavi Pantaleev";
const AUTHOR_LINK: &str = "https://github.com/spantaleev";
const AUTHOR_COMPANY_NAME: &str = "devture.com";
const AUTHOR_COMPANY_LINK: &str = "https://devture.com/";
const DONATE_LIBERAPAY_LINK: &str = "https://liberapay.com/s.pantaleev";
const DONATE_KOFI_LINK: &str = "https://ko-fi.com/spantaleev";

pub fn about(
    home_container: Container<'static, Message>,
    theme: &iced::theme::Theme,
) -> Element<'static, Message> {
    let mut btn_close = Button::new(strings::about_button_close_label())
        .with_on_press(Some(Message::Home(HomeMessage::About(false))));

    #[cfg(feature = "icons")]
    {
        btn_close = btn_close.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::ARROW_RETURN_LEFT,
        ));
    }

    let about = container(
        column![
            container(scrollable(
                row![
                    about_content(theme),
                    Space::new().width(SCROLLBAR_RESERVED_SPACE)
                ]
                .width(Length::Fill)
                .spacing(WIDGET_HORIZONTAL_SPACING),
            ),)
            .max_height(MODAL_MAX_HEIGHT),
            column![btn_close.build()].spacing(WIDGET_HORIZONTAL_SPACING),
        ]
        .spacing(MODAL_VERTICAL_SPACING),
    )
    .width(ABOUT_MODAL_WIDTH)
    .padding(MODAL_PADDING)
    .style(container::rounded_box);

    modal(
        home_container,
        about,
        Message::Home(HomeMessage::About(false)),
    )
}

fn about_content(theme: &iced::theme::Theme) -> Container<'static, Message> {
    #[cfg(feature = "icons")]
    let svg_icon_handle = svg::Handle::from_memory(crate::assets::APPLICATION_ICON_SVG);

    #[cfg(feature = "icons")]
    let row_icon = row![
        Space::new().width(Length::Fill),
        container(svg(svg_icon_handle)),
        Space::new().width(Length::Fill),
    ];

    #[cfg(not(feature = "icons"))]
    let row_icon = row![];

    let row_program = row![
        field_control_label(strings::about_program_label(), false, theme).width(ABOUT_LABEL_WIDTH),
        column![
            row![
                text_link(
                    env!("CARGO_PKG_NAME").to_string(),
                    SFTPMAN_ICED_LINK.to_string()
                ),
                Space::new().width(Length::Fill),
                text(env!("CARGO_PKG_VERSION")),
            ]
            .spacing(WIDGET_HORIZONTAL_SPACING),
            row![
                text(strings::about_program_powered_by_label()),
                text_link("sftpman".to_string(), SFTPMAN_LINK.to_string(),),
                Space::new().width(Length::Fill),
                text(libsftpman::VERSION),
            ]
            .spacing(WIDGET_HORIZONTAL_SPACING),
        ]
        .spacing(WIDGET_VERTICAL_SPACING),
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    let row_description = row![
        field_control_label(strings::about_description_label(), false, theme)
            .width(ABOUT_LABEL_WIDTH),
        text(strings::about_description_message()),
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    let row_author = row![
        field_control_label(strings::about_author_label(), false, theme).width(ABOUT_LABEL_WIDTH),
        column![
            row![
                text_link(AUTHOR_NAME.to_string(), AUTHOR_LINK.to_string()),
                Space::new().width(Length::Fill),
                text_link(
                    AUTHOR_COMPANY_NAME.to_string(),
                    AUTHOR_COMPANY_LINK.to_string()
                ),
            ],
            text(strings::about_donate_message()),
            row![
                text_link(
                    strings::about_donate_on_platform_button("Liberapay"),
                    DONATE_LIBERAPAY_LINK.to_string()
                ),
                Space::new().width(Length::Fill),
                text_link(
                    strings::about_donate_on_platform_button("Ko-Fi"),
                    DONATE_KOFI_LINK.to_string()
                ),
            ]
            .spacing(WIDGET_HORIZONTAL_SPACING),
        ]
        .spacing(WIDGET_VERTICAL_SPACING),
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    let row_license = row![
        field_control_label(strings::about_license_label(), false, theme).width(ABOUT_LABEL_WIDTH),
        text_link(
            "GNU Affero General Public License v3.0".to_string(),
            "https://www.gnu.org/licenses/agpl-3.0.html".to_string()
        ),
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    container(
        column![
            row_icon,
            row_program,
            row_description,
            row_author,
            row_license
        ]
        .spacing(WIDGET_VERTICAL_SPACING),
    )
}
