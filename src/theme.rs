use ratatui::{
    style::{Color, Style},
    widgets::Block,
};

pub fn get_base_color() -> Style {
    Style::default().bg(Color::Rgb(40, 50, 65))
}

pub fn get_highlight_color() -> Style {
    Style::default().bg(Color::Rgb(80, 90, 105))
}

pub fn create_base_block<'a>() -> Block<'a> {
    Block::new().style(get_base_color())
}
