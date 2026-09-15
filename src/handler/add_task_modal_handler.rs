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
            AddTaskEvent::EditFocusIn => state.focus_edit_task_modal(),
        }
    }

    pub fn handle_char_input(state: &mut AppState, ch: char) {
        if let Some(add_task_modal_state) = &state.add_task_focus {
            match add_task_modal_state.current_field {
                TaskField::Name => state.insert_char_at_name_cursor(ch),
                TaskField::Description => state.insert_char_at_description_cursor(ch),
                _ => (),
            }
        }
    }

    pub fn handle_char_pop(state: &mut AppState) {
        if let Some(add_task_modal_state) = &state.add_task_focus {
            match add_task_modal_state.current_field {
                TaskField::Name => state.backspace_at_name_cursor(),
                TaskField::Description => state.backspace_at_description_cursor(),
                _ => (),
            }
        }
    }

    fn handle_cursor_left(state: &mut AppState) {
        if let Some(add_task_modal_state) = &state.add_task_focus {
            match add_task_modal_state.current_field {
                TaskField::Name => state.move_name_cursor_left(),
                TaskField::Description => state.move_description_cursor_left(),
                _ => (),
            }
        }
    }

    fn handle_cursor_right(state: &mut AppState) {
        if let Some(add_task_modal_state) = &state.add_task_focus {
            match add_task_modal_state.current_field {
                TaskField::Name => state.move_name_cursor_right(),
                TaskField::Description => state.move_description_cursor_right(),
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
            InputEvent::PrevChar => AddTaskModalHandler::handle_cursor_left(state),
            InputEvent::NextChar => AddTaskModalHandler::handle_cursor_right(state),
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
            state.save_task_form();
            state.remove_add_task_focus();
            ()
        }
    }
}
