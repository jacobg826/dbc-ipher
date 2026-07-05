use dbc_rs::Message;
use tui_tree_widget::TreeState;

pub enum Selection<'a> {
    None,
    Message(&'a Message),
    Signal(&'a Message, &'a dbc_rs::Signal),
}

pub fn resolve_selection<'a>(
    tree_state: &TreeState<String>,
    messages: &[&'a Message],
) -> Selection<'a> {
    match tree_state.selected() {
        [] => Selection::None,
        [msg_id] => messages
            .iter()
            .find(|m| &m.id().to_string() == msg_id)
            .map(|m| Selection::Message(m))
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
