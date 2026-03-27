use clap::ValueEnum;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    Quit2,
    Up,
    Down,
    Left,
    Right,
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    Space,
    Enter,
    Esc,
    Backspace,
    WildCard(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum Group {
    CtrlMovement,
    Movement,
    Scroll,
    Select,
    Input,
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
        action: Action::Left,
        code: KeyCode::Left,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Movement,
        symbol: "←",
        description: "Left",
    },
    KeyBinding {
        action: Action::Right,
        code: KeyCode::Right,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Movement,
        symbol: "→",
        description: "Right",
    },
    KeyBinding {
        action: Action::ScrollUp,
        code: KeyCode::Up,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::CONTROL,
        group: Group::CtrlMovement,
        symbol: "Ctrl + ↑",
        description: "Scroll Up ",
    },
    KeyBinding {
        action: Action::ScrollDown,
        code: KeyCode::Down,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::CONTROL,
        group: Group::CtrlMovement,
        symbol: "Ctrl + ↓",
        description: "Scroll Down",
    },
    KeyBinding {
        action: Action::ScrollLeft,
        code: KeyCode::Left,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::CONTROL,
        group: Group::CtrlMovement,
        symbol: "Ctrl + ←",
        description: "Scroll Left",
    },
    KeyBinding {
        action: Action::ScrollRight,
        code: KeyCode::Right,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::CONTROL,
        group: Group::CtrlMovement,
        symbol: "Ctrl + →",
        description: "Scroll Right",
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
        description: "Go back to main menu",
    },
    KeyBinding {
        action: Action::Backspace,
        code: KeyCode::Backspace,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Input,
        symbol: "Backspace",
        description: "Delete character",
    },
    KeyBinding {
        action: Action::WildCard('_'),
        code: KeyCode::Char('_'),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Input,
        symbol: "[All]",
        description: "All keyboard characters",
    },
];

pub fn binding_for(action: Action) -> Option<&'static KeyBinding> {
    KEYBINDINGS.iter().find(|b| b.action == action)
}

pub fn event_to_action(event: &KeyEvent) -> Option<Action> {
    if let Some(b) = KEYBINDINGS
        .iter()
        .find(|b| b.code == event.code && b.kind == event.kind && b.modifiers == event.modifiers)
    {
        return Some(b.action);
    }

    if KEYBINDINGS
        .iter()
        .any(|b| matches!(b.action, Action::WildCard(_)) && b.kind == event.kind)
    {
        if let KeyCode::Char(c) = event.code {
            return Some(Action::WildCard(c));
        }
    }

    None
}
