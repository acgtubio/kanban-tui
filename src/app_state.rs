use std::collections::HashMap;

use crate::components::{Task, TaskStatus};

pub enum Panes {
    Preview,
    Kanban,
}

pub struct AppState {
    pub tasks: HashMap<TaskStatus, Task>,
    pub focused: Panes,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            tasks: HashMap::new(),
            focused: Panes::Kanban,
        }
    }
}
