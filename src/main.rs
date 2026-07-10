mod app;
mod keybinding;
mod selection;
mod ui;

use crate::app::App;
use crate::selection::Selection;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let path = std::env::args()
        .nth(1)
        .expect("usage: dbc-viewer <file.dbc>");

    let content = std::fs::read_to_string(&path)?;
    let dbc = dbc_rs::Dbc::parse(&content)?;

    let mut app = App::new(dbc);

    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
