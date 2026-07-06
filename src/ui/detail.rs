use dbc_rs::ByteOrder;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::Selection;
use crate::app::Focus;
use crate::ui::panel_block;

pub fn render_detail(frame: &mut Frame, area: Rect, selection: Selection, focus_state: &Focus) {
    let outer_block = panel_block("Detail", '2', *focus_state == Focus::Detail);
    let inner_area = outer_block.inner(area);
    frame.render_widget(outer_block, area);
    match selection {
        Selection::Message(msg) => {
            let [msg_area, sig_area] =
                Layout::vertical([Constraint::Length(8), Constraint::Fill(1)]).areas(inner_area);
            render_message_header(frame, msg_area, msg);
            render_signal_list(frame, sig_area, msg);
        }
        Selection::Signal(msg, sig) => {
            render_signal_detail(frame, inner_area, msg, sig);
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
        Line::from(vec![label("Name:", label_width), Span::raw(msg.name())]),
        Line::from(vec![
            label("ID:", label_width),
            Span::raw(format!("0x{:08X}", id)),
        ]),
        Line::from(vec![
            label("PGN:", label_width),
            Span::raw(format!("{:<col_width$}", format!("0x{:04X}", pgn))),
            label("Priority:", label_width),
            Span::raw(format!("{}", priority)),
        ]),
        Line::from(vec![
            label("SA:", label_width),
            Span::raw(format!("{:<col_width$}", format!("0x{:02X}", sa))),
            label("DA:", label_width),
            Span::raw(format!("0x{:02X}", da)),
        ]),
        Line::from(vec![
            label("DLC:", label_width),
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
        .map(|sig| Line::from(vec![Span::raw(sig.name())]))
        .collect();
    frame.render_widget(
        Paragraph::new(sig_names).block(Block::default().borders(Borders::ALL).title("Signals")),
        area,
    );
}

fn render_signal_detail(
    frame: &mut Frame,
    area: Rect,
    msg: &dbc_rs::Message,
    sig: &dbc_rs::Signal,
) {
    /*(frame.render_widget(
        Paragraph::new("a").block(Block::default().borders(Borders::ALL).title("Signals")),
        area,
    );*/
    render_signal_header(frame, area, msg, sig);
}

fn render_signal_header(
    frame: &mut Frame,
    area: Rect,
    _msg: &dbc_rs::Message,
    sig: &dbc_rs::Signal,
) {
    let byte_order = match sig.byte_order() {
        ByteOrder::LittleEndian => "Little Endian (Intel format)",
        ByteOrder::BigEndian => "Big Endian (Motorola format)",
    };

    let label_width = 16;
    let col_width = 20;

    let lines = vec![
        Line::from(vec![label("Name:", label_width), Span::raw(sig.name())]),
        Line::from(vec![
            label("Signed:", label_width),
            Span::raw(format!("{}", !sig.is_unsigned())),
        ]),
        Line::from(vec![
            label("Byte Order:", label_width),
            Span::raw(byte_order),
        ]),
        Line::from(vec![
            label("Length:", label_width),
            Span::raw(format!("{:<col_width$}", sig.length())),
            label("Units:", label_width),
            Span::raw(sig.unit().unwrap_or("-").to_string()),
        ]),
        Line::from(vec![
            label("Factor:", label_width),
            Span::raw(format!("{:<col_width$}", sig.factor())),
            label("Offset:", label_width),
            Span::raw(format!("{}", sig.offset())),
        ]),
        Line::from(vec![
            label("Min:", label_width),
            Span::raw(format!("{:<col_width$}", sig.min())),
            label("Max:", label_width),
            Span::raw(format!("{}", sig.max())),
        ]),
        Line::from(vec![
            label("Comment:", label_width),
            Span::raw(sig.comment().unwrap_or("-")),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Signal")),
        area,
    );
}

fn label(text: &str, width: usize) -> Span<'static> {
    Span::styled(
        format!("{:<width$}", text),
        Style::default().fg(Color::Yellow),
    )
}
