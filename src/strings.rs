pub fn control_bar_new_label() -> String {
    t!("control_bar_new_label").to_string()
}

pub fn control_bar_new_tooltip() -> String {
    t!("control_bar_new_tooltip").to_string()
}

pub fn control_bar_mount_all_label() -> String {
    t!("control_bar_mount_all_label").to_string()
}

pub fn control_bar_mount_all_tooltip() -> String {
    t!("control_bar_mount_all_tooltip").to_string()
}

pub fn control_bar_unmount_all_label() -> String {
    t!("control_bar_unmount_all_label").to_string()
}

pub fn control_bar_unmount_all_tooltip() -> String {
    t!("control_bar_unmount_all_tooltip").to_string()
}

pub fn control_bar_search_label() -> String {
    t!("control_bar_search_label").to_string()
}

pub fn control_bar_search_tooltip() -> String {
    t!("control_bar_search_tooltip").to_string()
}

pub fn search_input_placeholder() -> String {
    t!("search_input_placeholder").to_string()
}

pub fn control_bar_about_label() -> String {
    t!("control_bar_about_label").to_string()
}

pub fn fs_list_open_label() -> String {
    t!("fs_list_open_label").to_string()
}

pub fn fs_list_options_label() -> String {
    t!("fs_list_options_label").to_string()
}

pub fn fs_list_empty_list_label() -> String {
    t!("fs_list_empty_list_label").to_string()
}

pub fn record_mounted_will_be_remounted_warning_label() -> String {
    t!("record_mounted_will_be_remounted_warning_label").to_string()
}

pub fn record_id_label() -> String {
    t!("record_id_label").to_string()
}

pub fn record_id_placeholder() -> String {
    t!("record_id_placeholder").to_string()
}

pub fn record_host_label() -> String {
    t!("record_host_label").to_string()
}

pub fn record_host_placeholder() -> String {
    t!("record_host_placeholder").to_string()
}

pub fn record_port_label() -> String {
    t!("record_port_label").to_string()
}

pub fn record_username_label() -> String {
    t!("record_username_label").to_string()
}

pub fn record_username_placeholder() -> String {
    t!("record_username_placeholder").to_string()
}

pub fn record_ssh_key_label() -> String {
    t!("record_ssh_key_label").to_string()
}

pub fn record_ssh_key_placeholder() -> String {
    t!(
        "record_path_placeholder",
        path = "/home/user/.ssh/id_ed25519"
    )
    .to_string()
}

pub fn record_auth_type_label() -> String {
    t!("record_auth_type_label").to_string()
}

pub fn record_remote_path_label() -> String {
    t!("record_remote_path_label").to_string()
}

pub fn record_remote_path_placeholder() -> String {
    t!("record_path_placeholder", path = "/home/user/remote-dir").to_string()
}

pub fn record_mount_dest_path_label() -> String {
    t!("record_mount_dest_path_label").to_string()
}

pub fn record_mount_dest_path_placeholder(id: &str) -> String {
    let id_value = if id.is_empty() {
        format!("<{}>", t!("record_id_label"))
    } else {
        id.to_string()
    };

    t!("record_mount_dest_path_placeholder", id = id_value).to_string()
}

pub fn record_sshfs_options_label() -> String {
    t!("record_sshfs_options_label").to_string()
}

pub fn record_sshfs_options_placeholder() -> String {
    t!(
        "record_sshfs_options_placeholder",
        example = "follow_symlinks, workaround=rename"
    )
    .to_string()
}

pub fn record_sshfs_options_help_text() -> String {
    t!("record_sshfs_options_help_text").to_string()
}

pub fn record_command_before_mount_label() -> String {
    t!("record_command_before_mount_label").to_string()
}

pub fn record_command_before_mount_placeholder() -> String {
    t!(
        "record_command_before_mount_placeholder",
        example = "/bin/true"
    )
    .to_string()
}

pub fn record_save_label() -> String {
    t!("record_save_label").to_string()
}

pub fn record_cancel_label() -> String {
    t!("record_cancel_label").to_string()
}

pub fn browse_label() -> String {
    t!("browse_label").to_string()
}

pub fn save_failed_alert_validation_failed_title() -> String {
    t!("save_failed_alert_validation_failed_title").to_string()
}

pub fn save_failed_alert_validation_failed_message(errors: &str) -> String {
    errors.to_string()
}

pub fn save_failed_alert_persistence_failed_title() -> String {
    t!("save_failed_alert_persistence_failed_title").to_string()
}

pub fn save_failed_alert_persistence_failed_message(error: &str) -> String {
    error.to_string()
}

pub fn save_failed_id_check_failed_title() -> String {
    t!("save_failed_id_check_failed_title").to_string()
}

pub fn operation_failed_alert_title() -> String {
    t!("operation_failed_alert_title").to_string()
}

pub fn operation_failed_alert_message(error: &str) -> String {
    error.to_string()
}

pub fn save_failed_id_uniqueness_check_failed_message(id: &str) -> String {
    t!("save_failed_id_uniqueness_check_failed_message", id = id).to_string()
}

