use crate::app::{
    App, View,
    keybindings::{Action, event_to_action},
    states::SettingsValue,
};
use ratatui::crossterm::event::KeyEvent;
use std::any::type_name_of_val;

pub fn settings_keybindings(app: &mut App, key_event: &KeyEvent) {
    if let Some(action) = event_to_action(&key_event) {
        match action {
            Action::Up => {
                app.states.settings.selected_setting = app
                    .states
                    .settings
                    .selected_setting
                    .saturating_sub(1)
                    .max(0)
            }
            Action::Down => {
                app.states.settings.selected_setting = app
                    .states
                    .settings
                    .selected_setting
                    .saturating_add(1)
                    .min(9)
            }
            Action::Esc => app.view = View::MainMenu,
            Action::Space => app.states.settings.show_popup = !app.states.settings.show_popup,
            Action::Enter => {
                // FIXME: No feedback
                if app.states.settings.show_popup {
                    let option =
                        &app.states.settings.options[app.states.settings.selected_setting].value;

                    let option_type: &'static str = type_name_of_val(option);

                    let new_value: String = app.states.settings.selected_setting_new_value.clone();

                    match option_type {
                        "u8" => {
                            if let Ok(value) = new_value.parse::<u8>() {
                                app.states.settings.options[app.states.settings.selected_setting]
                                    .value = SettingsValue::U8(value);

                                app.states.settings.selected_setting_new_value = "".to_string();
                                app.states.settings.error_message = "".to_string();
                                app.states.settings.show_popup = false;
                            } else {
                                app.states.settings.error_message = "Invalid value".to_string();
                            }
                        }
                        "f32" => {
                            if let Ok(value) = new_value.parse::<f32>() {
                                app.states.settings.options[app.states.settings.selected_setting]
                                    .value = SettingsValue::F32(value);

                                app.states.settings.selected_setting_new_value = "".to_string();
                                app.states.settings.error_message = "".to_string();
                                app.states.settings.show_popup = false;
                            } else {
                                app.states.settings.error_message = "Invalid value".to_string();
                            }
                        }
                        "u16" => {
                            if let Ok(value) = new_value.parse::<u16>() {
                                app.states.settings.options[app.states.settings.selected_setting]
                                    .value = SettingsValue::U16(value);

                                app.states.settings.selected_setting_new_value = "".to_string();
                                app.states.settings.error_message = "".to_string();
                                app.states.settings.show_popup = false;
                            } else {
                                app.states.settings.error_message = "Invalid value".to_string();
                            }
                        }
                        "std::string::String" => {
                            app.states.settings.options[app.states.settings.selected_setting]
                                .value = SettingsValue::Text(new_value);

                            app.states.settings.selected_setting_new_value = "".to_string();
                            app.states.settings.error_message = "".to_string();
                            app.states.settings.show_popup = false;
                        }
                        _ => (),
                    }
                }
            }
            Action::Backspace => {
                if app.states.settings.show_popup {
                    app.states.settings.selected_setting_new_value.pop();
                }
            }
            Action::WildCard(c) => {
                if app.states.settings.show_popup {
                    app.states.settings.selected_setting_new_value.push(c);
                }
            }
            Action::Quit => app.exit = true,
            Action::Quit2 => app.exit = true,
            // _ => (),
        }
    }
}
