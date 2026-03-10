use crate::app::View;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    ScrollUp,
    ScrollDown,
    Select,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub action: Action,
    pub code: KeyCode,
    pub kind: KeyEventKind,
    pub modifiers: KeyModifiers,
    pub description: &'static str,
}

pub static KEYBINDINGS: &[KeyBinding] = &[
    KeyBinding {
        action: Action::Quit,
        code: KeyCode::Char('q'),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        description: "Quit",
    },
    KeyBinding {
        action: Action::ScrollUp,
        code: KeyCode::Up,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        description: "Up",
    },
    KeyBinding {
        action: Action::ScrollDown,
        code: KeyCode::Down,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        description: "Down",
    },
    KeyBinding {
        action: Action::Select,
        code: KeyCode::Char(' '),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        description: "Select",
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

pub fn binding_for_view(view: View) -> Vec<&'static KeyBinding> {
    match view {
        View::MainMenu => vec![
            binding_for(Action::ScrollUp).unwrap(),
            binding_for(Action::ScrollDown).unwrap(),
            binding_for(Action::Select).unwrap(),
            binding_for(Action::Quit).unwrap(),
        ],
        _ => vec![],
    }
}
