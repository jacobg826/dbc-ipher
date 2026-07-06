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
    pub focus_state: Focus,
}

#[derive(Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Default, PartialEq, Eq)]
pub enum Focus {
    #[default]
    Tree,
    Detail,
}

pub enum Msg {
    TreeMoveDown,
    TreeMoveUp,
    TreeMoveLeft,
    TreeMoveRight,
    TreeToggleSelected,
    DetailScrollDown,
    DetailScrollUp,
    FocusTree,
    FocusDetail,
    Quit,
}

impl App {
    pub fn new(dbc: dbc_rs::Dbc) -> Self {
        Self {
            dbc,
            tree_state: TreeState::default(),
            running_state: RunningState::default(),
            focus_state: Focus::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while self.running_state != RunningState::Done {
            // Render the current view
            terminal.draw(|frame| render(self, frame))?;

            // Handle events and map to a Message
            let mut current_msg = handle_event(self)?;

            // Process updates as long as they return a non-None message
            while current_msg.is_some() {
                current_msg = update(self, current_msg.unwrap());
            }
        }
        Ok(())
    }
}

fn handle_event(app: &App) -> color_eyre::Result<Option<Msg>> {
    if event::poll(Duration::from_millis(250))?
        && let Event::Key(key) = event::read()?
        && key.kind == event::KeyEventKind::Press
    {
        return Ok(handle_key(key.code, &app.focus_state));
    }
    Ok(None)
}

fn handle_key(key: KeyCode, focus_state: &Focus) -> Option<Msg> {
    match focus_state {
        Focus::Tree => match key {
            KeyCode::Char('j') | KeyCode::Down => Some(Msg::TreeMoveDown),
            KeyCode::Char('k') | KeyCode::Up => Some(Msg::TreeMoveUp),
            KeyCode::Char('h') | KeyCode::Left => Some(Msg::TreeMoveLeft),
            KeyCode::Char('l') | KeyCode::Right => Some(Msg::TreeMoveRight),
            KeyCode::Enter | KeyCode::Char(' ') => Some(Msg::TreeToggleSelected),
            KeyCode::Char('1') => Some(Msg::FocusTree),
            KeyCode::Char('2') => Some(Msg::FocusDetail),
            KeyCode::Char('q') | KeyCode::Esc => Some(Msg::Quit),
            _ => None,
        },
        Focus::Detail => match key {
            KeyCode::Char('j') | KeyCode::Down => Some(Msg::DetailScrollDown),
            KeyCode::Char('k') | KeyCode::Up => Some(Msg::DetailScrollUp),
            KeyCode::Char('1') => Some(Msg::FocusTree),
            KeyCode::Char('2') => Some(Msg::FocusDetail),
            KeyCode::Char('q') | KeyCode::Esc => Some(Msg::Quit),
            _ => None,
        },
    }
}

pub fn update(app: &mut App, msg: Msg) -> Option<Msg> {
    match msg {
        Msg::TreeMoveDown => {
            app.tree_state.key_down();
        }
        Msg::TreeMoveUp => {
            app.tree_state.key_up();
        }
        Msg::TreeMoveLeft => {
            app.tree_state.key_left();
        }
        Msg::TreeMoveRight => {
            app.tree_state.key_right();
        }
        Msg::TreeToggleSelected => {
            app.tree_state.toggle_selected();
        }
        Msg::DetailScrollDown => {}
        Msg::DetailScrollUp => {}
        Msg::FocusTree => {
            app.focus_state = Focus::Tree;
        }
        Msg::FocusDetail => {
            app.focus_state = Focus::Detail;
        }
        Msg::Quit => {
            app.running_state = RunningState::Done;
        }
    }
    None
}
