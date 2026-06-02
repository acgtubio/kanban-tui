use crate::{
    event::{AddTaskEvent, InputEvent, NavigationEvent},
    state::{app_state::AppState, task_field::TaskField},
};

pub struct AddTaskModalHandler;

impl AddTaskModalHandler {
    pub fn handle_events(state: &mut AppState, event: AddTaskEvent) {
        match event {
            AddTaskEvent::Save => AddTaskModalHandler::handle_save(state),
            AddTaskEvent::Input(input_event) => {
                AddTaskModalHandler::handle_input_event(state, input_event)
            }
            AddTaskEvent::Navigate(navigation_event) => {
                AddTaskModalHandler::handle_nav_event(state, navigation_event)
            }
        }
    }

    pub fn handle_char_input(state: &mut AppState, ch: char) {
        if let Some(add_task_modal_state) = &mut state.add_task_focus {
            match add_task_modal_state.current_field {
                TaskField::Name => state.add_to_name(ch),
                TaskField::Description => state.add_to_description(ch),
                _ => (),
            }
        }
    }

    pub fn handle_char_pop(state: &mut AppState) {
        if let Some(add_task_modal_state) = &mut state.add_task_focus {
            match add_task_modal_state.current_field {
                TaskField::Name => state.pop_name(),
                TaskField::Description => state.pop_description(),
                _ => (),
            }
        }
    }

    pub fn next_option(state: &mut AppState, field: TaskField) {
        match field {
            TaskField::TaskStatus => state.next_status(),
            TaskField::TaskPriority => state.next_priority(),
            _ => (),
        }
    }

    pub fn prev_option(state: &mut AppState, field: TaskField) {
        match field {
            TaskField::TaskStatus => state.prev_status(),
            TaskField::TaskPriority => state.prev_priority(),
            _ => (),
        }
    }

    fn handle_nav_event(state: &mut AppState, event: NavigationEvent) {
        match event {
            NavigationEvent::FocusIn => state.focus_add_task_modal(),
            NavigationEvent::FocusOut => state.remove_add_task_focus(),
            NavigationEvent::Next => state.cycle_add_task_field(),
            _ => (),
        }
    }

    fn handle_input_event(state: &mut AppState, event: InputEvent) {
        match event {
            InputEvent::Key(ch) => AddTaskModalHandler::handle_input(state, ch),
            InputEvent::PopChar => AddTaskModalHandler::handle_char_pop(state),
            InputEvent::PrevChar => todo!(),
            InputEvent::NextChar => todo!(),
        }
    }

    fn handle_input(state: &mut AppState, ch: char) {
        if !state.is_focused_add_task() {
            return;
        }

        if let Some(field) = state.get_add_task_focused_field() {
            match ch {
                'l' => AddTaskModalHandler::next_option(state, field),
                'h' => AddTaskModalHandler::prev_option(state, field),
                _ => (),
            }
        }

        AddTaskModalHandler::handle_char_input(state, ch)
    }

    fn handle_save(state: &mut AppState) {
        if state.is_focused_add_task() {
            state.save_new_task();
            state.remove_add_task_focus();
            ()
        }
    }
}
