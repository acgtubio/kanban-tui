use crate::{
    app_state::AppState,
    components::{Component, Kanban, Preview, Task, TaskConvertError, TaskPriority, TaskStatus},
    db::{Db, SqliteDb},
    event::{AppEvent, Event, EventHandler},
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout},
};

pub struct App {
    pub db: SqliteDb,
    pub running: bool,
    pub events: EventHandler,
    pub state: AppState,
    pub kanban: Kanban,
    pub preview: Preview,
}

impl App {
    pub fn new(db: SqliteDb) -> Self {
        Self {
            db: db,
            running: true,
            events: EventHandler::new(),
            kanban: Kanban::new(),
            preview: Preview::new(),
            state: AppState::new(),
        }
    }

    pub fn init_tasks(&mut self) {
        let tasks_raw = self.db.get_tasks().expect("Unable to fetch kanban data.");

        // TODO: Remove. This is for testing.
        if tasks_raw.len() == 0 {
            let _ = self.db.test_init();
        }

        let tasks = tasks_raw
            .iter()
            .map(|task| Task::from_task_model(task))
            .collect::<Result<Vec<Task>, TaskConvertError>>()
            .expect("Unable to convert to service models.");

        tasks.iter().for_each(|task| {
            let t = task.clone();
            self.state.add_task(t, task.status);
        });
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

                self.kanban.draw(frame, chunks[0], &mut self.state);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    // TODO: Remove
    fn add_test_tasks(&mut self) {
        self.state.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        self.state.add_pending_task(Task::new(
            String::from("task2"),
            String::from("heyhey2"),
            TaskPriority::Normal,
        ));
        self.state.add_pending_task(Task::new(
            String::from("task3"),
            String::from("heyhey3"),
            TaskPriority::High,
        ));

        self.state.add_in_progress_task(Task::new(
            String::from("ip1"),
            String::from("heyhey"),
            TaskPriority::Critical,
        ));
        self.state.add_in_progress_task(Task::new(
            String::from("ip2"),
            String::from("heyhey2"),
            TaskPriority::Critical,
        ));
        self.state.add_in_progress_task(Task::new(
            String::from("ip3"),
            String::from("heyhey3"),
            TaskPriority::Critical,
        ));
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event)
                    if key_event.kind == crossterm::event::KeyEventKind::Press =>
                {
                    self.handle_key_event(key_event)?
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
                AppEvent::SwitchWindow => self.cycle_focus(),
                AppEvent::FocusIn => self.state.focus_kanban(),
                AppEvent::FocusOut => self.state.remove_kanban_focus(),
            },
        }
        Ok(())
    }

    // Moves focus across different panes.
    fn cycle_focus(&mut self) {
        if !self.state.is_focused_kanban() {
            self.state.cycle_focus();
        } else {
            self.state.cycle_kanban_focus();
        }
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Tab => self.events.send(AppEvent::SwitchWindow),
            KeyCode::Enter => self.events.send(AppEvent::FocusIn),
            KeyCode::Esc => self.events.send(AppEvent::FocusOut),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            // KeyCode::Right => self.events.send(AppEvent::Increment),
            // KeyCode::Left => self.events.send(AppEvent::Decrement),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
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
