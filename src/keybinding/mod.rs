mod action;

use crossterm::event::KeyCode;
use std::collections::HashMap;

use crate::keybinding::action::Action;
pub use crate::keybinding::action::footer_actions;

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

    // priorities for how keys should be displayed in the keybinding popup
    pub fn key_sort_rank(key: &KeyCode) -> (u8, String) {
        match key {
            KeyCode::Char(c) => (0, c.to_string()),
            KeyCode::Up => (1, "1".into()),
            KeyCode::Down => (1, "2".into()),
            KeyCode::Left => (1, "3".into()),
            KeyCode::Right => (1, "4".into()),
            KeyCode::Enter => (2, "1".into()),
            KeyCode::Esc => (2, "2".into()),
            _ => (3, format!("{:?}", key)),
        }
    }
    pub fn lookup(&self, ctx: Context, key: KeyCode) -> Option<Action> {
        // Context-specific binding wins; fall back to Global.
        self.bindings
            .get(&(ctx, key))
            .or_else(|| self.bindings.get(&(Context::Global, key)))
            .copied()
    }

    pub fn grouped_bindings_for(&self, ctx: Context) -> Vec<(Action, Vec<KeyCode>)> {
        let mut by_action: HashMap<Action, Vec<KeyCode>> = HashMap::new();

        for ((c, key), action) in &self.bindings {
            if *c == ctx || *c == Context::Global {
                by_action.entry(*action).or_default().push(*key);
            }
        }

        Action::all()
            .iter()
            .filter_map(|a| {
                by_action.remove(a).map(|mut keys| {
                    keys.sort_by_key(Keymap::key_sort_rank);
                    (*a, keys)
                })
            })
            .collect()
    }

    pub fn primary_key_for(&self, ctx: Context, action: Action) -> Option<KeyCode> {
        self.bindings
            .iter()
            .map(|(&(c, k), &a)| (c, k, a)) // copy out, now working with plain values
            .filter(|&(c, _, a)| (c == ctx || c == Context::Global) && a == action)
            .map(|(_, k, _)| k)
            .min_by_key(|&k| Keymap::key_sort_rank(&k))
    }
}
