use clap::ValueEnum;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum Group {
    Movement,
    Select,
    Quit,
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
