pub mod default;
pub mod keybindings;
pub mod main_menu;
pub mod settings;

pub use default::default_view_keybindings;
pub use keybindings::{
    Action, Group, KEYBINDINGS, KeyBinding, binding_for, binding_for_view, event_to_action,
};
pub use main_menu::main_menu_keybindings;
pub use settings::settings_keybindings;
