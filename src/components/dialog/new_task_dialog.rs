use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Clear, Paragraph, Wrap},
};

use crate::{
    components::{TaskPriority, TaskStatus},
    state::{add_task_state::AddTaskModalState, app_state::AppState, task_field::TaskField},
    theme::create_bordered_block,
};

pub struct NewTaskDialog {}

impl NewTaskDialog {
    pub fn render_new_task_dialog(frame: &mut Frame, area: Rect, state: &AppState) {
        let base_block = create_bordered_block().border_style(Style::new().yellow());

        let base_layout = NewTaskDialog::get_dialog_layout().split(area);

        frame.render_widget(Clear, area);
        frame.render_widget(base_block, area);

        let Some(add_task_state) = &state.add_task_focus else {
            return;
        };

        NewTaskDialog::draw_title(frame, base_layout[0], add_task_state);
        NewTaskDialog::draw_description(frame, base_layout[1], add_task_state);
        NewTaskDialog::draw_status_field(frame, base_layout[2], add_task_state);
        NewTaskDialog::draw_priority_field(frame, base_layout[3], add_task_state);
    }

    fn create_default_block<'a>(field_for: TaskField, current_field: TaskField) -> Block<'a> {
        let mut block = create_bordered_block();

        if current_field == field_for {
            block = block.border_style(Style::default().yellow());
        }

        block
    }

    fn draw_title(frame: &mut Frame, area: Rect, state: &AddTaskModalState) {
        let block =
            NewTaskDialog::create_default_block(TaskField::Name, state.current_field.clone());
        let layout = NewTaskDialog::get_name_layout().split(area);

        let label = Paragraph::new("[1] Task");
        let value = Paragraph::new(state.field_values.name.clone());

        frame.render_widget(block, area);
        frame.render_widget(label, layout[0]);
        frame.render_widget(value, layout[1]);
    }

    fn draw_description(frame: &mut Frame, area: Rect, state: &AddTaskModalState) {
        let block = NewTaskDialog::create_default_block(
            TaskField::Description,
            state.current_field.clone(),
        );
        let layout = NewTaskDialog::get_description_layout().split(area);

        let label = Paragraph::new("[2] Description");
        let value =
            Paragraph::new(state.field_values.description.clone()).wrap(Wrap { trim: true });

        frame.render_widget(block, area);
        frame.render_widget(label, layout[0]);
        frame.render_widget(value, layout[1]);
    }

    fn draw_status_field(frame: &mut Frame, area: Rect, state: &AddTaskModalState) {
        let block =
            NewTaskDialog::create_default_block(TaskField::TaskStatus, state.current_field.clone());
        let layout = NewTaskDialog::get_option_area_layout().split(area);
        let option_layout = NewTaskDialog::get_status_option_layout().split(layout[1]);

        let label = Paragraph::new("[3] Task Status");

        frame.render_widget(block, area);
        frame.render_widget(label, layout[0]);

        let selected = state.field_values.task_status;
        NewTaskDialog::draw_status_option(frame, option_layout[0], TaskStatus::Pending, selected);
        NewTaskDialog::draw_status_option(
            frame,
            option_layout[1],
            TaskStatus::InProgress,
            selected,
        );
        NewTaskDialog::draw_status_option(frame, option_layout[2], TaskStatus::Completed, selected);
    }

    fn draw_priority_field(frame: &mut Frame, area: Rect, state: &AddTaskModalState) {
        let block = NewTaskDialog::create_default_block(
            TaskField::TaskPriority,
            state.current_field.clone(),
        );
        let layout = NewTaskDialog::get_option_area_layout().split(area);
        let option_layout = NewTaskDialog::get_priority_option_layout().split(layout[1]);

        let label = Paragraph::new("[4] Task Priority");

        frame.render_widget(block, area);
        frame.render_widget(label, layout[0]);

        let selected = state.field_values.task_priority;
        NewTaskDialog::draw_priority_option(frame, option_layout[0], TaskPriority::Low, selected);
        NewTaskDialog::draw_priority_option(
            frame,
            option_layout[1],
            TaskPriority::Normal,
            selected,
        );
        NewTaskDialog::draw_priority_option(frame, option_layout[2], TaskPriority::High, selected);
        NewTaskDialog::draw_priority_option(
            frame,
            option_layout[3],
            TaskPriority::Critical,
            selected,
        );
    }

    fn draw_priority_option(
        frame: &mut Frame,
        area: Rect,
        priority: TaskPriority,
        selected: TaskPriority,
    ) {
        let value = priority.to_readable_string();
        let is_selected = if selected == priority { "[x]" } else { "[ ]" };
        let value_widget = Paragraph::new(format!("{0} {1}", is_selected, value));

        frame.render_widget(value_widget, area);
    }

    fn draw_status_option(frame: &mut Frame, area: Rect, status: TaskStatus, selected: TaskStatus) {
        let value = status.to_readable_string();
        let is_selected = if selected == status { "[x]" } else { "[ ]" };
        let value_widget = Paragraph::new(format!("{0} {1}", is_selected, value));

        frame.render_widget(value_widget, area);
    }

    fn get_status_option_layout() -> Layout {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1); 3])
    }

    fn get_priority_option_layout() -> Layout {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1); 4])
    }

    fn get_dialog_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .horizontal_margin(1)
            .vertical_margin(1)
    }

    fn get_name_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1); 3])
            .horizontal_margin(1)
    }

    fn get_description_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .horizontal_margin(1)
    }

    fn get_option_area_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1); 2])
            .horizontal_margin(1)
    }
}
