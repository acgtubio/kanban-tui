use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    event::{
        AddTaskEvent, AppEvent, EventHandler, InputEvent, KanbanScreenEvent, MoveTaskEvent,
        NavigationEvent,
    },
    state::app_state::Pane,
};

pub fn handle_add_task_events(event_handler: &mut EventHandler, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char(ch) => event_handler.send(AppEvent::AddTaskEvent(AddTaskEvent::Input(
            InputEvent::Key(ch),
        ))),
        KeyCode::Enter => event_handler.send(AppEvent::AddTaskEvent(AddTaskEvent::Save)),
        KeyCode::Backspace => event_handler.send(AppEvent::AddTaskEvent(AddTaskEvent::Input(
            InputEvent::PopChar,
        ))),
        KeyCode::Esc => event_handler.send(AppEvent::AddTaskEvent(AddTaskEvent::Navigate(
            NavigationEvent::FocusOut,
        ))),
        KeyCode::Tab => event_handler.send(AppEvent::AddTaskEvent(AddTaskEvent::Navigate(
            NavigationEvent::Next,
        ))),
        _ => {}
    }
}

pub fn handle_move_task_event(event_handler: &mut EventHandler, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Tab => event_handler.send(AppEvent::MoveTaskEvent(MoveTaskEvent::Navigate(
            NavigationEvent::Next,
        ))),
        KeyCode::Enter => event_handler.send(AppEvent::MoveTaskEvent(MoveTaskEvent::Navigate(
            NavigationEvent::FocusIn,
        ))),
        _ => {}
    }
}

pub fn handle_kanban_event(event_handler: &mut EventHandler, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => event_handler.send(AppEvent::Quit),
        KeyCode::Enter => event_handler.send(AppEvent::KanbanScreenEvent(
            KanbanScreenEvent::Navigate(NavigationEvent::FocusIn),
        )),
        KeyCode::Tab => event_handler.send(AppEvent::KanbanScreenEvent(
            KanbanScreenEvent::Navigate(NavigationEvent::Next),
        )),
        KeyCode::Char('m') => event_handler.send(AppEvent::MoveTaskEvent(MoveTaskEvent::Navigate(
            NavigationEvent::FocusIn,
        ))),
        KeyCode::Char('n') => {
            event_handler.send(AppEvent::KanbanScreenEvent(KanbanScreenEvent::CreateTask))
        }
        KeyCode::Char('d') => {
            event_handler.send(AppEvent::KanbanScreenEvent(KanbanScreenEvent::Delete))
        }
        _ => {}
    }
}

pub fn handle_events(event_handler: &mut EventHandler, key_event: KeyEvent, current_pane: Pane) {
    match current_pane {
        Pane::Preview => todo!(),
        Pane::MoveTaskModal => handle_move_task_event(event_handler, key_event),
        Pane::AddTask => handle_add_task_events(event_handler, key_event),
        Pane::Kanban(_) => handle_kanban_event(event_handler, key_event),
        Pane::Column => todo!(),
    }
}
