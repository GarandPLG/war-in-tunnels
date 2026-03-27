pub mod default;
pub mod keybindings;
pub mod main_menu;
pub mod skirmish;

pub use default::{common_keybindings, default_keybindings};
pub use keybindings::{Action, Group, KEYBINDINGS, KeyBinding, binding_for, event_to_action};
pub use main_menu::main_menu_keybindings;
pub use skirmish::skirmish_keybindings;
