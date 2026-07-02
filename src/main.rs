use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::widgets::{List, ListDirection, ListState};
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut list_state = ListState::default().with_selected(Some(0));

    let path = std::env::args()
        .nth(1)
        .expect("usage: dbc-viewer <file.dbc>");

    let content = std::fs::read_to_string(&path)?;
    let dbc = dbc_rs::Dbc::parse(&content)?;

    let mut app = App { dbc, list_state };

    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}

struct App {
    dbc: dbc_rs::Dbc,
    list_state: ListState,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| render(self, frame))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('j') | KeyCode::Down => self.list_state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.list_state.select_previous(),
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    _ => {}
                }
            }
        }
    }
}

fn render(app: &mut App, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(outer_layout[0]);

    frame.render_widget(
        Paragraph::new("Detail").block(Block::new().title(" [1] Details ").borders(Borders::ALL)),
        inner_layout[1],
    );

    frame.render_widget(
        Paragraph::new("Bottom").block(Block::new().title(" [2] Console ").borders(Borders::ALL)),
        outer_layout[1],
    );

    let messages: Vec<_> = app.dbc.messages().iter().collect();
    render_list(frame, inner_layout[0], &messages, &mut app.list_state);

    let selected = app.list_state.selected().map(|i| messages[i]);
    render_detail(frame, inner_layout[1], selected);
}

fn render_list<T: Listable>(
    frame: &mut Frame,
    area: Rect,
    items: &[T],
    list_state: &mut ListState,
) {
    let list_items: Vec<String> = items.iter().map(|i| i.display()).collect();
    let list = List::new(list_items)
        .block(Block::default().title(" [0] Tree ").borders(Borders::ALL))
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, list_state);
}

fn render_detail(frame: &mut Frame, area: Rect, message: Option<&dbc_rs::Message>) {
    if let Some(msg) = message {
        render_message_header(frame, area, msg);
    }
}

fn render_message_header(frame: &mut Frame, area: Rect, msg: &dbc_rs::Message) {
    let id = msg.id();
    let pgn = (id >> 8) & 0xFFFF;
    let sa = id & 0xFF;
    let da = (id >> 8) & 0xFF;
    let priority = (id >> 26) & 0x7;

    let label_width = 10; // tune once, applies everywhere
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

trait Listable {
    fn display(&self) -> String;
}

struct Message {
    identifier: u32,
    signals: Vec<Signal>,
}

impl Listable for &dbc_rs::Message {
    fn display(&self) -> String {
        format!("0x{:08X} ({} signals)", self.id(), self.signals().len())
    }
}

struct Signal {
    name: String,
    position: u32,
}
