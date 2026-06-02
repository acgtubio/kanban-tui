use crate::{
    event::{MainScreenEvent, NavigationEvent},
    state::app_state::AppState,
};

pub struct MainScreenHandler;

impl MainScreenHandler {
    pub fn handle_events(state: &mut AppState, event: MainScreenEvent) {
        match event {
            MainScreenEvent::Navigate(navigation_event) => {
                MainScreenHandler::handle_navigation(state, navigation_event)
            }
        }
    }

    pub fn handle_navigation(state: &mut AppState, navigation_event: NavigationEvent) {
        match navigation_event {
            NavigationEvent::Next => state.cycle_pane(),
            _ => (),
        }
    }
}
