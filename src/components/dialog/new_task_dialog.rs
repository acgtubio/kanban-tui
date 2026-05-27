use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{BorderType, Borders},
};

use crate::{app_state::AppState, theme::create_base_block};

pub struct NewTaskDialog {}

impl NewTaskDialog {
    pub fn render_new_task_dialog(frame: &mut Frame, area: Rect, state: &AppState) {
        let base_block = create_base_block()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Add Task")
            .title_alignment(Alignment::Center);

        let base_layout = NewTaskDialog::get_dialog_layout().split(area);

        frame.render_widget(base_block, area);
    }

    fn get_dialog_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1); 4])
    }

    fn get_title_layout() -> Layout {
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
