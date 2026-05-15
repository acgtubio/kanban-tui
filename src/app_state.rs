use std::collections::HashMap;

use crate::components::{Task, TaskStatus};

#[derive(PartialEq)]
pub enum Pane {
    Preview,
    Kanban(TaskStatus),
}

pub struct AppState {
    pub tasks: HashMap<TaskStatus, Vec<Task>>,
    pub active_pane: Pane,
    pub kanban_focus: Option<KanbanFocus>,
}

pub struct KanbanFocus {
    pub column: TaskStatus,
    pub task: Option<usize>,
}

impl AppState {
    pub fn new() -> Self {
        let mut app_state = AppState {
            tasks: HashMap::new(),
            active_pane: Pane::Kanban(TaskStatus::Pending),
            kanban_focus: None,
        };

        app_state.tasks.insert(TaskStatus::Pending, vec![]);
        app_state.tasks.insert(TaskStatus::InProgress, vec![]);
        app_state.tasks.insert(TaskStatus::Completed, vec![]);

        app_state
    }

    pub fn cycle_focus(&mut self) {
        match self.kanban_focus {
            Some(_) => self.cycle_kanban_focus(),
            None => self.cycle_pane(),
        };
    }

    pub fn cycle_kanban_focus(&mut self) {
        let Some(status) = self.get_status_by_pane() else {
            return;
        };

        let Some(max_len) = self.get_task_size_by_status(&status) else {
            return;
        };

        let Some(focus) = &mut self.kanban_focus else {
            return;
        };

        let Some(index) = focus.task else {
            focus.task = Some(0);
            return;
        };

        if index < max_len {
            focus.task = Some(index + 1);
        } else {
            focus.task = Some(0);
        }
    }

    pub fn cycle_pane(&mut self) {
        self.active_pane = match self.active_pane {
            Pane::Preview => Pane::Kanban(TaskStatus::Pending),
            Pane::Kanban(task_status) => match task_status {
                TaskStatus::Pending => Pane::Kanban(TaskStatus::InProgress),
                TaskStatus::InProgress => Pane::Kanban(TaskStatus::Completed),
                TaskStatus::Completed => Pane::Kanban(TaskStatus::Pending),
            },
        }
    }

    pub fn focus_kanban(&mut self) {
        let status = match self.active_pane {
            Pane::Kanban(task_status) => task_status,
            Pane::Preview => return,
        };

        let Some(task_length) = self.get_task_size_by_status(&status) else {
            return;
        };

        if task_length == 0 {
            return;
        }

        self.kanban_focus = Some(KanbanFocus {
            column: status,
            task: Some(0),
        });
    }

    fn get_task_size_by_status(&mut self, status: &TaskStatus) -> Option<usize> {
        self.tasks.get(&status).map(|t| t.len())
    }

    fn get_status_by_pane(&self) -> Option<TaskStatus> {
        match self.active_pane {
            Pane::Kanban(task_status) => Some(task_status),
            Pane::Preview => None,
        }
    }
}
