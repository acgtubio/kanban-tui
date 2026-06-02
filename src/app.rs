use crate::{
    components::{Component, Kanban, MoveDialog, NewTaskDialog, Preview},
    db::SqliteDb,
    event::{
        AddTaskEvent, AppEvent, Event, EventHandler, KanbanScreenEvent, MainScreenEvent,
        MoveTaskEvent,
    },
    event_mux::handle_events,
    handler::{
        AddTaskModalHandler, column_pane_handler::ColumnHandler,
        main_screen_handler::MainScreenHandler, move_task_handler::MoveTaskHandler,
    },
    state::app_state::AppState,
    theme::create_base_block,
};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Padding,
};

pub struct App {
    pub running: bool,
    pub events: EventHandler,
    pub state: AppState,
    pub kanban: Kanban,
    pub preview: Preview,
}

impl App {
    pub fn new(db: SqliteDb) -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            kanban: Kanban::new(),
            preview: Preview::new(),
            state: AppState::new(db),
        }
    }

    pub fn init_tasks(&mut self) {
        self.state.init_tasks();
    }

    /// Run the application's main loop.
    pub fn run(self, terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.render(terminal)?;

        Ok(())
    }

    pub fn render(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                let chunks = self.get_layout().split(frame.area());

                let modal_block = create_base_block().padding(Padding::uniform(5));
                frame.render_widget(modal_block, frame.area());

                // Kanban
                self.kanban.draw(frame, chunks[0], &mut self.state);

                // Move task modal
                if self.state.is_moving_task() {
                    let area = frame.area();
                    let modal_area = App::get_modal_area(area, 80, 10);

                    MoveDialog::render_move_dialog(frame, modal_area, &mut self.state);
                }

                // Add task modal
                if self.state.is_focused_add_task() {
                    let area = frame.area();
                    let modal_area = App::get_modal_area(area, 60, 20);

                    NewTaskDialog::render_new_task_dialog(frame, modal_area, &mut self.state);
                }
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn get_modal_area(area: Rect, width: u16, height: u16) -> Rect {
        let mid_x = (area.x + area.width) / 2;
        let mid_y = (area.y + area.height) / 2;

        let start_x = mid_x - (width / 2);
        let start_y = mid_y - (height / 2);

        let modal_area = Rect {
            x: start_x,
            y: start_y,
            height: height,
            width: width,
        };

        modal_area
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event)
                    if key_event.kind == crossterm::event::KeyEventKind::Press =>
                {
                    handle_events(&mut self.events, key_event, self.state.active_pane.clone());
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::KanbanScreenEvent(event) => self.handle_kanban_screen_event(event),
                AppEvent::AddTaskEvent(event) => self.handle_add_task_event(event),
                AppEvent::MoveTaskEvent(move_task_event) => {
                    self.handle_move_task_event(move_task_event)
                }
                AppEvent::Quit => self.quit(),
                AppEvent::MainScreen(main_screen_event) => {
                    self.handle_main_screen_event(main_screen_event)
                }
            },
        }
        Ok(())
    }

    fn handle_main_screen_event(&mut self, event: MainScreenEvent) {
        MainScreenHandler::handle_events(&mut self.state, event);
    }

    fn handle_kanban_screen_event(&mut self, event: KanbanScreenEvent) {
        ColumnHandler::handle_events(&mut self.state, event);
    }

    fn handle_move_task_event(&mut self, event: MoveTaskEvent) {
        MoveTaskHandler::handle_events(&mut self.state, event);
    }

    fn handle_add_task_event(&mut self, event: AddTaskEvent) {
        AddTaskModalHandler::handle_events(&mut self.state, event);
    }

    fn get_layout(&self) -> Layout {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Fill(1)])
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modal_area_should_not_fail() {
        let r = Rect {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        };

        let res = App::get_modal_area(r, 80, 10);

        assert_eq!(920, res.x);
    }
}
