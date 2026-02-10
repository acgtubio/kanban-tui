use ratatui::{
    Frame,
    layout::{Constraint, Rect},
};

pub trait Component {
    fn draw(&mut self, frame: &mut Frame);
    fn get_layout(&mut self) -> Constraint;
    fn set_area(&mut self, area: Rect);
}
