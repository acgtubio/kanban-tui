use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::{
    app_state::{AppState, KanbanFocus},
    components::{Task, TaskPriority},
};

pub struct TaskCard {}

impl TaskCard {
    fn get_content_layout() -> Layout {
        Layout::horizontal([Constraint::Length(5), Constraint::Fill(1)])
    }

    fn get_prefix_style(prio: TaskPriority) -> Style {
        let style = Style::default().add_modifier(Modifier::BOLD);
        let style = match prio {
            TaskPriority::Normal => style.fg(Color::Green),
            TaskPriority::Low => style.fg(Color::White),
            TaskPriority::High => style.fg(Color::Rgb(255, 165, 0)),
            TaskPriority::Critical => style.fg(Color::Red),
        };

        style
    }

    fn get_block_style(focus: &Option<KanbanFocus>, idx: usize) -> Style {
        let style = if let Some(kanban_focus) = focus
            && let Some(focus_idx) = kanban_focus.task_idx
            && focus_idx == idx
        {
            Style::default().bg(Color::Rgb(40, 50, 65))
        } else {
            Style::default()
        };

        style
    }

    pub fn render_card(frame: &mut Frame, area: Rect, state: &AppState, task: &Task, idx: usize) {
        let block_style = TaskCard::get_block_style(&state.kanban_focus, idx);
        let prefix_style = TaskCard::get_prefix_style(task.get_priority());

        let block = Block::default().style(block_style);
        let layout = TaskCard::get_content_layout().split(area);

        let prio = task.get_priority().short_str();

        let prefix = Paragraph::new(vec![Line::from(vec![Span::raw(prio).style(prefix_style)])])
            .alignment(Alignment::Center);
        let task_title = Paragraph::new(task.name.clone());

        frame.render_widget(block, area);
        frame.render_widget(prefix, layout[0]);
        frame.render_widget(task_title, layout[1]);
    }
}
