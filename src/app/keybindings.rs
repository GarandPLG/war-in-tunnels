use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::View;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub action: Action,
    pub code: KeyCode,
    pub kind: KeyEventKind,
    pub modifiers: KeyModifiers,
    pub description: &'static str,
}

pub static KEYBINDINGS: &[KeyBinding] = &[KeyBinding {
    action: Action::Quit,
    code: KeyCode::Char('q'),
    kind: KeyEventKind::Press,
    modifiers: KeyModifiers::NONE,
    description: "Quit",
}];

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
        View::MainMenu => binding_for(Action::Quit)
            .map(|b| vec![b])
            .unwrap_or_else(|| vec![]),
        _ => vec![],
    }
}
