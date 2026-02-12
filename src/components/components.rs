use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

pub trait Component {
    fn draw(&mut self, frame: &mut Frame);
    fn get_layout(&mut self) -> Constraint;
    fn set_area(&mut self, area: Rect);
    fn get_children_layout(&self) -> Layout;
}
