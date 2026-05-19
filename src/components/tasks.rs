use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState},
};

use crate::app_state::{AppState, Pane};

use super::Component;
use super::TaskStatus;

pub struct Kanban {
    pub states: HashMap<TaskStatus, ListState>,
}

impl Kanban {
    pub fn new() -> Self {
        let mut kanban = Kanban {
            states: HashMap::new(),
        };

        kanban
            .states
            .insert(TaskStatus::Pending, ListState::default());
        kanban
            .states
            .insert(TaskStatus::InProgress, ListState::default());
        kanban
            .states
            .insert(TaskStatus::Completed, ListState::default());

        kanban
    }

    fn get_items(state: &AppState, status: TaskStatus) -> Vec<String> {
        let items = state
            .tasks
            .get(&status)
            .map(|tasks| {
                tasks
                    .iter()
                    .map(|item| item.name.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        items
    }

    fn build_list(items: Vec<String>) -> List<'static> {
        let list = items.into_iter().map(ListItem::new).collect::<Vec<_>>();

        List::new(list)
    }

    fn render_column(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        state: &AppState,
        status: TaskStatus,
    ) {
        let items = Self::get_items(state, status);
        let list = Self::build_list(items);

        let list_state = self.states.get_mut(&status).expect("State should exist.");

        let color = if state.active_pane == Pane::Kanban(status) {
            Style::new().yellow()
        } else {
            Style::new().white()
        };

        let block = Block::bordered()
            .border_style(color)
            .title(status.to_string())
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let list = list
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(list, area, list_state);
    }
}

impl Component for Kanban {
    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &mut AppState) {
        let mut block = Block::new().title_alignment(Alignment::Left);
        if state.active_pane != Pane::Preview {
            block = block.borders(Borders::ALL).border_type(BorderType::Rounded);
        }

        let inner_area = block.inner(area);
        let layout = self.get_children_layout().split(inner_area);

        self.render_column(frame, layout[0], state, TaskStatus::Pending);
        self.render_column(frame, layout[1], state, TaskStatus::InProgress);
        self.render_column(frame, layout[2], state, TaskStatus::Completed);
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
