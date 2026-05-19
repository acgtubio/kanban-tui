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

#[derive(Debug, PartialEq)]
pub struct KanbanFocus {
    pub column: TaskStatus,
    pub task_idx: Option<usize>,
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

    pub fn is_focused_kanban(&self) -> bool {
        self.kanban_focus != None
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

        let Some(index) = focus.task_idx else {
            focus.task_idx = Some(0);
            return;
        };

        if index < max_len - 1 {
            focus.task_idx = Some(index + 1);
        } else {
            focus.task_idx = Some(0);
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
            task_idx: Some(0),
        });
    }

    pub fn remove_kanban_focus(&mut self) {
        self.kanban_focus = None;
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

    pub fn add_task(&mut self, task: Task, status: TaskStatus) {
        if let Some(task_list) = self.tasks.get_mut(&status) {
            task_list.push(task);
        }
    }

    pub fn add_pending_task(&mut self, task: Task) {
        self.add_task(task, TaskStatus::Pending);
    }

    pub fn add_in_progress_task(&mut self, task: Task) {
        self.add_task(task, TaskStatus::InProgress);
    }

    pub fn add_completed_task(&mut self, task: Task) {
        self.add_task(task, TaskStatus::Completed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_task_size_should_1() {
        let mut app = AppState::new();
        app.add_pending_task(Task::new(String::from("task1"), String::from("heyhey")));

        assert_eq!(Some(1), app.get_task_size_by_status(&TaskStatus::Pending));
    }

    #[test]
    fn should_not_be_focused_kanban() {
        let mut app = AppState::new();
        app.add_pending_task(Task::new(String::from("task1"), String::from("heyhey")));

        assert_eq!(None, app.kanban_focus);
    }

    #[test]
    fn should_not_be_focused_kanban2() {
        let mut app = AppState::new();
        app.add_pending_task(Task::new(String::from("task1"), String::from("heyhey")));
        app.focus_kanban();
        app.remove_kanban_focus();

        assert_eq!(None, app.kanban_focus);
    }

    #[test]
    fn should_be_focused_on_kanban_pending() {
        let mut app = AppState::new();
        app.add_pending_task(Task::new(String::from("task1"), String::from("heyhey")));
        app.focus_kanban();

        assert_eq!(
            Some(KanbanFocus {
                column: TaskStatus::Pending,
                task_idx: Some(0)
            }),
            app.kanban_focus
        );
    }

    #[test]
    fn should_be_focused_on_kanban_in_progres_idx1() {
        let mut app = AppState::new();
        app.add_pending_task(Task::new(String::from("task1"), String::from("heyhey")));
        app.add_in_progress_task(Task::new(String::from("task2"), String::from("heyhey2")));
        app.add_in_progress_task(Task::new(String::from("task3"), String::from("heyhey3")));

        app.cycle_focus();

        app.focus_kanban();
        app.cycle_kanban_focus();

        assert_eq!(
            Some(KanbanFocus {
                column: TaskStatus::InProgress,
                task_idx: Some(1)
            }),
            app.kanban_focus
        );
    }

    #[test]
    fn should_be_focused_on_kanban_in_progres_idx0() {
        let mut app = AppState::new();
        app.add_pending_task(Task::new(String::from("task1"), String::from("heyhey")));
        app.add_in_progress_task(Task::new(String::from("task2"), String::from("heyhey2")));
        app.add_in_progress_task(Task::new(String::from("task3"), String::from("heyhey3")));

        app.cycle_focus();

        app.focus_kanban();
        app.cycle_kanban_focus();
        app.cycle_kanban_focus();

        assert_eq!(
            Some(KanbanFocus {
                column: TaskStatus::InProgress,
                task_idx: Some(0)
            }),
            app.kanban_focus
        );
    }

    #[test]
    fn should_be_focused_on_kanban_completed_idx2() {
        let mut app = AppState::new();
        app.add_pending_task(Task::new(String::from("task1"), String::from("heyhey")));

        app.add_in_progress_task(Task::new(String::from("task2"), String::from("heyhey2")));
        app.add_in_progress_task(Task::new(String::from("task3"), String::from("heyhey3")));

        app.add_completed_task(Task::new(String::from("task4"), String::from("heyhey2")));
        app.add_completed_task(Task::new(String::from("task5"), String::from("heyhey3")));
        app.add_completed_task(Task::new(String::from("task6"), String::from("heyhey3")));
        app.cycle_focus();
        app.cycle_focus();

        app.focus_kanban();

        app.cycle_kanban_focus();
        app.cycle_kanban_focus();

        assert_eq!(
            Some(KanbanFocus {
                column: TaskStatus::Completed,
                task_idx: Some(2)
            }),
            app.kanban_focus
        );
    }

    #[test]
    fn should_be_focused_on_kanban_completed_idx1() {
        let mut app = AppState::new();
        app.add_pending_task(Task::new(String::from("task1"), String::from("heyhey")));

        app.add_in_progress_task(Task::new(String::from("task2"), String::from("heyhey2")));
        app.add_in_progress_task(Task::new(String::from("task3"), String::from("heyhey3")));

        app.add_completed_task(Task::new(String::from("task4"), String::from("heyhey2")));
        app.add_completed_task(Task::new(String::from("task5"), String::from("heyhey3")));
        app.add_completed_task(Task::new(String::from("task6"), String::from("heyhey3")));
        app.cycle_focus();
        app.cycle_focus();

        app.focus_kanban();

        app.cycle_kanban_focus();

        assert_eq!(
            Some(KanbanFocus {
                column: TaskStatus::Completed,
                task_idx: Some(1)
            }),
            app.kanban_focus
        );
    }

    #[test]
    fn should_be_focused_on_kanban_completed_idx0() {
        let mut app = AppState::new();
        app.add_pending_task(Task::new(String::from("task1"), String::from("heyhey")));

        app.add_in_progress_task(Task::new(String::from("task2"), String::from("heyhey2")));
        app.add_in_progress_task(Task::new(String::from("task3"), String::from("heyhey3")));

        app.add_completed_task(Task::new(String::from("task4"), String::from("heyhey2")));
        app.add_completed_task(Task::new(String::from("task5"), String::from("heyhey3")));
        app.add_completed_task(Task::new(String::from("task6"), String::from("heyhey3")));
        app.cycle_focus();
        app.cycle_focus();

        app.focus_kanban();

        app.cycle_kanban_focus();
        app.cycle_kanban_focus();
        app.cycle_kanban_focus();

        assert_eq!(
            Some(KanbanFocus {
                column: TaskStatus::Completed,
                task_idx: Some(0)
            }),
            app.kanban_focus
        );
    }

    #[test]
    fn pane_should_be_pending() {
        let mut app = AppState::new();
        app.add_pending_task(Task::new(String::from("task1"), String::from("heyhey")));

        assert_eq!(Some(TaskStatus::Pending), app.get_status_by_pane());
    }

    #[test]
    fn pane_should_be_in_progress() {
        let mut app = AppState::new();
        app.add_pending_task(Task::new(String::from("task1"), String::from("heyhey")));
        app.cycle_pane();

        assert_eq!(Some(TaskStatus::InProgress), app.get_status_by_pane());
    }

    #[test]
    fn pane_should_be_completed() {
        let mut app = AppState::new();
        app.add_pending_task(Task::new(String::from("task1"), String::from("heyhey")));
        app.cycle_pane();
        app.cycle_pane();

        assert_eq!(Some(TaskStatus::Completed), app.get_status_by_pane());
    }
}
