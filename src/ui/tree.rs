use dbc_rs::Message;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};
use tui_tree_widget::{Tree, TreeItem, TreeState};

pub fn render_tree(
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
            if !msg.signals().is_empty() {
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

    if tree_state.selected().is_empty()
        && let Some(first) = items.first()
    {
        tree_state.select(vec![first.id().to_string()]);
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
