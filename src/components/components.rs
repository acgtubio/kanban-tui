use ratatui::{
    Frame,
    layout::{Layout, Rect},
};

use crate::state::app_state::AppState;

pub trait Component {
    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &mut AppState);
    fn get_children_layout(&self) -> Layout;
}
