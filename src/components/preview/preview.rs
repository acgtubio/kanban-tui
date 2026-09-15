use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use crate::{
    components::{Component, task::TaskCard},
    state::app_state::AppState,
    theme::create_bordered_block,
};

pub struct Preview {}

impl Preview {
    pub fn new() -> Self {
        Preview {}
    }
}

impl Component for Preview {
    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &mut AppState) {
        let block = create_bordered_block().title(" Preview ");
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let Some(task) = state.get_focused_task() else {
            let placeholder = Paragraph::new("No task selected").alignment(Alignment::Center);
            frame.render_widget(placeholder, inner_area);
            return;
        };

        let layout = self.get_children_layout().split(inner_area);

        let prefix_style = TaskCard::get_prefix_style(task.priority);
        let name = Paragraph::new(Line::from(vec![
            Span::raw(format!("{} ", task.priority.short_str())).style(prefix_style),
            Span::raw(task.name.clone()).style(Style::default().add_modifier(Modifier::BOLD)),
        ]));
        let status = Paragraph::new(format!("Status: {}", task.status.to_readable_string()));
        let priority = Paragraph::new(format!("Priority: {}", task.priority.to_readable_string()));
        let description = Paragraph::new(task.description.clone()).wrap(Wrap { trim: true });

        frame.render_widget(name, layout[0]);
        frame.render_widget(status, layout[1]);
        frame.render_widget(priority, layout[2]);
        frame.render_widget(description, layout[4]);
    }

    fn get_children_layout(&self) -> Layout {
        Layout::default().direction(Direction::Vertical).constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
    }
}
