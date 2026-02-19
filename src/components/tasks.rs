use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use super::Component;
use super::Task;
use super::TaskStatus;

pub struct Tasks {
    tasks: Vec<Task>,
    area: Option<Rect>,
}

impl Tasks {
    pub fn new() -> Self {
        Tasks {
            tasks: vec![],
            area: None,
        }
    }

    fn get_widget_ui(&self, status: TaskStatus) -> impl Widget {
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

        Paragraph::new(text).block(block).fg(Color::Cyan).centered()
    }

    fn filter_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| task.get_status() == status)
            .collect()
    }
}

impl Component for Tasks {
    fn draw(&mut self, frame: &mut Frame) {
        let area = match self.area {
            Some(a) => a,
            None => frame.area(),
        };

        let layout = self.get_children_layout().split(area);

        let widget = self.get_widget_ui();
        let widget2 = self.get_widget_ui();
        let widget3 = self.get_widget_ui();

        frame.render_widget(widget, layout[0]);
        frame.render_widget(widget2, layout[1]);
        frame.render_widget(widget3, layout[2]);
    }

    fn get_layout(&mut self) -> ratatui::prelude::Constraint {
        Constraint::Percentage(70)
    }

    fn set_area(&mut self, area: Rect) {
        self.area = Some(area)
    }

    fn get_children_layout(&self) -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Fill(1),
                Constraint::Percentage(30),
            ])
    }
}
