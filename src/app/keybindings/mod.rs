mod default;
mod keybindings;
mod main_menu;
mod skirmish;

pub use default::{common_keybindings, default_keybindings};
pub use keybindings::{
  Action, Group, KeyBinding, binding_for, count_largest_group, event_to_action,
};
pub use main_menu::main_menu_keybindings;
pub use skirmish::skirmish_keybindings;
