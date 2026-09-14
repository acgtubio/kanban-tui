use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Clear, Paragraph},
};

use crate::{
    state::app_state::AppState,
    theme::{create_base_highlighted_block, get_highlight_color},
};

pub struct DeleteConfirmDialog {}

impl DeleteConfirmDialog {
    pub fn render_delete_confirm_dialog(frame: &mut Frame, area: Rect, state: &AppState) {
        let Some(task) = state.get_focused_task() else {
            return;
        };

        let block = create_base_highlighted_block().title_alignment(Alignment::Center);
        let layout = DeleteConfirmDialog::get_dialog_layout().split(area);
        let title = Paragraph::new(format!("Archive \"{0}\"?", task.name))
            .alignment(Alignment::Center);

        frame.render_widget(Clear, area);
        frame.render_widget(block, area);
        frame.render_widget(title, layout[0]);

        let options_layout = DeleteConfirmDialog::get_options_layout().split(layout[1]);
        DeleteConfirmDialog::render_option(
            frame,
            options_layout[0],
            "Yes",
            state.delete_confirm_focus == Some(true),
        );
        DeleteConfirmDialog::render_option(
            frame,
            options_layout[1],
            "No",
            state.delete_confirm_focus == Some(false),
        );
    }

    fn render_option(frame: &mut Frame, area: Rect, label: &str, is_focused: bool) {
        let style = if is_focused {
            get_highlight_color()
        } else {
            Style::default()
        };

        frame.render_widget(Block::new().style(style), area);
        frame.render_widget(Paragraph::new(label).alignment(Alignment::Center), area);
    }

    fn get_options_layout() -> Layout {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1); 2])
    }

    fn get_dialog_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(2), Constraint::Fill(1)])
    }
}
