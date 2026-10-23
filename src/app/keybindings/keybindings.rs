use clap::ValueEnum;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::collections::HashMap;

/// Represents the set of actions that the UI can perform.
///
/// Each variant corresponds to a concrete operation that can be triggered by a
/// key binding. The `WildCard` variant is a special case that matches any
/// character key and carries the actual character pressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    /// Request the application to quit.
    Quit,
    /// Alternate quit action.
    Quit2,
    /// Move the selection cursor up.
    Up,
    /// Move the selection cursor down.
    Down,
    /// Move the selection cursor left.
    Left,
    /// Move the selection cursor right.
    Right,
    /// Scroll up without moving the cursor.
    ScrollUp,
    /// Scroll down without moving the cursor.
    ScrollDown,
    /// Scroll left without moving the cursor.
    ScrollLeft,
    /// Scroll right without moving the cursor.
    ScrollRight,
    /// Toggle selecting cell.
    Space,
    /// Open or close the construction view.
    Building,
    /// Open or close the recruitment view.
    Recruitment,
    /// Unmark last selected cell.
    Backspace,
    /// Cancel marking cell.
    Delete,
    /// Submit or confirm the current choice.
    Enter,
    /// Return to the main menu or previous screen.
    Esc,
    /// Zoom the view in.
    ZoomIn,
    /// Zoom the view out.
    ZoomOut,
    /// Open side panel.
    Tab,
    /// Toggle skirmish board/skills view.
    ShiftTab,
    /// Mute music.
    Mute,
    /// Volume up.
    VolumeUp,
    /// Volume down.
    VolumeDown,
    /// Matches any character key; the inner `char` is the actual key pressed.
    WildCard(char),
}

/// Logical groups of key bindings used for organising help screens or
/// configuration sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum Group {
    /// Bindings that combine `Ctrl` with movement keys.
    CtrlMovement,
    /// Plain movement bindings.
    Movement,
    /// Scroll bindings.
    Scroll,
    /// Selection bindings.
    Select,
    /// Text input related bindings.
    Input,
    /// Zoom related bindings.
    Zoom,
    /// Music related bindings.
    Music,
    /// Opens/Closes/Toggles elements in current view.
    Opener,
    /// Quit related bindings.
    Quit,
}

/// Description of a single key binding.
///
/// This struct stores the mapping from a physical key event to an `Action`,
/// together with metadata used for displaying help information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    /// The logical action performed when this binding is triggered.
    pub action: Action,
    /// The raw `crossterm::event::KeyCode` that identifies the key.
    pub code: KeyCode,
    /// The kind of key event (crossterm::event::KeyEventKind).
    pub kind: KeyEventKind,
    /// Any modifier keys that must be held (crossterm::event::KeyModifiers).
    pub modifiers: KeyModifiers,
    /// The group this binding belongs to, useful for categorising help.
    pub group: Group,
    /// Human‑readable symbol shown in UI/help screens.
    pub symbol: &'static str,
    /// Short description of what the binding does.
    pub description: &'static str,
}

/// Complete list of all key bindings used by the application.
///
/// The slice is ordered primarily for readability; the runtime lookup scans
/// this slice linearly, which is acceptable given the modest size of the list.
static KEYBINDINGS: &[KeyBinding] = &[
    KeyBinding {
        action: Action::Recruitment,
        code: KeyCode::Char('r'),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Opener,
        symbol: "r",
        description: "Recruit",
    },
    KeyBinding {
        action: Action::Building,
        code: KeyCode::Char('b'),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Opener,
        symbol: "b",
        description: "Buildings",
    },
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
        description: "Marking cells",
    },
    KeyBinding {
        action: Action::Backspace,
        code: KeyCode::Backspace,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Select,
        symbol: "Backspace",
        description: "Unmark cell",
    },
    KeyBinding {
        action: Action::Delete,
        code: KeyCode::Delete,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Select,
        symbol: "Delete",
        description: "Cancel marking",
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
        group: Group::Quit,
        symbol: "Esc",
        description: "Main menu",
    },
    KeyBinding {
        action: Action::ZoomIn,
        code: KeyCode::Char(','),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Zoom,
        symbol: ",",
        description: "Zoom in",
    },
    KeyBinding {
        action: Action::ZoomOut,
        code: KeyCode::Char('.'),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Zoom,
        symbol: ".",
        description: "Zoom out",
    },
    KeyBinding {
        action: Action::Mute,
        code: KeyCode::Char('m'),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Music,
        symbol: "m",
        description: "🔈",
    },
    KeyBinding {
        action: Action::VolumeUp,
        code: KeyCode::Char('+'),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Music,
        symbol: "+",
        description: "🔊",
    },
    KeyBinding {
        action: Action::VolumeDown,
        code: KeyCode::Char('-'),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Music,
        symbol: "-",
        description: "🔉",
    },
    KeyBinding {
        action: Action::Tab,
        code: KeyCode::Tab,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        group: Group::Opener,
        symbol: "Tab",
        description: "Toggle side panel",
    },
    KeyBinding {
        action: Action::ShiftTab,
        code: KeyCode::Tab,
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::SHIFT,
        group: Group::Opener,
        symbol: "Shift + Tab",
        description: "Toggle Map/Skills window",
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

/// Returns the `KeyBinding` associated with the given `Action`, if one exists.
pub fn binding_for(action: Action) -> Option<&'static KeyBinding> {
    KEYBINDINGS.iter().find(|b| b.action == action)
}

/// Converts a raw `KeyEvent` into an `Action` if a matching binding is found.
///
/// The function first looks for an exact match on code, kind, and modifiers.
/// If no exact match exists but a wildcard binding is present for the same
/// event kind, the function returns `Action::WildCard` with the character that
/// was pressed. Returns `None` when the event does not correspond to any known
/// action.
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

/// Determines the size of the largest group among a slice of actions.
///
/// It counts how many times each group appears in the supplied actions and
/// returns the highest count. This can be used, for example, to size UI
/// elements that display groups of bindings.
pub fn count_largest_group(actions: &[Action]) -> u16 {
    let mut group_counts: HashMap<Group, u16> = HashMap::new();

    for action in actions {
        if let Some(binding) = binding_for(*action) {
            *group_counts.entry(binding.group).or_insert(0) += 1;
        }
    }

    group_counts.values().copied().max().map_or(0, |v| v)
}
