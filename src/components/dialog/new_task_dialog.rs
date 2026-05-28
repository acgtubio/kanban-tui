use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{BorderType, Borders, Clear, Paragraph},
};

use crate::{state::app_state::AppState, theme::create_base_block};

pub struct NewTaskDialog {}

impl NewTaskDialog {
    pub fn render_new_task_dialog(frame: &mut Frame, area: Rect, state: &AppState) {
        let base_block = create_base_block()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Add Task")
            .title_alignment(Alignment::Center);

        let base_layout = NewTaskDialog::get_dialog_layout().split(area);

        frame.render_widget(Clear, area);
        frame.render_widget(base_block, area);
        NewTaskDialog::draw_title(frame, base_layout[1]);
    }

    fn draw_title(frame: &mut Frame, area: Rect) {
        let layout = NewTaskDialog::get_name_layout().split(area);

        let label = Paragraph::new("Task");

        frame.render_widget(label, layout[1]);
    }

    fn get_dialog_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1); 4])
    }

    fn get_name_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1); 3])
    }

    fn get_description_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1); 3])
    }

    fn get_option_area_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1); 2])
    }
}
