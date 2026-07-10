use crate::app::Msg;

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
