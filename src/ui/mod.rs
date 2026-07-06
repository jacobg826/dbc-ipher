mod detail;
mod tree;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::selection::resolve_selection;
use crate::ui::detail::render_detail;
use crate::ui::tree::render_tree;

// Top-level entry point called once per frame from App::run.
// Splits the screen into regions and hands each region off to
// the module responsible for rendering it.
pub fn render(app: &mut App, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(outer_layout[0]);

    frame.render_widget(
        Paragraph::new("Bottom").block(Block::new().title(" [2] Console ").borders(Borders::ALL)),
        outer_layout[1],
    );

    let messages: Vec<_> = app.dbc.messages().iter().collect();
    render_tree(
        frame,
        inner_layout[0],
        &messages,
        &mut app.tree_state,
        &app.focus_state,
    );

    let selected = resolve_selection(&app.tree_state, &messages);
    render_detail(frame, inner_layout[1], selected, &app.focus_state);
}

pub fn panel_block(title: &str, keybinding: char, is_focused: bool) -> Block<'_> {
    let mut block = Block::default()
        .borders(Borders::ALL)
        .title(block_title(title, keybinding));
    if is_focused {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }
    block
}

fn block_title(text: &str, keybinding: char) -> String {
    format!(" [{}] {} ", keybinding, text)
}
