use crate::state::{app_state::AppState, task_field::TaskField};

pub struct AddTaskModalHandler;

impl AddTaskModalHandler {
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
}
