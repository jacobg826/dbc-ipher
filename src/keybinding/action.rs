use crate::app::Msg;
use crate::keybinding::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    TreeMoveDown,
    TreeMoveUp,
    TreeMoveLeft,
    TreeMoveRight,
    TreeToggleSelected,
    DetailScrollDown,
    DetailScrollUp,
    FocusTree,
    FocusDetail,
    ToggleKeybindPopup,
    ClosePopup,
    Quit,
}

impl Action {
    pub fn description(&self) -> &'static str {
        match self {
            Action::TreeMoveDown => "Move down",
            Action::TreeMoveUp => "Move up",
            Action::TreeMoveLeft => "Collapse / move left",
            Action::TreeMoveRight => "Expand / move right",
            Action::TreeToggleSelected => "Toggle selected node",
            Action::DetailScrollDown => "Scroll detail down",
            Action::DetailScrollUp => "Scroll detail up",
            Action::FocusTree => "Focus tree panel",
            Action::FocusDetail => "Focus detail panel",
            Action::ToggleKeybindPopup => "Show/hide keybindings",
            Action::ClosePopup => "Close popup",
            Action::Quit => "Quit",
        }
    }

    // Canonical order of actions for keybinding popup
    pub fn all() -> &'static [Action] {
        &[
            Action::FocusTree,
            Action::FocusDetail,
            Action::TreeMoveUp,
            Action::TreeMoveDown,
            Action::TreeMoveLeft,
            Action::TreeMoveRight,
            Action::TreeToggleSelected,
            Action::DetailScrollUp,
            Action::DetailScrollDown,
            Action::ToggleKeybindPopup,
            Action::ClosePopup,
            Action::Quit,
        ]
    }

    /// currently a direct line to the Msg enum. payload free right now,
    /// but this is the place where a future Action::JumpToSignal(String)
    /// can plug in without touching Keymap at all.
    pub fn to_msg(self) -> Msg {
        match self {
            Action::TreeMoveDown => Msg::TreeMoveDown,
            Action::TreeMoveUp => Msg::TreeMoveUp,
            Action::TreeMoveLeft => Msg::TreeMoveLeft,
            Action::TreeMoveRight => Msg::TreeMoveRight,
            Action::TreeToggleSelected => Msg::TreeToggleSelected,
            Action::DetailScrollDown => Msg::DetailScrollDown,
            Action::DetailScrollUp => Msg::DetailScrollUp,
            Action::FocusTree => Msg::FocusTree,
            Action::FocusDetail => Msg::FocusDetail,
            Action::ToggleKeybindPopup => Msg::ToggleKeybindPopup,
            Action::ClosePopup => Msg::ToggleKeybindPopup,
            Action::Quit => Msg::Quit,
        }
    }
}

pub fn footer_actions(ctx: Context) -> &'static [Action] {
    match ctx {
        Context::Tree => &[
            Action::TreeMoveDown,
            Action::TreeMoveUp,
            Action::ToggleKeybindPopup,
            Action::Quit,
        ],
        Context::Detail => &[Action::ToggleKeybindPopup, Action::Quit],
        Context::Popup => &[Action::ClosePopup],
        Context::Global => &[Action::ToggleKeybindPopup, Action::Quit],
    }
}

#[test]
fn all_actions_are_listed() {
    // Exhaustive match forces a compile error the moment a new Action
    // variant is added and not handled here
    fn assert_covered(action: Action) {
        match action {
            Action::TreeMoveDown
            | Action::TreeMoveUp
            | Action::TreeMoveLeft
            | Action::TreeMoveRight
            | Action::TreeToggleSelected
            | Action::DetailScrollDown
            | Action::DetailScrollUp
            | Action::FocusTree
            | Action::FocusDetail
            | Action::ToggleKeybindPopup
            | Action::ClosePopup
            | Action::Quit => {}
        }
    }

    for action in Action::all() {
        assert_covered(*action);
    }

    // Check all variants exist in all()
    assert_eq!(
        Action::all().len(),
        12,
        "update this count when adding a new Action variant"
    );
}
