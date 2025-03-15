use iced::Alignment::Center;
use iced::widget::{
    self, Column, Container, Row, Space, column, container, horizontal_space, pick_list, row,
    scrollable, text, text_input,
};
use iced::{Element, Length, Padding, Subscription, Task, keyboard};
use iced_aw::number_input;

#[cfg(feature = "icons")]
use iced::widget::svg;

use libsftpman::validator::Validate;
use libsftpman::{AuthType, FilesystemMountDefinition, Manager};

use crate::application::{Navigation, Page};
use crate::messages::Message as GlobalMessage;
use crate::pages::{AlertConfig, alert};
use crate::strings;
use crate::ui_config::{
    ICON_SIZE, ICON_TO_TEXT_SPACING, RECORD_LABEL_WIDTH, SCROLLBAR_RESERVED_SPACE,
    WIDGET_HORIZONTAL_SPACING, WIDGET_VERTICAL_SPACING,
};
use crate::utils::errors_to_string_list;
use crate::widgets::{Button, ButtonStyle, field_control_label, text_link};

#[cfg(feature = "icons")]
use crate::widgets::{IconColor, icon};

#[derive(Debug, Clone)]
pub enum Message {
    Save,
    Cancel,

    DefinitionUpdated(FilesystemMountDefinition),
    PortChanged(u16),

    #[cfg(feature = "file-picker")]
    BrowseSshKeyTriggered(State),

    KeyboardTabPressed {
        shift: bool,
    },

    Alert(Option<AlertConfig>),
}

#[derive(Clone, Debug)]
pub struct State {
    is_existing: bool,
    is_mounted: bool,
    original_definition: FilesystemMountDefinition,

    definition: FilesystemMountDefinition,

    alert: Option<AlertConfig>,
}

impl State {
    pub fn new(definition: FilesystemMountDefinition, is_existing: bool, is_mounted: bool) -> Self {
        Self {
            is_existing,
            is_mounted,
            original_definition: definition.clone(),
            definition,
            alert: None,
        }
    }

    fn with_definition(mut self, val: FilesystemMountDefinition) -> Self {
        self.definition = val;
        self
    }

    fn definition(&self) -> &FilesystemMountDefinition {
        &self.definition
    }
}

pub struct Record {
    manager: Manager,
    state: State,
}

impl Record {
    pub fn new(manager: Manager, definition: FilesystemMountDefinition, is_existing: bool) -> Self {
        let is_mounted = manager.is_definition_mounted(&definition).unwrap_or(false);

        Self {
            manager,
            state: State::new(definition, is_existing, is_mounted),
        }
    }

    fn post_process_definition(
        &self,
        definition: FilesystemMountDefinition,
    ) -> FilesystemMountDefinition {
        let mut definition = definition.clone();

        definition.mount_options = definition
            .mount_options
            .iter()
            .map(|s| {
                s.trim_matches(|c: char| c.is_whitespace() || c == ',')
                    .to_string()
            })
            .filter(|s| !s.is_empty())
            .collect();

        definition
    }

    fn save(&self) -> Navigation {
        let definition = self.state.definition();

        if let Err(errors) = definition.validate() {
            let errors_list = errors_to_string_list(errors);
            let error_message = strings::save_failed_alert_validation_failed_message(
                &errors_list
                    .iter()
                    .map(|e| format!("- {}", e))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );

            return self.alert(
                strings::save_failed_alert_validation_failed_title(),
                error_message,
            );
        }

        // Check for ID uniqueness when:
        // - adding a brand new definition
        // - changing the id of an existing definition
        let check_for_id_uniqueness = if self.state.is_existing {
            self.state.definition.id != self.state.original_definition.id
        } else {
            true
        };

        if check_for_id_uniqueness {
            // This is not perfect. Some errors (failing to read files or parse a definition)
            // may be interpretted as "no existing definition".
            if self.manager.definition(&self.state.definition.id).is_ok() {
                return self.alert(
                    strings::save_failed_id_check_failed_title(),
                    strings::save_failed_id_uniqueness_check_failed_message(
                        &self.state.definition.id,
                    ),
                );
            };
        }

        let id_changed = if self.state.is_existing {
            self.state.definition.id != self.state.original_definition.id
        } else {
            false
        };

        let is_mounted_before_save = if self.state.is_existing {
            self.manager
                .is_definition_mounted(&self.state.original_definition)
                .unwrap_or(false)
        } else {
            false
        };

        if is_mounted_before_save {
            let unmount_result = self.manager.umount(&self.state.original_definition);
            if let Err(err) = unmount_result {
                return self.alert(
                    strings::operation_failed_alert_title(),
                    strings::operation_failed_alert_message(&err.to_string()),
                );
            }
        }

        if let Err(e) = self.manager.persist(definition) {
            return self.alert(
                strings::save_failed_alert_persistence_failed_title(),
                strings::save_failed_alert_persistence_failed_message(&e.to_string()),
            );
        }

        if id_changed {
            let remove_result = self.manager.remove(&self.state.original_definition);
            if let Err(err) = remove_result {
                return self.alert(
                    strings::operation_failed_alert_title(),
                    strings::operation_failed_alert_message(&err.to_string()),
                );
            }
        }

        if is_mounted_before_save {
            let mount_result = self.manager.mount(&self.state.definition);
            if let Err(err) = mount_result {
                return self.alert(
                    strings::operation_failed_alert_title(),
                    strings::operation_failed_alert_message(&err.to_string()),
                );
            }
        }

        Navigation::Back(Task::none())
    }

