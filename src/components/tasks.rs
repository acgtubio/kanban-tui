use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, ListState},
};

use crate::{
    app_state::{AppState, Pane},
    components::task::TaskCard,
    db::SqliteDb,
};

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

    fn render_column(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        state: &AppState,
        status: TaskStatus,
    ) {
        let tasks = state.tasks.get(&status).expect("State should exist.");

        let color = if state.active_pane == Pane::Kanban(status) {
            Style::new().yellow()
        } else {
            Style::new().white()
        };

        let block = Block::bordered()
            .border_style(color)
            .title(status.to_readable_string())
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let inner = block.inner(area);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); tasks.len()])
            .split(inner);

        for (i, task) in tasks.iter().enumerate() {
            TaskCard::render_card(frame, layout[i], state, task, i);
        }

        frame.render_widget(block, area);
    }
}

impl Component for Kanban {
    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &mut AppState) {
        let mut block = Block::new().title_alignment(Alignment::Left);
        if state.active_pane != Pane::Preview {
            block = block.borders(Borders::ALL).border_type(BorderType::Rounded);
        }

        let inner_area = block.inner(area);
        if let Some(focus) = &state.kanban_focus {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1)])
                .split(inner_area);

            self.render_column(frame, layout[0], state, focus.column);
        } else {
            let layout = self.get_children_layout().split(inner_area);

            self.render_column(frame, layout[0], state, TaskStatus::Pending);
            self.render_column(frame, layout[1], state, TaskStatus::InProgress);
            self.render_column(frame, layout[2], state, TaskStatus::Completed);
        }
    }

    fn get_children_layout(&self) -> Layout {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
    }
}
