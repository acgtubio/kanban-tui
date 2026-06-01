use crate::{
    event::{KanbanScreenEvent, NavigationEvent},
    state::app_state::AppState,
};

pub struct KanbanScreenHandler;

impl KanbanScreenHandler {
    pub fn handle_events(state: &mut AppState, event: KanbanScreenEvent) {
        match event {
            KanbanScreenEvent::Navigate(navigation_event) => {
                KanbanScreenHandler::handle_navigation(state, navigation_event)
            }
            KanbanScreenEvent::MoveTask => state.focus_kanban(),
            KanbanScreenEvent::CreateTask => state.focus_add_task_modal(),
            KanbanScreenEvent::Delete => todo!(),
        }
    }

    pub fn handle_navigation(state: &mut AppState, navigation_event: NavigationEvent) {
        match navigation_event {
            NavigationEvent::Next => state.cycle_kanban_focus(),
            NavigationEvent::Prev => todo!(),
            _ => (),
        }
    }
}
