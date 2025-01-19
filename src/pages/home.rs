use std::time::{Duration, Instant};

use iced::widget::{column, container, scrollable, text_input, Container};
use iced::{keyboard, time, Length, Padding, Subscription, Task};

#[cfg(feature = "icons")]
use iced::widget::svg;

use libsftpman::{FilesystemMountDefinition, Manager, MountState};

use crate::application::{ApplicationMessage, Navigation, Page};
use crate::messages::Message as GlobalMessage;
use crate::pages::{about, alert, confirmation, AlertConfig, ConfirmationConfig, Record};
use crate::strings;
use crate::ui_config::{
    MOUNT_ERROR_MODAL_WIDTH, SCROLLBAR_RESERVED_SPACE, WIDGET_VERTICAL_SPACING,
};
use crate::widgets::{
    control_bar, fs_empty_list, fs_list, preflight_check_errors_bar, search_bar, Button,
    ButtonStyle, SEARCH_BAR_INPUT_FIELD_ID,
};

const REFRESH_INTERVAL_MS: u64 = 5000;

#[derive(Debug, Clone)]
pub enum Message {
    New,

    Mount(FilesystemMountDefinition),
    DoMount(FilesystemMountDefinition),
    MountResult(FilesystemMountDefinition, Option<(String, String)>),

    Unmount(FilesystemMountDefinition),
    DoUnmount(FilesystemMountDefinition),
    UnmountResult(FilesystemMountDefinition, Option<(String, String)>),

    Open(FilesystemMountDefinition),

    ToggleOptionsForRecord(Option<FilesystemMountDefinition>),
    EditRecord(FilesystemMountDefinition),
    CloneRecord(FilesystemMountDefinition),
    RemoveRecord(FilesystemMountDefinition),
    RemoveRecordConfirmed(FilesystemMountDefinition),
    RemoveRecordCancelled,

    MountAll,
    UnmountAll,

    ToggleSearchEnabled,
    SearchInputChanged(String),

    About(bool),

    Alert(Option<AlertConfig>),
    Confirmation(Option<ConfirmationConfig>),

    PeriodicRefreshTicked(Instant),

    RunPreflightCheck,
}

struct State {
    search_enabled: bool,
    search_input_text: String,

    // expand_options_for_fs_id is used to indicate for which filesystem we're showing the Options menu.
    // This is a single id (not a list), because it's possibly not very useful to expand multiple menus at once.
    expand_options_for_fs_id: Option<String>,

    // mounting_in_progress_for_fs_id is used to indicate that the given filesystem is being mounted.
    // This is a single id (not a list), because mounting is intentionally synchronous
    // (so we can ask for SSH passphrases in the foreground, etc.).
    mounting_in_progress_for_fs_id: Option<String>,

    // mounting_scheduled_for_definitions is used to indicate that the given filesystems are scheduled for mounting.
    // This is used when performing a "Mount all" operation.
    mounting_scheduled_for_definitions: Vec<FilesystemMountDefinition>,

    // unmounting_in_progress_for_fs_id is used to indicate that the given filesystem is being unmounted.
    // This is a single id (not a list), because unmounting is intentionally synchronous.
    // Unmounting can potentially be asynchronous, but:
    // - we're being consistent with the way we do mounting
    // - if it is, multiple unmounts can fail at the same time and we'll need to support simultaneous alerting
    //   (we only handle single alert right now - see the `alert` field below)
    unmounting_in_progress_for_fs_id: Option<String>,

    // unmounting_scheduled_for_definitions is used to indicate that the given filesystems are scheduled for unmounting.
    // This is used when performing an "Unmount all" operation.
    unmounting_scheduled_for_definitions: Vec<FilesystemMountDefinition>,

    confirmation: Option<ConfirmationConfig>,

    alert: Option<AlertConfig>,

    about_shown: bool,

    preflight_check_errors: Vec<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            search_enabled: false,
            search_input_text: "".to_owned(),
            expand_options_for_fs_id: None,

