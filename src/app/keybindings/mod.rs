pub mod keybindings;
pub mod main_menu;

pub use keybindings::{
    Action, Group, KEYBINDINGS, KeyBinding, binding_for, binding_for_view, event_to_action,
};
pub use main_menu::main_menu_keybindings;
