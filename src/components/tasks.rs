use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
};

use crate::{
    app_state::{AppState, Panes},
    components::Task,
};

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

    fn get_widget_ui<'a>(&self, tasks: &'a Vec<Task>) -> List<'a> {
        let block = Block::bordered()
            .title("tasks")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let items = tasks
            .iter()
            .enumerate()
            .map(|(_, item)| ListItem::from(item.name.as_str()))
            .collect::<Vec<_>>();

        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        list
    }
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

        if let Some(pending_tasks) = &state.tasks.get(&TaskStatus::Pending) {
            let widget = self.get_widget_ui(pending_tasks);
            frame.render_stateful_widget(widget, layout[0], &mut self.pending_state);
        }

        if let Some(inprogress_tasks) = &state.tasks.get(&TaskStatus::InProgress) {
            let widget = self.get_widget_ui(inprogress_tasks);
            frame.render_stateful_widget(widget, layout[1], &mut self.in_progress_state);
        }

        if let Some(completed_tasks) = &state.tasks.get(&TaskStatus::Completed) {
            let widget = self.get_widget_ui(completed_tasks);
            frame.render_stateful_widget(widget, layout[2], &mut self.completed_state);
        }
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
