use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, ListState, Paragraph, Widget},
};

use crate::app_state::{AppState, Panes};

use super::Component;
use super::TaskStatus;

pub struct Kanban {
    list_state: ListState,
}

impl Kanban {
    pub fn new() -> Self {
        Kanban {
            list_state: ListState::default(),
        }
    }

    fn get_widget_ui(&self, status: TaskStatus, state: &AppState) -> impl Widget {
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

    // fn filter_by_status(&self, status: TaskStatus) -> Vec<&Task> {
    //     self.tasks
    //         .iter()
    //         .filter(|task| task.get_status() == status)
    //         .collect()
    // }
}

impl Component for Kanban {
    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &mut AppState) {
        let border_color = match state.focused {
            Panes::Preview => Style::new().blue(),
            Panes::Kanban => Style::new().red(),
        };

        let block = Block::bordered()
            .border_style(border_color)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let inner_area = block.inner(area);

        let layout = self.get_children_layout().split(inner_area);

        let widget = self.get_widget_ui(TaskStatus::Pending, state);
        let widget2 = self.get_widget_ui(TaskStatus::InProgress, state);
        let widget3 = self.get_widget_ui(TaskStatus::Completed, state);

        frame.render_widget(block, area);
        frame.render_widget(widget, layout[0]);
        // frame.render_stateful_widget(widget, layout[0]);
        frame.render_widget(widget2, layout[1]);
        frame.render_widget(widget3, layout[2]);
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