pub fn edit_button_label() -> String {
    t!("edit_button_label").to_string()
}

pub fn clone_button_label() -> String {
    t!("clone_button_label").to_string()
}

pub fn remove_button_label() -> String {
    t!("remove_button_label").to_string()
}

pub fn remove_confirmation_title() -> String {
    t!("remove_confirmation_title").to_string()
}

pub fn remove_confirmation_message(id: &str) -> String {
    t!("remove_confirmation_message", id = id).to_string()
}

pub fn remove_confirmation_confirmation_button_label() -> String {
    t!("remove_confirmation_confirmation_button_label").to_string()
}

pub fn remove_confirmation_cancellation_button_label() -> String {
    t!("remove_confirmation_cancellation_button_label").to_string()
}

pub fn remove_failed_alert_title(id: &str) -> String {
    t!("remove_failed_alert_title", id = id).to_string()
}

pub fn remove_failed_alert_message(error: &str) -> String {
    error.to_string()
}

pub fn mount_failed_alert_title(id: &str) -> String {
    t!("mount_failed_alert_title", id = id).to_string()
}

pub fn mount_failed_alert_message(
    mount_command: &Option<String>,
    error_human: &str,
    error_debug: &str,
) -> String {
    let mut err = String::new();
    err.push_str(&error_human[0..1].to_uppercase());
    err.push_str(&error_human[1..]);

    if let Some(mount_command) = mount_command {
        err.push_str(&format!(
            "\n\n{}\n    {}",
            t!("mount_failed_alert_message_mount_command"),
            mount_command
        ));
    }

    err.push_str(&format!(
        "\n\n{}\n    {}",
        t!("mount_unmount_failed_alert_message_error_debug"),
        error_debug
    ));

    err
}

pub fn mount_unmount_failed_button_copy_command_label() -> String {
    t!("mount_unmount_failed_button_copy_command_label").to_string()
}

pub fn mount_unmount_failed_button_copy_error_label() -> String {
    t!("mount_unmount_failed_button_copy_error_label").to_string()
}

pub fn unmount_failed_alert_title(id: &str) -> String {
    t!("unmount_failed_alert_title", id = id).to_string()
}

pub fn unmount_failed_alert_message(
    unmount_command: &Option<String>,
    error_human: &str,
    error_debug: &str,
) -> String {
    let mut err = String::new();
    err.push_str(&error_human[0..1].to_uppercase());
    err.push_str(&error_human[1..]);

    if let Some(unmount_command) = unmount_command {
        err.push_str(&format!(
            "\n\n{}\n    {}",
            t!("unmount_failed_alert_message_unmount_command"),
            unmount_command
        ));
    }

    err.push_str(&format!(
        "\n\n{}\n    {}",
        t!("mount_unmount_failed_alert_message_error_debug"),
        error_debug
    ));

    err
}

pub fn open_failed_alert_title(id: &str) -> String {
    t!("open_failed_alert_title", id = id).to_string()
}

pub fn open_failed_alert_message(error: &str) -> String {
    error.to_string()
}

pub fn about_program_label() -> String {
    t!("about_program_label").to_string()
}

pub fn about_program_powered_by_label() -> String {
    t!("about_program_powered_by_label").to_string()
}

pub fn about_description_label() -> String {
    t!("about_description_label").to_string()
}

pub fn about_description_message() -> String {
    t!("about_description_message").to_string()
}

pub fn about_author_label() -> String {
    t!("about_author_label").to_string()
}

pub fn about_donate_message() -> String {
    t!("about_donate_message").to_string()
}

pub fn about_donate_on_platform_button(platform: &str) -> String {
    t!("about_donate_on_platform_button", platform = platform).to_string()
}

pub fn about_license_label() -> String {
    t!("about_license_label").to_string()
}

pub fn about_button_close_label() -> String {
    t!("about_button_close_label").to_string()
}

pub fn filesystem_definition_name_mounting_label(name: &str) -> String {
    format!(
        "{} {}",
        name,
        t!(
            "filesystem_definition_name_mounting_label_mounting_suffix",
            name = name
        )
    )
}

pub fn filesystem_definition_name_unmounting_label(name: &str) -> String {
    format!(
        "{} {}",
        name,
        t!(
            "filesystem_definition_name_unmounting_label_unmounting_suffix",
            name = name
        )
    )
}

pub fn alert_close_button_label() -> String {
    t!("alert_close_button_label").to_string()
}

pub fn confirmation_confirmation_button_label() -> String {
    t!("confirmation_confirmation_button_label").to_string()
}

pub fn confirmation_cancellation_button_label() -> String {
    t!("confirmation_cancellation_button_label").to_string()
}

pub fn field_control_label_required_label() -> String {
    t!("field_control_label_required_label").to_string()
}

pub fn field_control_label_required_tooltip() -> String {
    t!("field_control_label_required_tooltip").to_string()
}

pub fn button_retry() -> String {
    t!("button_retry").to_string()
}
