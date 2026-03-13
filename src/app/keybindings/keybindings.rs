use crate::app::View;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    Quit2,
    Up,
    Down,
    Space,
    Enter,
    Esc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Group {
    Movement,
    Select,
    Quit,
}

impl Group {
    pub fn iter() -> impl Iterator<Item = Group> {
        [Group::Movement, Group::Select, Group::Quit]
            .iter()
            .copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub action: Action,
    pub code: KeyCode,
    pub kind: KeyEventKind,
    pub modifiers: KeyModifiers,
    pub group: Group,
    pub symbol: &'static str,
    pub description: &'static str,
}

pub static KEYBINDINGS: &[KeyBinding] = &[
    KeyBinding {
        action: Action::Quit,
        code: KeyCode::Char('q'),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Quit,
        symbol: "q",
        description: "Quit",
    },
    KeyBinding {
        action: Action::Quit2,
        code: KeyCode::Char('c'),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::CONTROL,
        group: Group::Quit,
        symbol: "Ctrl + c",
        description: "Quit",
    },
    KeyBinding {
        action: Action::Up,
        code: KeyCode::Up,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Movement,
        symbol: "↑",
        description: "Up",
    },
    KeyBinding {
        action: Action::Down,
        code: KeyCode::Down,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Movement,
        symbol: "↓",
        description: "Down",
    },
    KeyBinding {
        action: Action::Space,
        code: KeyCode::Char(' '),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Select,
        symbol: "Space",
        description: "Select",
    },
    KeyBinding {
        action: Action::Enter,
        code: KeyCode::Enter,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Select,
        symbol: "Enter",
        description: "Submit",
    },
    KeyBinding {
        action: Action::Esc,
        code: KeyCode::Esc,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Movement,
        symbol: "Esc",
        description: "Go back",
    },
];

pub fn binding_for(action: Action) -> Option<&'static KeyBinding> {
    KEYBINDINGS.iter().find(|b| b.action == action)
}

pub fn event_to_action(event: &KeyEvent) -> Option<Action> {
    KEYBINDINGS
        .iter()
        .find(|b| b.code == event.code && b.kind == event.kind && b.modifiers == event.modifiers)
        .map(|b| b.action)
}

fn used_groups_in_view(keybindings: &Vec<Option<&'static KeyBinding>>) -> HashSet<Group> {
    keybindings
        .iter()
        .filter_map(|b| b.map(|binding| binding.group))
        .collect()
}

pub fn binding_for_view(view: View) -> (HashSet<Group>, Vec<Option<&'static KeyBinding>>) {
    let main_menu_keybindings: Vec<Option<&'static KeyBinding>> = vec![
        binding_for(Action::Up),
        binding_for(Action::Down),
        binding_for(Action::Space),
        binding_for(Action::Quit),
        binding_for(Action::Quit2),
    ];

    let settings_keybinding: Vec<Option<&'static KeyBinding>> = vec![
        binding_for(Action::Up),
        binding_for(Action::Down),
        binding_for(Action::Space),
        binding_for(Action::Enter),
        binding_for(Action::Quit),
        binding_for(Action::Quit2),
        binding_for(Action::Esc),
    ];

    let default_keybindings: Vec<Option<&'static KeyBinding>> = vec![
        binding_for(Action::Quit),
        binding_for(Action::Quit2),
        binding_for(Action::Esc),
    ];

    match view {
        View::MainMenu => (
            used_groups_in_view(&main_menu_keybindings),
            main_menu_keybindings,
        ),
        View::Settings => (
            used_groups_in_view(&settings_keybinding),
            settings_keybinding,
        ),
        _ => (
            used_groups_in_view(&default_keybindings),
            default_keybindings,
        ),
    }
}
