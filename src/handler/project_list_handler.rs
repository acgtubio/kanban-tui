use crate::{
    event::{NavigationEvent, ProjectListEvent},
    state::app_state::AppState,
};

pub struct ProjectListHandler;

impl ProjectListHandler {
    pub fn handle_events(state: &mut AppState, event: ProjectListEvent) {
        match event {
            ProjectListEvent::Navigate(navigation_event) => {
                ProjectListHandler::handle_navigation(state, navigation_event)
            }
        }
    }

    fn handle_navigation(state: &mut AppState, navigation_event: NavigationEvent) {
        match navigation_event {
            NavigationEvent::FocusIn => state.open_selected_project(),
            NavigationEvent::Next => state.update_project_selection(1),
            NavigationEvent::Prev => state.update_project_selection(-1),
            NavigationEvent::FocusOut => (),
        }
    }
}
