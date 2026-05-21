use crate::{app::App, db::SqliteDb};

pub mod app;
pub mod app_state;
pub mod components;
pub mod db;
pub mod event;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let db = SqliteDb::new().expect("Cannot connect to database.");
    let app = App::new(db);

    let result = app.run(terminal);

    ratatui::restore();
    result
}
