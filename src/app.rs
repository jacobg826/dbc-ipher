use crossterm::event::{self, KeyCode};
use ratatui::DefaultTerminal;
use tui_tree_widget::TreeState;

use crate::ui::render;

pub struct App {
    pub dbc: dbc_rs::Dbc,
    pub tree_state: TreeState<String>,
}

impl App {
    pub fn new(dbc: dbc_rs::Dbc) -> Self {
        Self {
            dbc,
            tree_state: TreeState::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| render(self, frame))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.tree_state.key_down();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.tree_state.key_up();
                    }
                    KeyCode::Char('h') | KeyCode::Left => {
                        self.tree_state.key_left();
                    }
                    KeyCode::Char('l') | KeyCode::Right => {
                        self.tree_state.key_right();
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        self.tree_state.toggle_selected();
                    }
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    _ => {}
                }
            }
        }
    }
}
