use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Offset, Rect},
    style::Style,
    widgets::{Block, Clear, Paragraph},
};

use crate::{
    components::TaskStatus,
    state::app_state::AppState,
    theme::{create_base_highlighted_block, get_highlight_color},
};

pub struct MoveDialog {}

impl MoveDialog {
    pub fn render_move_dialog(frame: &mut Frame, area: Rect, state: &AppState) {
        let block = create_base_highlighted_block().title_alignment(Alignment::Left);

        let Some(task_to_move) = state.get_focused_task() else {
            return;
        };

        let layout = MoveDialog::get_dialog_layout().split(area);
        let title = Paragraph::new(format!("Moving \"{0}\"", task_to_move.name))
            .alignment(Alignment::Center);

        frame.render_widget(Clear, area);
        frame.render_widget(block, area);
        frame.render_widget(title, layout[0]);

        for (i, status) in TaskStatus::ALL.iter().enumerate() {
            let item_layout = MoveDialog::get_item_layout(layout[1], i as u16 + 1);
            let block_style = MoveDialog::get_block_style(state.modal_focus, status.clone());

            let option = Paragraph::new(status.to_readable_string());
            let item_block = Block::new().style(block_style);

            frame.render_widget(item_block, item_layout);
            frame.render_widget(option, item_layout.offset(Offset { x: 3, y: 0 }));
        }
    }

    fn get_block_style(focus: Option<TaskStatus>, task_status: TaskStatus) -> Style {
        let style = if let Some(status) = focus
            && status == task_status
        {
            get_highlight_color()
        } else {
            Style::default()
        };

        style
    }

    fn get_item_layout(area: Rect, offset: u16) -> Rect {
        Rect {
            x: area.x + 3,
            y: area.y + offset,
            width: area.width - 6,
            height: 1,
        }
    }

    fn get_dialog_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(2), Constraint::Fill(1)])
    }
}
