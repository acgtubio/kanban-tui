use std::collections::HashMap;

use crate::{
    components::{Task, TaskConvertError, TaskPriority, TaskStatus},
    db::{Db, SqliteDb, TaskModel},
};

#[derive(PartialEq, Clone)]
pub enum Pane {
    Preview,
    MoveTaskModal,
    Kanban(TaskStatus),
}

#[derive(Debug, PartialEq, Clone)]
pub struct KanbanFocus {
    pub column: TaskStatus,
    pub task_idx: Option<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AddTaskModalFocus {
    pub current_field: TaskField,
    pub field_values: TaskFieldValues,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TaskField {
    Name,
    Description,
    TaskStatus,
    TaskPriority,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TaskFieldValues {
    name: String,
    description: String,
    task_status: TaskStatus,
    task_priority: TaskPriority,
}

impl Default for TaskFieldValues {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            task_status: TaskStatus::Pending,
            task_priority: TaskPriority::Low,
        }
    }
}

pub struct AddTaskState {
    pub focused_field: TaskField,
}

pub struct AppState {
    pub tasks: HashMap<TaskStatus, Vec<Task>>,
    pub active_pane: Pane,
    pub kanban_focus: Option<KanbanFocus>,
    pub modal_focus: Option<TaskStatus>,
    pub add_task_focus: Option<AddTaskModalFocus>,
    db: SqliteDb,
}

impl AppState {
    pub fn new(db: SqliteDb) -> Self {
        let mut app_state = AppState {
            tasks: HashMap::new(),
            active_pane: Pane::Kanban(TaskStatus::Pending),
            kanban_focus: None,
            modal_focus: None,
            add_task_focus: None,
            db: db,
        };

        app_state.tasks.insert(TaskStatus::Pending, vec![]);
        app_state.tasks.insert(TaskStatus::InProgress, vec![]);
        app_state.tasks.insert(TaskStatus::Completed, vec![]);

        app_state
    }

    pub fn init_tasks(&mut self) {
        let tasks_raw = self.db.get_tasks().expect("Unable to fetch kanban data.");

        let tasks = tasks_raw
            .iter()
            .map(|task| Task::from_task_model(task))
            .collect::<Result<Vec<Task>, TaskConvertError>>()
            .expect("Unable to convert to service models.");

        tasks.iter().for_each(|task| {
            let t = task.clone();
            self.add_task(t, task.status);
        });
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

    pub fn is_moving_task(&self) -> bool {
        self.active_pane == Pane::MoveTaskModal
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

    pub fn cycle_task_status_focus(&mut self) {
        if let Some(modal_focus) = self.modal_focus {
            self.modal_focus = match modal_focus {
                TaskStatus::Pending => Some(TaskStatus::InProgress),
                TaskStatus::InProgress => Some(TaskStatus::Completed),
                TaskStatus::Completed => Some(TaskStatus::Pending),
            };
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
            Pane::MoveTaskModal => self.active_pane.clone(),
        }
    }

    pub fn is_focused_add_task(&self) -> bool {
        self.add_task_focus != None
    }

    pub fn focus_add_task_modal(&mut self) {
        if self.is_focused_add_task() {
            return;
        }

        self.add_task_focus = Some(AddTaskModalFocus {
            current_field: TaskField::Name,
            field_values: TaskFieldValues::default(),
        });
    }

    pub fn cycle_add_task_field(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.current_field = match add_task_focus.current_field {
                TaskField::Name => TaskField::Description,
                TaskField::Description => TaskField::TaskStatus,
                TaskField::TaskStatus => TaskField::TaskPriority,
                TaskField::TaskPriority => TaskField::Name,
            }
        }
    }

    pub fn focus_kanban(&mut self) {
        if self.is_focused_kanban() {
            return;
        }

        let status = match self.active_pane {
            Pane::Kanban(task_status) => task_status,
            _ => return,
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
        if self.is_moving_task() {
            self.modal_focus = None;
            // TODO: Find a way to retain the previous task status.

            self.active_pane = Pane::Kanban(TaskStatus::Pending);
            return;
        }

        self.kanban_focus = None;
    }

    fn get_task_size_by_status(&mut self, status: &TaskStatus) -> Option<usize> {
        self.tasks.get(&status).map(|t| t.len())
    }

    fn get_status_by_pane(&self) -> Option<TaskStatus> {
        match self.active_pane {
            Pane::Kanban(task_status) => Some(task_status),
            _ => None,
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

    pub fn open_move_task_modal(&mut self) {
        self.active_pane = Pane::MoveTaskModal;
        self.modal_focus = Some(TaskStatus::Pending);
    }

    pub fn move_task(&mut self, task: Task, target_status: TaskStatus) {
        let new_task = Task {
            id: task.id,
            name: task.name,
            description: task.description,
            status: target_status,
            priority: task.priority,
        };

        self.update_task_on_db(new_task.clone());
        self.move_task_state(new_task, task.status);
    }

    // TODO: Handle error.
    fn update_task_on_db(&mut self, task: Task) {
        let _ = self.db.update_task(TaskModel::from_task(task));
    }

    fn move_task_state(&mut self, new_task: Task, from_status: TaskStatus) -> Option<Task> {
        let target_task_list = self.tasks.get_mut(&new_task.status)?;

        target_task_list.push(new_task.clone());

        let source_task_list = self.tasks.get_mut(&from_status)?;
        source_task_list.retain(|t| t.id != new_task.id);

        Some(new_task)
    }

    pub fn get_focused_task(&self) -> Option<Task> {
        let focus = self.kanban_focus.clone()?;
        let selected_index = focus.task_idx?;

        let tasks = self.tasks.get(&focus.column)?;

        let task = tasks.get(selected_index)?;

        Some(task.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::components::TaskPriority;

    use super::*;

    #[test]
    fn pending_task_size_should_1() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));

        assert_eq!(Some(1), app.get_task_size_by_status(&TaskStatus::Pending));
    }

    #[test]
    fn should_not_be_focused_kanban() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));

        assert_eq!(None, app.kanban_focus);
    }

