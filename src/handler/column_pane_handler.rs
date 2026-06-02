use crate::{
    event::{KanbanScreenEvent, NavigationEvent},
    state::app_state::AppState,
};

pub struct ColumnHandler;

impl ColumnHandler {
    pub fn handle_events(state: &mut AppState, event: KanbanScreenEvent) {
        match event {
            KanbanScreenEvent::Navigate(navigation_event) => {
                ColumnHandler::handle_navigation(state, navigation_event)
            }
            KanbanScreenEvent::Delete => {
                state.remove_selected_task();
                ()
            }
        }
    }

    pub fn handle_navigation(state: &mut AppState, navigation_event: NavigationEvent) {
        match navigation_event {
            NavigationEvent::FocusIn => state.focus_kanban(),
            NavigationEvent::FocusOut => state.remove_kanban_focus(),
            NavigationEvent::Next => state.cycle_kanban_focus(),
            NavigationEvent::Prev => todo!(),
        }
    }
}