    fn alert(&self, title: String, message: String) -> Navigation {
        let alert_config = AlertConfig::new(
            title,
            message,
            Box::new(GlobalMessage::Record(Message::Alert(None))),
        );

        let task = Task::perform(
            async { GlobalMessage::Record(Message::Alert(Some(alert_config))) },
            |m| m,
        );

        Navigation::None(task)
    }

    fn render(&self, theme: &iced::theme::Theme) -> Container<'static, GlobalMessage> {
        // We expand this to fill the entire window height, because alert/confirmation modals
        // are shown on top of it and a small base container would not result in a well-visible modal.
        let row_scrollable_main_column = scrollable(row![
            record(self.state.clone(), theme),
            Space::with_width(SCROLLBAR_RESERVED_SPACE)
        ])
        .height(Length::Fill);

        let footer_column = footer(self.state.is_mounted, theme);

        container(column![row_scrollable_main_column, footer_column])
    }
}

impl Page for Record {
    fn update(&mut self, message: GlobalMessage) -> Navigation {
        let GlobalMessage::Record(msg) = message else {
            return Navigation::None(Task::none());
        };

        match msg {
            Message::KeyboardTabPressed { shift } => {
                if shift {
                    Navigation::None(widget::focus_previous())
                } else {
                    Navigation::None(widget::focus_next())
                }
            }
            Message::DefinitionUpdated(definition) => {
                self.state = self.state.clone().with_definition(definition);

                Navigation::None(Task::none())
            }
            #[cfg(feature = "file-picker")]
            Message::BrowseSshKeyTriggered(state) => {
                let rt = tokio::runtime::Runtime::new().unwrap();

                let mut file = None;

                rt.block_on(async {
                    file = tokio::task::block_in_place(|| {
                        rfd::FileDialog::new().set_directory("~").pick_file()
                    });
                });

                if let Some(file) = file {
                    let mut def = state.definition.clone();
                    def.ssh_key = file.as_os_str().to_str().unwrap().to_owned();

                    let state = state.with_definition(def);

                    self.state = state;
                };

                Navigation::None(Task::none())
            }
            Message::PortChanged(port) => {
                let mut def = self.state.definition.clone();
                def.port = port;

                self.state = self.state.clone().with_definition(def);

                Navigation::None(Task::none())
            }
            Message::Alert(alert) => {
                self.state.alert = alert;
                Navigation::None(Task::none())
            }
            Message::Save => {
                self.state.definition = self.post_process_definition(self.state.definition.clone());

                self.save()
            }
            Message::Cancel => Navigation::Back(Task::none()),
        }
    }

    fn view(&self, theme: &iced::theme::Theme) -> iced::Element<GlobalMessage> {
        let container = self.render(theme);

        if let Some(payload) = &self.state.alert {
            alert(container, payload)
        } else {
            container.into()
        }
    }

    fn subscription(&self) -> Subscription<GlobalMessage> {
        fn handle_hotkey(
            key: keyboard::Key,
            modifiers: keyboard::Modifiers,
        ) -> Option<GlobalMessage> {
            use keyboard::key;

            match key {
                key::Key::Named(named) => match (named, modifiers) {
                    (key::Named::Tab, _) => {
                        Some(GlobalMessage::Record(Message::KeyboardTabPressed {
                            shift: modifiers.shift(),
                        }))
                    }
                    _ => None,
                },
                key::Key::Character(_) => None,
                key::Key::Unidentified => None,
            }
        }

        Subscription::batch(vec![keyboard::on_key_press(handle_hotkey)])
    }
}

