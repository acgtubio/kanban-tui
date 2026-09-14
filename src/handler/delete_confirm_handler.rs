use crate::{
    event::{DeleteConfirmEvent, NavigationEvent},
    state::app_state::AppState,
};

pub struct DeleteConfirmHandler;

impl DeleteConfirmHandler {
    pub fn handle_events(state: &mut AppState, event: DeleteConfirmEvent) {
        match event {
            DeleteConfirmEvent::ConfirmDelete => state.confirm_delete_focused_task(),
            DeleteConfirmEvent::Navigate(navigation_event) => {
                DeleteConfirmHandler::handle_nav_events(state, navigation_event)
            }
        }
    }

    fn handle_nav_events(state: &mut AppState, event: NavigationEvent) {
        match event {
            NavigationEvent::FocusIn => state.open_delete_confirm_modal(),
            NavigationEvent::FocusOut => state.close_delete_confirm_modal(),
            NavigationEvent::Next | NavigationEvent::Prev => state.toggle_delete_confirm_focus(),
        }
    }
}
