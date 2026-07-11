mod detail;
mod tree;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::{App, Focus, current_context};
use crate::keybinding::{Context, Keymap};
use crate::selection::resolve_selection;
use crate::ui::detail::{label, render_detail};
use crate::ui::tree::render_tree;

// Top-level entry point called once per frame from App::run.
// Splits the screen into regions and hands each region off to
// the module responsible for rendering it.
pub fn render(app: &mut App, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(outer_layout[0]);

    let messages: Vec<_> = app.dbc.messages().iter().collect();

    let curr_focus = if app.show_keybind_popup {
        &Focus::None
    } else {
        &app.focus_state
    };

    render_tree(
        frame,
        inner_layout[0],
        &messages,
        &mut app.tree_state,
        curr_focus,
    );

    let selected = resolve_selection(&app.tree_state, &messages);
    render_detail(frame, inner_layout[1], selected, curr_focus);

    render_footer(frame, outer_layout[1]);

    if app.show_keybind_popup {
        let ctx = current_context(app);
        render_keybinding_popup(frame, &app.keymap, ctx);
    }
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

// TODO: make footer dynamic based off current focus
fn render_footer(frame: &mut Frame, area: Rect) {
    let hints = [("q", "quit"), ("<esc>", "cancel"), ("?", "help")];

    let spans: Vec<Span> = hints
        .iter()
        .flat_map(|(key, label)| {
            vec![
                Span::styled(*key, Style::default().fg(Color::Yellow)),
                Span::raw(format!(":{label}  ")),
            ]
        })
        .collect();

    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_keybinding_popup(frame: &mut Frame, keymap: &Keymap, ctx: Context) {
    let popup_block = Block::bordered().title("Keybindings");
    let centered_area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Percentage(20));
    frame.render_widget(Clear, centered_area);

    let paragraph = Paragraph::new("Lorem ipsum").block(popup_block);

    let label_width = 10;
    let lines: Vec<Line> = keymap
        .bindings_for(ctx)
        .iter()
        .map(|(key, action)| {
            Line::from(vec![
                label(&key.to_string(), label_width),
                Span::raw(action.description()),
            ])
        })
        .collect();

    frame.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("thing")),
        centered_area,
    );
}
