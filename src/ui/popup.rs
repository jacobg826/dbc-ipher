use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::Focus;
use crate::keybinding::{Context, Keymap};
use crate::ui::detail::label;

pub fn render_keybinding_popup(frame: &mut Frame, keymap: &Keymap, ctx: Context, focus: &Focus) {
    let area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Percentage(20));

    let label_width = 20;
    let lines: Vec<Line> = keymap
        .grouped_bindings_for(ctx)
        .iter()
        .map(|(action, keys)| {
            let key_str = keys
                .iter()
                .map(|k| k.to_string())
                .collect::<Vec<_>>()
                .join("/");
            Line::from(vec![
                label(&key_str, label_width),
                Span::raw(action.description()),
            ])
        })
        .collect();

    let block = popup_block("Keybindings", *focus == Focus::Popup);
    let popup = Paragraph::new(lines).block(block);

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}

pub fn popup_block(title: &str, is_focused: bool) -> Block<'_> {
    let mut block = Block::default().borders(Borders::ALL).title(title);
    if is_focused {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }
    block
}