fn record(state: State, theme: &iced::theme::Theme) -> Container<'static, GlobalMessage> {
    let definition = state.definition().clone();

    let def = definition.clone();
    let id_text_input =
        text_input(&strings::record_id_placeholder(), &definition.id).on_input(move |v| {
            let mut definition_clone = def.clone();
            definition_clone.id = v;

            GlobalMessage::Record(Message::DefinitionUpdated(definition_clone))
        });

    let row_id = row![
        field_control_label(strings::record_id_label(), true, theme).width(RECORD_LABEL_WIDTH),
        id_text_input,
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    let def = definition.clone();
    let row_host = row![
        field_control_label(strings::record_host_label(), true, theme).width(RECORD_LABEL_WIDTH),
        text_input(&strings::record_host_placeholder(), &definition.host).on_input(move |v| {
            let mut definition_clone = def.clone();
            definition_clone.host = v;
            GlobalMessage::Record(Message::DefinitionUpdated(definition_clone))
        })
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    let def = definition.clone();
    let row_port = row![
        field_control_label(strings::record_port_label(), true, theme).width(RECORD_LABEL_WIDTH),
        number_input(&def.port, 1..65535, |v| {
            GlobalMessage::Record(Message::PortChanged(v))
        })
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    let def = definition.clone();
    let row_username = row![
        field_control_label(strings::record_username_label(), true, theme)
            .width(RECORD_LABEL_WIDTH),
        text_input(&strings::record_username_placeholder(), &def.user).on_input(move |v| {
            let mut definition_clone = def.clone();
            definition_clone.user = v;
            GlobalMessage::Record(Message::DefinitionUpdated(definition_clone))
        })
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    let def = definition.clone();
    let row_auth_type = row![
        field_control_label(strings::record_auth_type_label(), true, theme)
            .width(RECORD_LABEL_WIDTH),
        pick_list(&AuthType::ALL[..], Some(def.auth_type.clone()), move |v| {
            let mut definition_clone = def.clone();
            definition_clone.auth_type = v;

            if definition_clone.auth_type != AuthType::PublicKey {
                definition_clone.ssh_key = "".to_owned();
            }

            GlobalMessage::Record(Message::DefinitionUpdated(definition_clone))
        }),
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    let row_ssh_key: Element<'static, GlobalMessage> = if definition.auth_type
        == AuthType::PublicKey
    {
        let def = definition.clone();

        let ssh_key_text_input = text_input(&strings::record_ssh_key_placeholder(), &def.ssh_key)
            .on_input(move |v| {
                let mut definition_clone = def.clone();
                definition_clone.ssh_key = v;

                GlobalMessage::Record(Message::DefinitionUpdated(definition_clone))
            });

        let mut row_fields =
            Row::with_children(vec![ssh_key_text_input.into()]).spacing(WIDGET_HORIZONTAL_SPACING);

        #[cfg(feature = "file-picker")]
        {
            let mut btn_pick_ssh_key = Button::new(strings::browse_label())
                .with_style(ButtonStyle::Secondary)
                .with_on_press(Some(GlobalMessage::Record(Message::BrowseSshKeyTriggered(
                    state,
                ))));

            #[cfg(feature = "icons")]
            {
                btn_pick_ssh_key = btn_pick_ssh_key.with_svg_icon_handle(svg::Handle::from_memory(
                    crate::assets::bootstrap_icons::FOLDER2_OPEN,
                ));
            }

            row_fields = row_fields.push(btn_pick_ssh_key.build());
        }

        row![
            field_control_label(strings::record_ssh_key_label(), true, theme)
                .width(RECORD_LABEL_WIDTH),
            row_fields,
        ]
        .spacing(WIDGET_HORIZONTAL_SPACING)
        .into()
    } else {
        container("").height(0).into()
    };

    let def = definition.clone();
    let row_remote_path = row![
        field_control_label(strings::record_remote_path_label(), true, theme)
            .width(RECORD_LABEL_WIDTH),
        text_input(
            &strings::record_remote_path_placeholder(),
            &def.remote_path.clone(),
        )
        .on_input(move |v| {
            let mut definition_clone = def.clone();

            definition_clone.remote_path = v;

            GlobalMessage::Record(Message::DefinitionUpdated(definition_clone))
        }),
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    let def = definition.clone();
    let row_mount_dest_path = row![
        field_control_label(strings::record_mount_dest_path_label(), false, theme)
            .width(RECORD_LABEL_WIDTH),
        text_input(
            &strings::record_mount_dest_path_placeholder(&def.id),
            &def.mount_dest_path.clone().unwrap_or_default(),
        )
        .on_input(move |v| {
            let mut definition_clone = def.clone();

            if v.is_empty() {
                definition_clone.mount_dest_path = None;
            } else {
                definition_clone.mount_dest_path = Some(v);
            }

            GlobalMessage::Record(Message::DefinitionUpdated(definition_clone))
        }),
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    let def = definition.clone();

    let sshfs_options_text_input = text_input(
        &strings::record_sshfs_options_placeholder(),
        &def.mount_options.join(", "),
    )
    .on_input(move |v| {
        let mut definition_clone = def.clone();
        definition_clone.mount_options = if v.trim().is_empty() {
            vec![]
        } else {
            // Intentionally not trimming, to allow the user to enter spaces.
            // We do post-processing in `post_process_definition()`.
            v.split(", ").map(|s| s.to_string()).collect()
        };

        GlobalMessage::Record(Message::DefinitionUpdated(definition_clone))
    });

    let row_sshfs_options = row![
        field_control_label(strings::record_sshfs_options_label(), false, theme)
            .width(RECORD_LABEL_WIDTH),
        column![sshfs_options_text_input, row_sshfs_options_help_text()]
            .spacing(WIDGET_VERTICAL_SPACING),
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    let def = definition.clone();
    let row_command_before_mount = row![
        field_control_label(strings::record_command_before_mount_label(), false, theme)
            .width(RECORD_LABEL_WIDTH),
        text_input(
            &strings::record_command_before_mount_placeholder(),
            &def.cmd_before_mount
        )
        .on_input(move |v| {
            let mut definition_clone = def.clone();
            definition_clone.cmd_before_mount = v;
            GlobalMessage::Record(Message::DefinitionUpdated(definition_clone))
        })
    ]
    .spacing(WIDGET_HORIZONTAL_SPACING);

    container(
        column![
            row_id,
            row_host,
            row_port,
            row_username,
            row_auth_type,
            row_ssh_key,
            row_remote_path,
            row_mount_dest_path,
            row_sshfs_options,
            row_command_before_mount,
        ]
        .spacing(WIDGET_VERTICAL_SPACING)
        .padding(
            Padding::new(0.0)
                .top(WIDGET_VERTICAL_SPACING)
                .bottom(WIDGET_VERTICAL_SPACING)
                .left(SCROLLBAR_RESERVED_SPACE),
        ),
    )
}

fn footer(is_mounted: bool, theme: &iced::theme::Theme) -> Container<'static, GlobalMessage> {
    let mut footer_column = Column::new().spacing(WIDGET_VERTICAL_SPACING).padding(
        Padding::new(0.0)
            .left(SCROLLBAR_RESERVED_SPACE)
            .right(SCROLLBAR_RESERVED_SPACE)
            .bottom(WIDGET_VERTICAL_SPACING),
    );

    if is_mounted {
        let mut warning_row = Row::new().spacing(ICON_TO_TEXT_SPACING).align_y(Center);

        #[cfg(feature = "icons")]
        {
            let icon = icon(
                &svg::Handle::from_memory(crate::assets::bootstrap_icons::EXCLAMATION_TRIANGLE),
                ICON_SIZE,
                IconColor::Danger,
            );

            warning_row = warning_row.push(icon);
        }

        let warning_text = text(strings::record_mounted_will_be_remounted_warning_label())
            .color(theme.palette().danger);

        warning_row = warning_row.push(warning_text);

        footer_column = footer_column.push(warning_row);
    }

    let row_controls = row_controls();

    footer_column = footer_column.push(row_controls);

    container(footer_column)
}

fn row_controls() -> Row<'static, GlobalMessage> {
    let mut btn_save = Button::new(strings::record_save_label())
        .with_style(ButtonStyle::Primary)
        .with_on_press(Some(GlobalMessage::Record(Message::Save)));

    #[cfg(feature = "icons")]
    {
        btn_save = btn_save.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::CHECK_CIRCLE,
        ));
    }

    let mut btn_cancel = Button::new(strings::record_cancel_label())
        .with_style(ButtonStyle::Secondary)
        .with_on_press(Some(GlobalMessage::Record(Message::Cancel)));

    #[cfg(feature = "icons")]
    {
        btn_cancel = btn_cancel.with_svg_icon_handle(svg::Handle::from_memory(
            crate::assets::bootstrap_icons::ARROW_RETURN_LEFT,
        ));
    }

    row![btn_save.build(), horizontal_space(), btn_cancel.build()]
}

fn row_sshfs_options_help_text() -> Row<'static, GlobalMessage> {
    let mut sshfs_options_help_text_row = row![]
        .align_y(iced::Alignment::Center)
        .spacing(ICON_TO_TEXT_SPACING);

    #[cfg(feature = "icons")]
    {
        let icon_text = icon(
            &svg::Handle::from_memory(crate::assets::bootstrap_icons::QUESTION_CIRCLE),
            ICON_SIZE,
            IconColor::Primary,
        );
        sshfs_options_help_text_row = sshfs_options_help_text_row.push(icon_text);
    }

    sshfs_options_help_text_row = sshfs_options_help_text_row.push(text_link(
        strings::record_sshfs_options_help_text(),
        "https://www.man7.org/linux/man-pages/man1/sshfs.1.html#OPTIONS".to_owned(),
    ));

    sshfs_options_help_text_row
}
