mod action;

use crossterm::event::KeyCode;
use std::collections::HashMap;

use crate::keybinding::action::Action;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Context {
    Tree,
    Detail,
    Popup,
    Global,
}

pub struct Keymap {
    bindings: HashMap<(Context, KeyCode), Action>,
}

impl Keymap {
    pub fn default() -> Self {
        let mut b = HashMap::new();

        // Global
        b.insert((Context::Global, KeyCode::Char('1')), Action::FocusTree);
        b.insert((Context::Global, KeyCode::Char('2')), Action::FocusDetail);
        b.insert((Context::Global, KeyCode::Char('q')), Action::Quit);
        b.insert((Context::Global, KeyCode::Esc), Action::Quit);
        b.insert(
            (Context::Global, KeyCode::Char('?')),
            Action::ToggleKeybindPopup,
        );

        // Popup (overrides Esc and ? from Global while open)
        b.insert((Context::Popup, KeyCode::Esc), Action::ClosePopup);
        b.insert((Context::Popup, KeyCode::Char('?')), Action::ClosePopup);

        // Tree
        b.insert((Context::Tree, KeyCode::Char('j')), Action::TreeMoveDown);
        b.insert((Context::Tree, KeyCode::Down), Action::TreeMoveDown);
        b.insert((Context::Tree, KeyCode::Char('k')), Action::TreeMoveUp);
        b.insert((Context::Tree, KeyCode::Up), Action::TreeMoveUp);
        b.insert((Context::Tree, KeyCode::Char('h')), Action::TreeMoveLeft);
        b.insert((Context::Tree, KeyCode::Left), Action::TreeMoveLeft);
        b.insert((Context::Tree, KeyCode::Char('l')), Action::TreeMoveRight);
        b.insert((Context::Tree, KeyCode::Right), Action::TreeMoveRight);
        b.insert((Context::Tree, KeyCode::Enter), Action::TreeToggleSelected);
        b.insert(
            (Context::Tree, KeyCode::Char(' ')),
            Action::TreeToggleSelected,
        );

        // Detail
        b.insert(
            (Context::Detail, KeyCode::Char('j')),
            Action::DetailScrollDown,
        );
        b.insert((Context::Detail, KeyCode::Down), Action::DetailScrollDown);
        b.insert(
            (Context::Detail, KeyCode::Char('k')),
            Action::DetailScrollUp,
        );
        b.insert((Context::Detail, KeyCode::Up), Action::DetailScrollUp);

        Keymap { bindings: b }
    }

    pub fn lookup(&self, ctx: Context, key: KeyCode) -> Option<Action> {
        // Context-specific binding wins; fall back to Global.
        self.bindings
            .get(&(ctx, key))
            .or_else(|| self.bindings.get(&(Context::Global, key)))
            .copied()
    }

    /// For tool tips: everything active in this context, popup excluded
    /// unless explicitly asked for.
    pub fn bindings_for(&self, ctx: Context) -> Vec<(KeyCode, Action)> {
        self.bindings
            .iter()
            .filter(|((c, _), _)| *c == ctx || *c == Context::Global)
            .map(|((_, k), a)| (*k, *a))
            .collect()
    }
}
