use crate::{
    components::{Component, Kanban, MoveDialog, NewTaskDialog, Preview},
    db::SqliteDb,
    event::{AppEvent, Event, EventHandler},
    handler::AddTaskModalHandler,
    state::app_state::AppState,
    theme::create_base_block,
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
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
                    self.handle_key_event(key_event)
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
                AppEvent::SwitchContext => self.cycle_focus(),
                AppEvent::FocusIn => self.state.focus_kanban(),
                AppEvent::FocusOut => self.state.remove_kanban_focus(),
                AppEvent::MoveTask => self.open_move_dialog(),
                AppEvent::NewTask => self.open_new_task_dialog(),
                AppEvent::ConfirmMove => self.handle_move(),
                AppEvent::KeyInput(ch) => self.handle_char_input(ch),
                AppEvent::Save => self.handle_save(),
                AppEvent::PopChar => self.handle_pop_char(),
            },
        }
        Ok(())
    }

    fn handle_save(&mut self) {
        if self.state.is_focused_add_task() {
            self.state.save_new_task();
            self.events.send(AppEvent::FocusOut);
            ()
        }
    }

    fn handle_char_input(&mut self, ch: char) {
        if !self.state.is_focused_add_task() {
            return;
        }

        if let Some(field) = self.state.get_add_task_focused_field() {
            match ch {
                'l' => AddTaskModalHandler::next_option(&mut self.state, field),
                'h' => AddTaskModalHandler::prev_option(&mut self.state, field),
                _ => (),
            }
        }

        AddTaskModalHandler::handle_char_input(&mut self.state, ch)
    }

    fn handle_pop_char(&mut self) {
        if self.state.is_focused_add_task() {
            AddTaskModalHandler::handle_char_pop(&mut self.state)
        }
    }

    // Moves focus across different panes.
    fn cycle_focus(&mut self) {
        if self.state.is_moving_task() {
            self.state.cycle_task_status_focus();
            return;
        }
        if self.state.is_focused_add_task() {
            self.state.cycle_add_task_field();
            return;
        }
        if !self.state.is_focused_kanban() {
            self.state.cycle_focus()
        } else {
            self.state.cycle_kanban_focus()
        }
    }

    fn open_move_dialog(&mut self) {
        if !self.state.is_focused_kanban() {
            return;
        }
        self.state.open_move_task_modal();
    }

    fn open_new_task_dialog(&mut self) {
        if self.state.is_focused_add_task() {
            return;
        }
        self.state.focus_add_task_modal();
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.state.is_focused_add_task() {
            self.handle_add_task_events(key_event);
            return;
        }

        self.handle_base_key_events(key_event);
    }

    pub fn handle_add_task_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(ch) => self.events.send(AppEvent::KeyInput(ch)),
            KeyCode::Enter => self.events.send(AppEvent::Save),
            KeyCode::Backspace => self.events.send(AppEvent::PopChar),
            KeyCode::Esc => self.events.send(AppEvent::FocusOut),
            KeyCode::Tab => self.events.send(AppEvent::SwitchContext),
            _ => {}
        }
    }

    pub fn handle_base_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('m') => self.events.send(AppEvent::MoveTask),
            KeyCode::Char('n') => self.events.send(AppEvent::NewTask),
            KeyCode::Tab => self.events.send(AppEvent::SwitchContext),
            KeyCode::Enter => self.handle_enter(),
            KeyCode::Esc => self.events.send(AppEvent::FocusOut),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            _ => {}
        }
    }

    fn handle_enter(&mut self) {
        if self.state.is_moving_task() {
            self.events.send(AppEvent::ConfirmMove);
            return;
        }

        self.events.send(AppEvent::FocusIn);
    }

    fn handle_move(&mut self) {
        if let Some(target_status) = self.state.modal_focus
            && let Some(task) = self.state.get_focused_task()
        {
            self.state.move_task(task, target_status);
            self.state.remove_kanban_focus();
        }
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