    #[test]
    fn should_not_be_focused_kanban2() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.focus_kanban();
        app.remove_kanban_focus();

        assert_eq!(None, app.kanban_focus);
    }

    #[test]
    fn should_be_focused_on_kanban_pending() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
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
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_in_progress_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_in_progress_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));

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
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_in_progress_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_in_progress_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));

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
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));

        app.add_in_progress_task(Task::new(
            String::from("task2"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_in_progress_task(Task::new(
            String::from("task3"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));

        app.add_completed_task(Task::new(
            String::from("task4"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_completed_task(Task::new(
            String::from("task5"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_completed_task(Task::new(
            String::from("task6"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
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
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));

        app.add_in_progress_task(Task::new(
            String::from("task2"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_in_progress_task(Task::new(
            String::from("task3"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));

        app.add_completed_task(Task::new(
            String::from("task4"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_completed_task(Task::new(
            String::from("task5"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_completed_task(Task::new(
            String::from("task6"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
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
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));

        app.add_in_progress_task(Task::new(
            String::from("task2"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_in_progress_task(Task::new(
            String::from("task3"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));

        app.add_completed_task(Task::new(
            String::from("task4"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_completed_task(Task::new(
            String::from("task5"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.add_completed_task(Task::new(
            String::from("task6"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
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
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));

        assert_eq!(Some(TaskStatus::Pending), app.get_status_by_pane());
    }

    #[test]
    fn pane_should_be_in_progress() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.cycle_pane();

        assert_eq!(Some(TaskStatus::InProgress), app.get_status_by_pane());
    }

    #[test]
    fn pane_should_be_completed() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_pending_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        app.cycle_pane();
        app.cycle_pane();

        assert_eq!(Some(TaskStatus::Completed), app.get_status_by_pane());
    }

    #[test]
    fn cycle_field_should_be_description() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.focus_add_task_modal();
        app.cycle_add_task_field();

        let expected_value = AddTaskModalFocus {
            current_field: TaskField::Description,
            field_values: TaskFieldValues::default(),
        };

        assert_eq!(Some(expected_value), app.add_task_focus);
    }
}
