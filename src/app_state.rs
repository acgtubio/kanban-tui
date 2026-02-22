use std::collections::HashMap;

use crate::components::{Task, TaskStatus};

pub enum Panes {
    Preview,
    Kanban,
}

pub struct AppState {
    pub tasks: HashMap<TaskStatus, Vec<Task>>,
    pub focused: Panes,
}

impl AppState {
    pub fn new() -> Self {
        let mut app_state = AppState {
            tasks: HashMap::new(),
            focused: Panes::Kanban,
        };
        app_state.tasks.insert(TaskStatus::Pending, vec![]);
        app_state.tasks.insert(TaskStatus::InProgress, vec![]);
        app_state.tasks.insert(TaskStatus::Completed, vec![]);

        app_state
    }
}
