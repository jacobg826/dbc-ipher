use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::widgets::{List, ListDirection, ListState};
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let data = dummy_data();
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
    //frame.render_widget("hello world", frame.area());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new("Bottom").block(Block::new().borders(Borders::ALL)),
        layout[1],
    );

    let messages: Vec<_> = app.dbc.messages().iter().collect();
    render_list(frame, layout[0], &messages, &mut app.list_state);
}

fn render_list<T: Listable>(
    frame: &mut Frame,
    area: Rect,
    items: &[T],
    list_state: &mut ListState,
) {
    let list_items: Vec<String> = items.iter().map(|i| i.display()).collect();
    let list = List::new(list_items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, list_state);
}

fn dummy_data() -> Vec<Message> {
    let mut data: Vec<Message> = Vec::new();
    let mut msg1: Vec<Signal> = Vec::new();
    let mut msg2: Vec<Signal> = Vec::new();
    let sig1 = Signal {
        name: "DM1".to_string(),
        position: 15,
    };
    let sig2 = Signal {
        name: "DM2".to_string(),
        position: 16,
    };
    msg1.push(sig1);
    msg2.push(sig2);
    let dm1 = Message {
        identifier: 0x18feca00,
        signals: msg1,
    };
    let dm2 = Message {
        identifier: 0x18f00127,
        signals: msg2,
    };

    data.push(dm1);
    data.push(dm2);

    return data;
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
