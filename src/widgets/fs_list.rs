use iced::widget::{horizontal_space, row, toggler, Column, Container, Row};
use iced::Length;

#[cfg(feature = "icons")]
use iced::widget::svg;

use libsftpman::{FilesystemMountDefinition, MountState};

use super::{Button, ButtonIconPosition, ButtonStyle};
use crate::messages::Message;
use crate::pages::HomeMessage;
use crate::strings;
use crate::ui_config::{
    home_action_button_width, FS_LIST_SPACING_BETWEEN_ROWS,
    FS_LIST_SPACING_BETWEEN_ROW_AND_OPTIONS, WIDGET_HORIZONTAL_SPACING,
};

pub fn fs_list(
    state: Vec<MountState>,
    expand_options_for_fs_id: Option<String>,
    mounting_in_progress_for_fs_id: Option<String>,
    unmounting_in_progress_for_fs_id: Option<String>,
) -> Column<'static, Message> {
    let home_action_button_width = Length::Fixed(home_action_button_width());

    state
        .into_iter()
        .fold(Column::new(), |col, entity| {
            let is_expanded = expand_options_for_fs_id
                .as_ref()
                .map_or(false, |id| id == &entity.definition.id);

            let is_mounting = mounting_in_progress_for_fs_id
                .as_ref()
                .map_or(false, |id| id == &entity.definition.id);

            let is_unmounting = unmounting_in_progress_for_fs_id
                .as_ref()
                .map_or(false, |id| id == &entity.definition.id);

            col.push(fs_list_row(
                entity,
                &home_action_button_width,
                is_expanded,
                is_mounting,
                is_unmounting,
            ))
        })
        .spacing(FS_LIST_SPACING_BETWEEN_ROWS)
}

fn fs_list_row(
    entity: MountState,
    home_action_button_width: &Length,
    options_expanded: bool,
    is_mounting: bool,
    is_unmounting: bool,
) -> Container<'static, Message> {
    let definition = entity.definition.clone();

    let toggler_checked = (entity.mounted || is_mounting) && !is_unmounting;

    let label = if is_mounting {
        strings::filesystem_definition_name_mounting_label(&entity.definition.id)
    } else if is_unmounting {
        strings::filesystem_definition_name_unmounting_label(&entity.definition.id)
    } else {
        entity.definition.id.clone()
    };

    let widget_toggler = toggler(toggler_checked)
        .label(label)
        // Make it larger (default is 16), as this is an important action that is frequently used
        .size(24)
        .on_toggle(move |new_toggle_state| {
            if new_toggle_state {
                Message::Home(HomeMessage::Mount(definition.clone()))
            } else {
                Message::Home(HomeMessage::Unmount(definition.clone()))
            }
        });

    let mut widget_btn_open = Button::new(strings::fs_list_open_label())
        .with_style(if entity.mounted && !is_mounting {
            ButtonStyle::Primary
        } else {
            ButtonStyle::Secondary
        })
        .with_width(*home_action_button_width)
        .with_on_press(
            (entity.mounted).then_some(Message::Home(HomeMessage::Open(entity.definition.clone()))),
        );

    #[cfg(feature = "icons")]
    {
        widget_btn_open = widget_btn_open.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::FOLDER2_OPEN,
        ));
    }

    let widget_btn_options = create_dropdown_button(&entity.definition, options_expanded)
        .with_width(*home_action_button_width);

    let row = row![
        widget_toggler,
        horizontal_space(),
        widget_btn_open.build(),
        widget_btn_options.build(),
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    let mut col =
        Column::with_children([row.into()]).spacing(FS_LIST_SPACING_BETWEEN_ROW_AND_OPTIONS);

    if options_expanded {
        col = col.push(build_options_row(
            &entity.definition,
            home_action_button_width,
        ));
    }

    Container::new(col)
}

fn build_options_row(
    definition: &FilesystemMountDefinition,
    home_action_button_width: &Length,
) -> Row<'static, Message> {
    row![
        horizontal_space(),
        create_edit_option_button(definition)
            .with_width(*home_action_button_width)
            .build(),
        create_clone_option_button(definition)
            .with_width(*home_action_button_width)
            .build(),
        create_remove_option_button(definition)
            .with_width(*home_action_button_width)
            .build(),
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING)
}

fn create_dropdown_button(definition: &FilesystemMountDefinition, is_expanded: bool) -> Button {
    // We previously used an iced_aw dropdown widget for this, but we've replaced it
    // with a custom alternative, because dropdowns do not work well in scrollable lists.
    // See: https://github.com/iced-rs/iced_aw/issues/300#issuecomment-2563377964

    let mut btn = Button::new(strings::fs_list_options_label())
        .with_style(ButtonStyle::Secondary)
        .with_icon_position(ButtonIconPosition::Right);

    btn = if is_expanded {
        btn = btn.with_style(ButtonStyle::Primary);

        #[cfg(feature = "icons")]
        {
            btn = btn.with_svg_icon_handle(svg::Handle::from_memory(
                crate::assets::bootstrap_icons::CHEVRON_UP,
            ));
        }

        btn.with_on_press(Some(Message::Home(HomeMessage::ToggleOptionsForRecord(
            None,
        ))))
    } else {
        #[cfg(feature = "icons")]
        {
            btn = btn.with_svg_icon_handle(svg::Handle::from_memory(
                crate::assets::bootstrap_icons::CHEVRON_DOWN,
            ));
        }

        btn.with_on_press(Some(Message::Home(HomeMessage::ToggleOptionsForRecord(
            Some(definition.clone()),
        ))))
    };

    btn
}

fn create_edit_option_button(definition: &FilesystemMountDefinition) -> Button {
    let mut btn = Button::new(strings::edit_button_label())
        .with_style(ButtonStyle::Primary)
        .with_on_press(Some(Message::Home(HomeMessage::EditRecord(
            definition.clone(),
        ))));

    #[cfg(feature = "icons")]
    {
        btn = btn.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::PENCIL,
        ));
    }

    btn
}

fn create_clone_option_button(definition: &FilesystemMountDefinition) -> Button {
    let mut btn = Button::new(strings::clone_button_label())
        .with_style(ButtonStyle::Secondary)
        .with_on_press(Some(Message::Home(HomeMessage::CloneRecord(
            definition.clone(),
        ))));

    #[cfg(feature = "icons")]
    {
        btn = btn.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::COPY,
        ));
    }

    btn
}

fn create_remove_option_button(definition: &FilesystemMountDefinition) -> Button {
    let mut btn = Button::new(strings::remove_button_label())
        .with_style(ButtonStyle::Danger)
        .with_on_press(Some(Message::Home(HomeMessage::RemoveRecord(
            definition.clone(),
        ))));

    #[cfg(feature = "icons")]
    {
        btn = btn.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::TRASH,
        ));
    }

    btn
}
