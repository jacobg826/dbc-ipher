use color_eyre::eyre::Ok;
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;
use std::time::Duration;
use tui_tree_widget::TreeState;

use crate::ui::render;

pub struct App {
    pub dbc: dbc_rs::Dbc,
    pub tree_state: TreeState<String>,
    pub running_state: RunningState,
}

#[derive(Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

pub enum Msg {
    MoveDown,
    MoveUp,
    MoveLeft,
    MoveRight,
    ToggleSelected,
    Quit,
}

impl App {
    pub fn new(dbc: dbc_rs::Dbc) -> Self {
        Self {
            dbc,
            tree_state: TreeState::default(),
            running_state: RunningState::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while self.running_state != RunningState::Done {
            // Render the current view
            terminal.draw(|frame| render(self, frame))?;

            // Handle events and map to a Message
            let mut current_msg = handle_event(&self)?;

            // Process updates as long as they return a non-None message
            while current_msg.is_some() {
                current_msg = update(self, current_msg.unwrap());
            }
        }
        Ok(())
    }
}

fn handle_event(_: &App) -> color_eyre::Result<Option<Msg>> {
    if event::poll(Duration::from_millis(250))?
        && let Event::Key(key) = event::read()?
        && key.kind == event::KeyEventKind::Press
    {
        return Ok(handle_key(key.code));
    }
    Ok(None)
}

fn handle_key(key: KeyCode) -> Option<Msg> {
    match key {
        KeyCode::Char('j') | KeyCode::Down => Some(Msg::MoveDown),
        KeyCode::Char('k') | KeyCode::Up => Some(Msg::MoveUp),
        KeyCode::Char('h') | KeyCode::Left => Some(Msg::MoveLeft),
        KeyCode::Char('l') | KeyCode::Right => Some(Msg::MoveRight),
        KeyCode::Enter | KeyCode::Char(' ') => Some(Msg::ToggleSelected),
        KeyCode::Char('q') | KeyCode::Esc => Some(Msg::Quit),
        _ => None,
    }
}

pub fn update(app: &mut App, msg: Msg) -> Option<Msg> {
    match msg {
        Msg::MoveDown => {
            app.tree_state.key_down();
        }
        Msg::MoveUp => {
            app.tree_state.key_up();
        }
        Msg::MoveLeft => {
            app.tree_state.key_left();
        }
        Msg::MoveRight => {
            app.tree_state.key_right();
        }
        Msg::ToggleSelected => {
            app.tree_state.toggle_selected();
        }
        Msg::Quit => {
            app.running_state = RunningState::Done;
        }
    }
    None
}
