use std::collections::HashMap;

use crate::components::{Task, TaskStatus};

#[derive(PartialEq)]
pub enum Panes {
    Preview,
    Kanban(TaskStatus),
}

pub struct AppState {
    pub tasks: HashMap<TaskStatus, Vec<Task>>,
    pub focused: Panes,
}

impl AppState {
    pub fn new() -> Self {
        let mut app_state = AppState {
            tasks: HashMap::new(),
            focused: Panes::Kanban(TaskStatus::Pending),
        };
        app_state.tasks.insert(TaskStatus::Pending, vec![]);
        app_state.tasks.insert(TaskStatus::InProgress, vec![]);
        app_state.tasks.insert(TaskStatus::Completed, vec![]);

        app_state
    }

    pub fn cycle_focus(&mut self) {
        self.focused = match self.focused {
            Panes::Preview => Panes::Kanban(TaskStatus::Pending),
            Panes::Kanban(task_status) => match task_status {
                TaskStatus::Pending => Panes::Kanban(TaskStatus::InProgress),
                TaskStatus::InProgress => Panes::Kanban(TaskStatus::Completed),
                TaskStatus::Completed => Panes::Kanban(TaskStatus::Pending),
            },
        }
    }
}
