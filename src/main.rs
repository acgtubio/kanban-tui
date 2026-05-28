use crate::{app::App, db::SqliteDb};

pub mod app;
pub mod components;
pub mod db;
pub mod event;
pub mod state;
pub mod theme;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let db = SqliteDb::new().expect("Cannot connect to database.");
    let _ = db.init_db();

    let mut app = App::new(db);
    app.init_tasks();

    let result = app.run(terminal);

    ratatui::restore();
    result
}
