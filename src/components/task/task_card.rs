use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};

use crate::{
    components::{Task, TaskPriority},
    state::app_state::{AppState, KanbanFocus},
    theme::get_highlight_color,
};

pub struct TaskCard {}

impl TaskCard {
    const PREFIX_WIDTH: u16 = 5;

    fn get_content_layout() -> Layout {
        Layout::horizontal([Constraint::Length(TaskCard::PREFIX_WIDTH), Constraint::Fill(1)])
    }

    /// Estimates how many terminal rows `text` will wrap to when rendered in a column
    /// `column_width` cells wide (i.e. the raw area passed to `render_card`, before the
    /// name/prefix split). Used by `Kanban::render_column` to size each task's row before
    /// rendering, since row heights must be decided ahead of the per-row content split.
    pub(crate) fn estimate_wrapped_lines(text: &str, column_width: u16) -> u16 {
        let text_width = column_width.saturating_sub(TaskCard::PREFIX_WIDTH).max(1);
        let char_count = text.chars().count() as u16;
        char_count.div_ceil(text_width).max(1)
    }

    pub(crate) fn get_prefix_style(prio: TaskPriority) -> Style {
        let style = Style::default().add_modifier(Modifier::BOLD);
        let style = match prio {
            TaskPriority::Normal => style.fg(Color::Green),
            TaskPriority::Low => style.fg(Color::White),
            TaskPriority::High => style.fg(Color::Rgb(255, 165, 0)),
            TaskPriority::Critical => style.fg(Color::Red),
        };

        style
    }

    fn get_block_style(focus: &Option<KanbanFocus>, idx: isize) -> Style {
        let style = if let Some(kanban_focus) = focus
            && let Some(focus_idx) = kanban_focus.task_idx
            && focus_idx == idx
        {
            get_highlight_color()
        } else {
            Style::default()
        };

        style
    }

    pub fn render_card(frame: &mut Frame, area: Rect, state: &AppState, task: &Task, idx: isize) {
        let block_style = TaskCard::get_block_style(&state.kanban_focus, idx);
        let prefix_style = TaskCard::get_prefix_style(task.get_priority());

        let block = Block::default().style(block_style);
        let layout = TaskCard::get_content_layout().split(area);

        let prio = task.get_priority().short_str();

        let prefix = Paragraph::new(vec![Line::from(vec![Span::raw(prio).style(prefix_style)])])
            .alignment(Alignment::Center);
        let task_title = Paragraph::new(task.name.clone()).wrap(Wrap { trim: true });

        frame.render_widget(block, area);
        frame.render_widget(prefix, layout[0]);
        frame.render_widget(task_title, layout[1]);
    }
}