            mounting_in_progress_for_fs_id: None,
            mounting_scheduled_for_definitions: Vec::new(),

            unmounting_in_progress_for_fs_id: None,
            unmounting_scheduled_for_definitions: Vec::new(),

            confirmation: None,
            alert: None,
            about_shown: false,

            preflight_check_errors: Vec::new(),
        }
    }
}

pub struct Home {
    manager: Manager,
    state: State,
}

impl Home {
    pub fn new(manager: Manager) -> Self {
        Self {
            manager,
            state: State::default(),
        }
    }

    fn filesystems_filtered(&self) -> Vec<MountState> {
        let state = self.manager.full_state().unwrap();

        if self.state.search_enabled {
            state
                .into_iter()
                .filter(|item| item.definition.id.contains(&self.state.search_input_text))
                .collect::<Vec<_>>()
        } else {
            state
        }
    }

    fn render(&self, theme: &iced::theme::Theme) -> Container<'static, GlobalMessage> {
        let state_filtered = self.filesystems_filtered();

        let are_all_mounted = state_filtered.iter().all(|item| item.mounted);
        let are_all_unmounted = state_filtered.iter().all(|item| !item.mounted);

        // We reserve some space (right-padding) for the scrollbar in the widget_fs_list.
        // We don't add the padding to the column itself, because this way we don't get a scrollbar that's
        // right-most attached.
        //
        // We similarly add the same right-padding (and left-padding) to all other widgets that are part of the same column,
        // for alignment purposes.

        let widget_control_bar = control_bar(
            are_all_mounted,
            are_all_unmounted,
            self.state.search_enabled,
        )
        .padding(Padding::new(0.0).right(SCROLLBAR_RESERVED_SPACE));

        let mut main_column = column![widget_control_bar,]
            .spacing(WIDGET_VERTICAL_SPACING)
            .padding(
                Padding::new(0.0)
                    .top(WIDGET_VERTICAL_SPACING)
                    .left(SCROLLBAR_RESERVED_SPACE),
            );

        if self.state.search_enabled {
            let search_bar = search_bar(&self.state.search_input_text)
                .padding(Padding::new(0.0).right(SCROLLBAR_RESERVED_SPACE));

            main_column = main_column.push(search_bar);
        }

        if !self.state.preflight_check_errors.is_empty() {
            let widget_preflight_check_errors =
                preflight_check_errors_bar(self.state.preflight_check_errors.clone(), theme)
                    .padding(
                        Padding::new(0.0)
                            .right(SCROLLBAR_RESERVED_SPACE)
                            .bottom(WIDGET_VERTICAL_SPACING),
                    );

            main_column = main_column.push(widget_preflight_check_errors);
        }

        if !state_filtered.is_empty() {
            let widget_fs_list = fs_list(
                state_filtered,
                self.state.expand_options_for_fs_id.clone(),
                self.state.mounting_in_progress_for_fs_id.clone(),
                self.state.unmounting_in_progress_for_fs_id.clone(),
            )
            .padding(
                Padding::new(0.0)
                    .right(SCROLLBAR_RESERVED_SPACE)
                    .bottom(WIDGET_VERTICAL_SPACING),
            );
            main_column = main_column.push(scrollable(widget_fs_list));
        } else {
            let widget_empty_list = fs_empty_list()
                .height(Length::Fill)
                .align_y(iced::Alignment::Center);

            main_column = main_column.push(widget_empty_list);
        }

        // We expand this to fill the entire window height, because alert/confirmation modals
        // are shown on top of it and a small base container would not result in a well-visible modal.
        container(main_column).height(Length::Fill)
    }

    fn alert(&self, title: String, message: String, additional_buttons: Vec<Button>) -> Navigation {
        let alert_config = AlertConfig::new(
            title,
            message,
            Box::new(GlobalMessage::Home(Message::Alert(None))),
        )
        .with_additional_buttons(additional_buttons);

        let task = Task::perform(
            async { GlobalMessage::Home(Message::Alert(Some(alert_config))) },
            |m| m,
        );

        Navigation::None(task)
    }
}

