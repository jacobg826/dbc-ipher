use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::Selection;

pub fn render_detail(frame: &mut Frame, area: Rect, selection: Selection) {
    match selection {
        Selection::Message(msg) => {
            let outer_block = Block::default().borders(Borders::ALL);
            let inner_area = outer_block.inner(area);
            frame.render_widget(outer_block, area);

            let [msg_area, sig_area] =
                Layout::vertical([Constraint::Length(8), Constraint::Fill(1)]).areas(inner_area);

            render_message_header(frame, msg_area, msg);
            render_signal_list(frame, sig_area, msg);
        }
        Selection::Signal(msg, sig) => {
            render_signal_detail(frame, area, msg, sig);
        }
        Selection::None => {
            frame.render_widget(
                Paragraph::new("").block(Block::default().borders(Borders::ALL).title("Detail")),
                area,
            );
        }
    }
}

fn render_message_header(frame: &mut Frame, area: Rect, msg: &dbc_rs::Message) {
    let id = msg.id();
    let pgn = (id >> 8) & 0xFFFF;
    let sa = id & 0xFF;
    let da = (id >> 8) & 0xFF;
    let priority = (id >> 26) & 0x7;

    let label_width = 10;
    let col_width = 16;

    let lines = vec![
        Line::from(vec![
            Span::styled(
                format!("{:<label_width$}", "Name:"),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(msg.name()),
        ]),
        Line::from(vec![
            Span::styled(
                format!("{:<label_width$}", "ID:"),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(format!("0x{:08X}", id)),
        ]),
        Line::from(vec![
            Span::styled(
                format!("{:<label_width$}", "PGN:"),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(format!("{:<col_width$}", format!("0x{:04X}", pgn))),
            Span::styled(
                format!("{:<label_width$}", "Priority:"),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(format!("{}", priority)),
        ]),
        Line::from(vec![
            Span::styled(
                format!("{:<label_width$}", "SA:"),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(format!("{:<col_width$}", format!("0x{:02X}", sa))),
            Span::styled(
                format!("{:<label_width$}", "DA:"),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(format!("0x{:02X}", da)),
        ]),
        Line::from(vec![
            Span::styled(
                format!("{:<label_width$}", "DLC:"),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(format!("{}", msg.dlc())),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Message")),
        area,
    );
}

fn render_signal_list(frame: &mut Frame, area: Rect, msg: &dbc_rs::Message) {
    let sig_names: Vec<Line> = msg
        .signals()
        .iter()
        .map(|sig| {
            return Line::from(vec![Span::raw(sig.name())]);
        })
        .collect();
    frame.render_widget(
        Paragraph::new(sig_names).block(Block::default().borders(Borders::ALL).title("Signals")),
        area,
    );
}

fn render_signal_detail(
    frame: &mut Frame,
    area: Rect,
    _message: &dbc_rs::Message,
    _sig: &dbc_rs::Signal,
) {
    frame.render_widget(
        Paragraph::new("a").block(Block::default().borders(Borders::ALL).title("Signals")),
        area,
    );
}
