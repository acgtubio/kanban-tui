use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph},
};

use super::Component;

pub struct Tasks {
    tasks: Vec<String>,
    area: Option<Rect>,
}

impl Tasks {
    pub fn new() -> Self {
        Tasks {
            tasks: vec![],
            area: None,
        }
    }
}

impl Component for Tasks {
    fn draw(&mut self, frame: &mut Frame) {
        let area = match self.area {
            Some(a) => a,
            None => frame.area(),
        };

        let block = Block::bordered()
            .title("tasks")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let text = format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: "
        );

        let widget = Paragraph::new(text).block(block).fg(Color::Cyan).centered();

        frame.render_widget(widget, area);
    }

    fn get_layout(&mut self) -> ratatui::prelude::Constraint {
        Constraint::Percentage(70)
    }

    fn set_area(&mut self, area: Rect) {
        self.area = Some(area)
    }
}
