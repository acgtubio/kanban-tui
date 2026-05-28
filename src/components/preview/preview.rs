use ratatui::{Frame, layout::Rect};

use crate::{components::Component, state::app_state::AppState};

pub struct Preview {}

impl Preview {
    pub fn new() -> Self {
        Preview {}
    }
}

impl Component for Preview {
    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &mut AppState) {
        todo!()
    }

    fn get_children_layout(&self) -> ratatui::prelude::Layout {
        todo!()
    }
}
