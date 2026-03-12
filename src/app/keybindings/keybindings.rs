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
    Esc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Group {
    Quit,
    Movement,
    Select,
}

impl Group {
    pub fn iter() -> impl Iterator<Item = Group> {
        [Group::Quit, Group::Movement, Group::Select]
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

fn used_groups_in_view(keybindings: &Vec<&'static KeyBinding>) -> HashSet<Group> {
    keybindings.iter().map(|b| b.group).collect()
}

pub fn binding_for_view(view: View) -> (HashSet<Group>, Vec<&'static KeyBinding>) {
    let main_menu_keybindings: Vec<&'static KeyBinding> = vec![
        binding_for(Action::Up).unwrap(),
        binding_for(Action::Down).unwrap(),
        binding_for(Action::Space).unwrap(),
        binding_for(Action::Quit).unwrap(),
        binding_for(Action::Quit2).unwrap(),
    ];

    let default_keybindings: Vec<&'static KeyBinding> = vec![
        binding_for(Action::Quit).unwrap(),
        binding_for(Action::Quit2).unwrap(),
        binding_for(Action::Esc).unwrap(),
    ];

    match view {
        View::MainMenu => (
            used_groups_in_view(&main_menu_keybindings),
            main_menu_keybindings,
        ),
        _ => (
            used_groups_in_view(&default_keybindings),
            default_keybindings,
        ),
    }
}
