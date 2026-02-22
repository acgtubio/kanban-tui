use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState},
};

use crate::app_state::{AppState, Panes};

use super::Component;
use super::TaskStatus;

pub struct Kanban {
    pending_state: ListState,
    in_progress_state: ListState,
    completed_state: ListState,
}

impl Kanban {
    pub fn new() -> Self {
        Kanban {
            pending_state: ListState::default(),
            in_progress_state: ListState::default(),
            completed_state: ListState::default(),
        }
    }

    fn get_widget_ui<'a>(&self, state: &'a AppState, status: TaskStatus) -> List<'a> {
        let title = match status {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Completed => "Completed",
        };

        let mut color = Style::new().white();
        if state.focused == Panes::Kanban(status) {
            color = Style::new().yellow();
        };

        let block = Block::bordered()
            .border_style(color)
            .title(title)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let mut items = vec![];
        if let Some(pending_tasks) = &state.tasks.get(&status) {
            items = pending_tasks
                .iter()
                .enumerate()
                .map(|(_, item)| ListItem::from(item.name.as_str()))
                .collect::<Vec<_>>();
        }

        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        list
    }
}

impl Component for Kanban {
    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &mut AppState) {
        let mut block = Block::new().title_alignment(Alignment::Left);
        if state.focused != Panes::Preview {
            block = block.borders(Borders::ALL).border_type(BorderType::Rounded);
        }

        let inner_area = block.inner(area);
        let layout = self.get_children_layout().split(inner_area);

        let widget = self.get_widget_ui(state, TaskStatus::Pending);
        let widget2 = self.get_widget_ui(state, TaskStatus::InProgress);
        let widget3 = self.get_widget_ui(state, TaskStatus::Completed);

        frame.render_stateful_widget(widget, layout[0], &mut self.pending_state);
        frame.render_stateful_widget(widget2, layout[1], &mut self.in_progress_state);
        frame.render_stateful_widget(widget3, layout[2], &mut self.completed_state);
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
