use crossterm::event::{self, KeyCode};
use dbc_rs::Message;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::widgets::{List, ListState};
use ratatui::{DefaultTerminal, Frame};
use tui_tree_widget::{Tree, TreeItem, TreeState};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut tree_state = TreeState::default();

    let path = std::env::args()
        .nth(1)
        .expect("usage: dbc-viewer <file.dbc>");

    let content = std::fs::read_to_string(&path)?;
    let dbc = dbc_rs::Dbc::parse(&content)?;

    let mut app = App { dbc, tree_state };

    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}

struct App {
    dbc: dbc_rs::Dbc,
    tree_state: TreeState<String>,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
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
    render_tree(frame, inner_layout[0], &messages, &mut app.tree_state);

    let selected = resolve_selection(&app.tree_state, &messages);
    render_detail(frame, inner_layout[1], selected);
}

enum Selection<'a> {
    None,
    Message(&'a Message),
    Signal(&'a Message, &'a dbc_rs::Signal),
}

fn resolve_selection<'a>(
    tree_state: &TreeState<String>,
    messages: &[&'a Message],
) -> Selection<'a> {
    match tree_state.selected() {
        [] => Selection::None,
        [msg_id] => messages
            .iter()
            .find(|m| &m.id().to_string() == msg_id)
            .map(|m| Selection::Message(*m))
            .unwrap_or(Selection::None),
        [msg_id, sig_name, ..] => messages
            .iter()
            .find(|m| &m.id().to_string() == msg_id)
            .and_then(|m| {
                m.signals()
                    .iter()
                    .find(|s| s.name() == sig_name)
                    .map(|s| Selection::Signal(m, s))
            })
            .unwrap_or(Selection::None),
    }
}

fn render_tree(
    frame: &mut Frame,
    area: Rect,
    items: &Vec<&Message>,
    tree_state: &mut TreeState<String>,
) {
    let outer_block = Block::default().borders(Borders::ALL);
    let inner_area = outer_block.inner(area);

    frame.render_widget(outer_block, area);
    let tree_items: Vec<TreeItem<String>> = items
        .iter()
        .map(|msg| {
            if msg.signals().len() > 0 {
                let signals = msg
                    .signals()
                    .iter()
                    .map(|sig| TreeItem::new_leaf(sig.name().to_string(), sig.name()))
                    .collect();
                TreeItem::new(msg.id().to_string(), msg.name(), signals)
                    .expect("signal ids cannot contain duplicates")
            } else {
                TreeItem::new_leaf(msg.id().to_string(), msg.name())
            }
        })
        .collect();

    if tree_state.selected().is_empty() {
        if let Some(first) = items.first() {
            tree_state.select(vec![first.id().to_string()]);
        }
    }

    let tree = Tree::new(&tree_items)
        .expect("message identifiers must be unique")
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(tree, inner_area, tree_state);
}

fn render_detail(frame: &mut Frame, area: Rect, selection: Selection) {
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
    message: &dbc_rs::Message,
    sig: &dbc_rs::Signal,
) {
    frame.render_widget(
        Paragraph::new("a").block(Block::default().borders(Borders::ALL).title("Signals")),
        area,
    );
}

trait Listable {
    fn display(&self) -> String;
}

impl Listable for &dbc_rs::Message {
    fn display(&self) -> String {
        format!("0x{:08X} ({} signals)", self.id(), self.signals().len())
    }
}

impl Listable for &dbc_rs::Signal {
    fn display(&self) -> String {
        self.name().to_string()
    }
}
