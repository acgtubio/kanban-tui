use crate::{
    event::{MoveTaskEvent, NavigationEvent},
    state::app_state::AppState,
};

pub struct MoveTaskHandler;

impl MoveTaskHandler {
    pub fn handle_events(state: &mut AppState, event: MoveTaskEvent) {
        match event {
            MoveTaskEvent::ConfirmMove => MoveTaskHandler::handle_move(state),
            MoveTaskEvent::Navigate(navigation_event) => {
                MoveTaskHandler::handle_nav_events(state, navigation_event)
            }
        }
    }

    fn handle_move(state: &mut AppState) {
        if let Some(target_status) = state.modal_focus
            && let Some(task) = state.get_focused_task()
        {
            state.move_task(task, target_status);
            state.remove_move_task_focus();
        }
    }

    fn handle_nav_events(state: &mut AppState, event: NavigationEvent) {
        match event {
            NavigationEvent::FocusIn => state.open_move_task_modal(),
            NavigationEvent::FocusOut => state.remove_move_task_focus(),
            NavigationEvent::Next => state.cycle_task_status_focus(),
            _ => (),
        }
    }
}