impl Page for Home {
    fn update(&mut self, message: GlobalMessage) -> Navigation {
        let GlobalMessage::Home(msg) = message else {
            return Navigation::None(Task::none());
        };

        match msg {
            Message::About(show) => {
                self.state.about_shown = show;
                Navigation::None(Task::none())
            }
            Message::ToggleSearchEnabled => {
                self.state.search_enabled = !self.state.search_enabled;

                if self.state.search_enabled {
                    Navigation::None(text_input::focus(SEARCH_BAR_INPUT_FIELD_ID))
                } else {
                    Navigation::None(Task::none())
                }
            }
            Message::SearchInputChanged(value) => {
                self.state.search_input_text = value;

                Navigation::None(Task::none())
            }
            Message::New => {
                let entity = FilesystemMountDefinition::default();
                let page = Record::new(self.manager.clone(), entity, false);

                Navigation::GoTo(Box::new(page), Task::none())
            }
            Message::Mount(definition) => {
                self.state.mounting_in_progress_for_fs_id = Some(definition.id.clone());

                Navigation::None(Task::perform(
                    async { GlobalMessage::Home(Message::DoMount(definition)) },
                    |m| m,
                ))
            }
            Message::DoMount(definition) => {
                // This is intentionally not done asynchronously,
                // because we want it to happen in the foreground and potentially ask the user
                // for their password or SSH key passphrase.
                // If we background it, we can do none of that.

                let result = self.manager.mount(&definition);

                let error = match result {
                    Ok(_) => None,
                    Err(e) => Some((e.to_string(), format!("{:?}", e))),
                };

                Navigation::None(Task::perform(
                    async { GlobalMessage::Home(Message::MountResult(definition, error)) },
                    |m| m,
                ))
            }
            Message::MountResult(definition, error) => {
                self.state.mounting_in_progress_for_fs_id = None;

                let Some((error_human, error_debug)) = error else {
                    // On success, proceed with mounting the scheduled ones

                    let Some(next_definition) = self
                        .state
                        .mounting_scheduled_for_definitions
                        .first()
                        .cloned()
                    else {
                        return Navigation::None(Task::none());
                    };

                    self.state.mounting_scheduled_for_definitions.remove(0);

                    let next_task = Task::perform(
                        async { Message::Mount(next_definition) },
                        GlobalMessage::Home,
                    );

                    return Navigation::None(next_task);
                };

                let mount_command = match definition.mount_commands() {
                    Ok(commands) => {
                        let command_strings: Vec<String> =
                            commands.iter().map(|cmd| format!("{:?}", cmd)).collect();
                        Some(command_strings.join(" && "))
                    }
                    Err(_) => None,
                };

                let error_message =
                    strings::mount_failed_alert_message(&mount_command, &error_human, &error_debug);

                let mut additional_buttons = vec![];
                if let Some(mount_command) = mount_command {
                    additional_buttons.push(create_copy_command_button(mount_command));
                }
                additional_buttons.push(create_copy_error_button(error_debug));

                let mut alert_config = AlertConfig::new(
                    strings::mount_failed_alert_title(&definition.id),
                    error_message,
                    Box::new(GlobalMessage::Home(Message::Alert(None))),
                )
                .with_width(MOUNT_ERROR_MODAL_WIDTH)
                .with_additional_buttons(additional_buttons);

                #[cfg(feature = "icons")]
                {
                    alert_config = alert_config.with_svg_icon_handle(svg::Handle::from_memory(
                        crate::assets::bootstrap_icons::EXCLAMATION_TRIANGLE,
                    ));
                }

                let task = Task::perform(
                    async { GlobalMessage::Home(Message::Alert(Some(alert_config))) },
                    |m| m,
                );

                Navigation::None(task)
            }
            Message::Unmount(definition) => {
                self.state.unmounting_in_progress_for_fs_id = Some(definition.id.clone());

                Navigation::None(Task::perform(
                    async { GlobalMessage::Home(Message::DoUnmount(definition)) },
                    |m| m,
                ))
            }
            Message::DoUnmount(definition) => {
                let result = self.manager.umount(&definition);

                let error = match result {
                    Ok(_) => None,
                    Err(e) => Some((e.to_string(), format!("{:?}", e))),
                };

                Navigation::None(Task::perform(
                    async { GlobalMessage::Home(Message::UnmountResult(definition, error)) },
                    |m| m,
                ))
            }
            Message::UnmountResult(definition, result) => {
                self.state.unmounting_in_progress_for_fs_id = None;

                let Some((error_human, error_debug)) = result else {
                    // On success, proceed with unmounting the scheduled ones

                    let Some(next_definition) = self
                        .state
                        .unmounting_scheduled_for_definitions
                        .first()
                        .cloned()
                    else {
                        return Navigation::None(Task::none());
                    };

                    self.state.unmounting_scheduled_for_definitions.remove(0);

                    let next_task = Task::perform(
                        async { Message::Unmount(next_definition) },
                        GlobalMessage::Home,
                    );

                    return Navigation::None(next_task);
                };

                let unmount_command = match definition.umount_commands() {
                    Ok(commands) => {
                        let command_strings: Vec<String> =
                            commands.iter().map(|cmd| format!("{:?}", cmd)).collect();
                        Some(command_strings.join(" && "))
                    }
                    Err(_) => None,
                };

                let error_message = strings::unmount_failed_alert_message(
                    &unmount_command,
                    &error_human,
                    &error_debug,
                );

                let mut additional_buttons = vec![];
                if let Some(unmount_command) = unmount_command {
                    additional_buttons.push(create_copy_command_button(unmount_command));
                }
                additional_buttons.push(create_copy_error_button(error_debug));

                let mut alert_config = AlertConfig::new(
                    strings::unmount_failed_alert_title(&definition.id),
                    error_message,
                    Box::new(GlobalMessage::Home(Message::Alert(None))),
                )
                .with_width(MOUNT_ERROR_MODAL_WIDTH)
                .with_additional_buttons(additional_buttons);

                #[cfg(feature = "icons")]
                {
                    alert_config = alert_config.with_svg_icon_handle(svg::Handle::from_memory(
                        crate::assets::bootstrap_icons::EXCLAMATION_TRIANGLE,
                    ));
                }

                let task = Task::perform(
                    async { GlobalMessage::Home(Message::Alert(Some(alert_config))) },
                    |m| m,
                );

                Navigation::None(task)
            }
            Message::Open(definition) => {
                let result = self.manager.open(&definition);

                if let Err(e) = result {
                    let error_message = strings::open_failed_alert_message(&e.to_string());

                    return self.alert(
                        strings::open_failed_alert_title(&definition.id),
                        error_message,
                        vec![],
                    );
                }

                Navigation::None(Task::none())
            }
            Message::ToggleOptionsForRecord(definition) => {
                self.state.expand_options_for_fs_id = if let Some(definition) = definition {
                    Some(definition.id)
                } else {
                    None
                };

                self.state.confirmation = None;

                Navigation::None(Task::none())
            }
            Message::EditRecord(definition) => {
                // Close the options menu
                self.state.expand_options_for_fs_id = None;

                Navigation::GoTo(
                    Box::new(Record::new(self.manager.clone(), definition, true)),
                    Task::none(),
                )
            }
            Message::CloneRecord(definition) => {
                // Close the options menu
                self.state.expand_options_for_fs_id = None;

                let mut entity = definition.clone();
                entity.id = format!("{}-clone", entity.id);

                Navigation::GoTo(
                    Box::new(Record::new(self.manager.clone(), entity, false)),
                    Task::none(),
                )
            }
            Message::RemoveRecord(definition) => {
                let mut btn_remove =
                    Button::new(strings::remove_confirmation_confirmation_button_label())
                        .with_style(ButtonStyle::Danger);

                #[cfg(feature = "icons")]
                {
                    btn_remove = btn_remove.with_svg_icon_handle(svg::Handle::from_memory(
                        crate::assets::bootstrap_icons::TRASH,
                    ));
                }

                let mut btn_cancel =
                    Button::new(strings::remove_confirmation_cancellation_button_label())
                        .with_style(ButtonStyle::SecondaryOutlined);

                #[cfg(feature = "icons")]
                {
                    btn_cancel = btn_cancel.with_svg_icon_handle(svg::Handle::from_memory(
                        crate::assets::bootstrap_icons::ARROW_RETURN_LEFT,
                    ));
                }

                let confirmation = ConfirmationConfig::new(
                    strings::remove_confirmation_title(),
                    strings::remove_confirmation_message(&definition.id),
                    Box::new(GlobalMessage::Home(Message::RemoveRecordConfirmed(
                        definition,
                    ))),
                    Box::new(GlobalMessage::Home(Message::RemoveRecordCancelled)),
                )
                .with_confirmation_button(btn_remove)
                .with_cancellation_button(btn_cancel);

                Navigation::None(Task::perform(
                    async { GlobalMessage::Home(Message::Confirmation(Some(confirmation))) },
                    |m| m,
                ))
            }
            Message::RemoveRecordCancelled => {
                // Close the confirmation dialog
                self.state.confirmation = None;

                // Intentionally not closing the options menu here.
                // The user may wish to use another option.

                Navigation::None(Task::none())
            }
            Message::RemoveRecordConfirmed(definition) => {
                // Close the confirmation dialog
                self.state.confirmation = None;

                // Close the options menu
                self.state.expand_options_for_fs_id = None;

                let result = self.manager.remove(&definition);

                let Err(e) = result else {
                    return Navigation::None(Task::none());
                };

                let error_message = strings::remove_failed_alert_message(&e.to_string());

                let mut alert_config = AlertConfig::new(
                    strings::remove_failed_alert_title(&definition.id),
                    error_message,
                    Box::new(GlobalMessage::Home(Message::Alert(None))),
                );

                #[cfg(feature = "icons")]
                {
                    alert_config = alert_config.with_svg_icon_handle(svg::Handle::from_memory(
                        crate::assets::bootstrap_icons::EXCLAMATION_TRIANGLE,
                    ));
                }

                let task = Task::perform(
                    async { GlobalMessage::Home(Message::Alert(Some(alert_config))) },
                    |m| m,
                );

                Navigation::None(task)
            }

            Message::MountAll => {
                // This performs synchronous mounting of all unmounted filesystems, by:
                // - returning a task that immediately mounts the first one
                // - recording all others in `mounting_scheduled_for_definitions` for later mounting
                //
                // `mounting_scheduled_for_definitions` is processed after successful mounting in `Message::MountResult`
                //
                // We intentionally mount synchronously (in the foreground) to be able to ask for SSH key passphrases, etc.

                let state_filtered_unmounted_only = self.filesystems_filtered()
                    .into_iter()
                    .filter(|item| !item.mounted)
                    .collect::<Vec<_>>();

                let Some(first_definition) = state_filtered_unmounted_only.first() else {
                    return Navigation::None(Task::none());
                };

                let first_def = first_definition.definition.clone();
                let task_mount_first =
                    Task::perform(async { Message::Mount(first_def) }, GlobalMessage::Home);

                // Schedule all others
                for entity in state_filtered_unmounted_only.iter().skip(1) {
                    self.state
                        .mounting_scheduled_for_definitions
                        .push(entity.definition.clone());
                }

                Navigation::None(task_mount_first)
            }
            Message::UnmountAll => {
                // This performs synchronous unmounting of all mounted filesystems, by:
                // - returning a task that immediately unmounts the first one
                // - recording all others in `unmounting_scheduled_for_definitions` for later unmounting
                //
                // `unmounting_scheduled_for_definitions` is processed after successful unmounting in `Message::UnmountResult`
                //
                // We intentionally unmount synchronously (in the foreground) to be able to ask for SSH key passphrases, etc.
                // If we background it, we can do none of that.

                let state_filtered_mounted_only = self.filesystems_filtered()
                    .into_iter()
                    .filter(|item| item.mounted)
                    .collect::<Vec<_>>();

                let Some(first_definition) = state_filtered_mounted_only.first() else {
                    return Navigation::None(Task::none());
                };

                let first_def = first_definition.definition.clone();
                let task_unmount_first =
                    Task::perform(async { Message::Unmount(first_def) }, GlobalMessage::Home);

                // Schedule all others
                for entity in state_filtered_mounted_only.iter().skip(1) {
                    self.state
                        .unmounting_scheduled_for_definitions
                        .push(entity.definition.clone());
                }

                Navigation::None(task_unmount_first)
            }
            Message::Confirmation(confirmation) => {
                self.state.confirmation = confirmation;
                Navigation::None(Task::none())
            }
            Message::Alert(alert) => {
                self.state.alert = alert;
                Navigation::None(Task::none())
            }
            Message::PeriodicRefreshTicked(_instant) => {
                // The fact that this message got triggered is enough to cause a re-render of whatever page we're on.
                // There's no need to do anything here.
                Navigation::None(Task::none())
            }
            Message::RunPreflightCheck => {
                log::info!("Running preflight check");

                if let Err(errors) = self.manager.preflight_check() {
                    self.state.preflight_check_errors = errors
                        .into_iter()
                        .map(|e| format!("{}: {:?}", e, e))
                        .collect::<Vec<String>>();
                } else {
                    self.state.preflight_check_errors = Vec::new();
                }

                Navigation::None(Task::none())
            }
        }
    }

