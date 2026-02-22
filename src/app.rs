use crate::{
    app_state::AppState,
    components::{Component, Kanban, Preview, Task, TaskStatus},
    event::{AppEvent, Event, EventHandler},
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout},
};

pub struct App {
    pub running: bool,
    pub events: EventHandler,
    pub state: AppState,
    pub kanban: Kanban,
    pub preview: Preview,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            kanban: Kanban::new(),
            preview: Preview::new(),
            state: AppState::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(self, terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.render(terminal)?;

        Ok(())
    }

    pub fn render(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        // TODO: Remove. This is for testing.
        self.add_test_tasks();

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
        if let Some(v) = self.state.tasks.get_mut(&TaskStatus::Pending) {
            v.push(Task::new(String::from("task1"), String::from("heyhey")));
            v.push(Task::new(String::from("task2"), String::from("heyhey2")));
            v.push(Task::new(String::from("task3"), String::from("heyhey3")));
        } else {
            self.state.tasks.insert(TaskStatus::Pending, vec![]);
        }
        if let Some(v) = self.state.tasks.get_mut(&TaskStatus::InProgress) {
            v.push(Task::new(String::from("ip1"), String::from("heyhey")));
            v.push(Task::new(String::from("ip2"), String::from("heyhey2")));
            v.push(Task::new(String::from("ip3"), String::from("heyhey3")));
        } else {
            self.state.tasks.insert(TaskStatus::InProgress, vec![]);
        }
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
            },
        }
        Ok(())
    }

    // Moves focus across different panes.
    fn cycle_focus(&mut self) {
        self.state.cycle_focus();
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Tab => self.events.send(AppEvent::SwitchWindow),
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
            .constraints([Constraint::Percentage(70), Constraint::Fill(1)])
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
