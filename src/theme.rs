use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders},
};

pub fn get_base_color() -> Style {
    Style::default().bg(Color::Rgb(40, 50, 65))
}

pub fn get_modal_color() -> Style {
    Style::default().bg(Color::Rgb(20, 30, 45))
}

pub fn get_highlight_color() -> Style {
    Style::default().bg(Color::Rgb(80, 90, 105))
}

pub fn create_base_block<'a>() -> Block<'a> {
    Block::new().style(get_base_color())
}

pub fn create_base_modal_block<'a>() -> Block<'a> {
    Block::new().style(get_modal_color())
}

pub fn create_base_highlighted_block<'a>() -> Block<'a> {
    create_base_block().border_style(Style::new().yellow())
}

pub fn create_bordered_block<'a>() -> Block<'a> {
    create_base_block()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title_alignment(Alignment::Center)
}

pub fn create_bordered_modal_block<'a>() -> Block<'a> {
    create_base_modal_block()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title_alignment(Alignment::Center)
}