    fn view(&self, theme: &iced::theme::Theme) -> iced::Element<GlobalMessage> {
        let container = self.render(theme);

        if let Some(payload) = &self.state.confirmation {
            confirmation(container, payload)
        } else if let Some(payload) = &self.state.alert {
            alert(container, payload)
        } else if self.state.about_shown {
            about(container, theme)
        } else {
            container.into()
        }
    }

    fn subscription(&self) -> Subscription<GlobalMessage> {
        // This refreshes the Home page every X seconds, so that we can show up-to-date information about what's mounted.
        // The About page is shown as a dialog on top of the home page, so we trigger a refresh on it as well.
        let periodic_refresh_tick = time::every(Duration::from_millis(REFRESH_INTERVAL_MS))
            .map(|instant| GlobalMessage::Home(Message::PeriodicRefreshTicked(instant)));

        fn handle_hotkey(
            key: keyboard::Key,
            modifiers: keyboard::Modifiers,
        ) -> Option<GlobalMessage> {
            use keyboard::key;

            match key {
                key::Key::Named(_) => None,
                key::Key::Character(c) => {
                    if (c.as_str() == "f" || c.as_str() == "k") && modifiers.command() {
                        Some(GlobalMessage::Home(Message::ToggleSearchEnabled))
                    } else {
                        None
                    }
                }
                key::Key::Unidentified => None,
            }
        }

        Subscription::batch(vec![
            periodic_refresh_tick,
            keyboard::on_key_press(handle_hotkey),
        ])
    }
}

fn create_copy_command_button(command: String) -> Button {
    let mut button = Button::new(strings::mount_unmount_failed_button_copy_command_label())
        .with_style(ButtonStyle::SecondaryOutlined)
        .with_on_press(Some(GlobalMessage::Application(
            ApplicationMessage::PutContentInClipboard(command),
        )));

    #[cfg(feature = "icons")]
    {
        button = button.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::CLIPBOARD_CHECK,
        ));
    }

    button
}

fn create_copy_error_button(error: String) -> Button {
    let mut button = Button::new(strings::mount_unmount_failed_button_copy_error_label())
        .with_style(ButtonStyle::SecondaryOutlined)
        .with_on_press(Some(GlobalMessage::Application(
            ApplicationMessage::PutContentInClipboard(error.clone()),
        )));

    #[cfg(feature = "icons")]
    {
        button = button.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::BUG,
        ));
    }

    button
}
