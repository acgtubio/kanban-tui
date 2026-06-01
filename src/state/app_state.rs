use std::collections::HashMap;

use crate::{
    components::{Task, TaskConvertError, TaskStatus},
    db::{Db, SqliteDb, TaskModel},
    state::{
        add_task_state::AddTaskModalState, task_field::TaskField, task_field_value::TaskFieldValues,
    },
};

#[derive(PartialEq, Clone)]
pub enum Pane {
    Preview,
    MoveTaskModal,
    AddTask,
    Column,
    Kanban(TaskStatus),
}

#[derive(Debug, PartialEq, Clone)]
pub struct KanbanFocus {
    pub column: TaskStatus,
    pub task_idx: Option<usize>,
}

pub struct AppState {
    pub tasks: HashMap<TaskStatus, Vec<Task>>,
    pub active_pane: Pane,
    pub kanban_focus: Option<KanbanFocus>,
    pub modal_focus: Option<TaskStatus>,
    pub add_task_focus: Option<AddTaskModalState>,
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
            self.add_task(t);
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
            _ => self.active_pane.clone(),
        }
    }

    pub fn is_focused_add_task(&self) -> bool {
        self.add_task_focus != None
    }

    pub fn focus_add_task_modal(&mut self) {
        if self.is_focused_add_task() {
            return;
        }

        self.add_task_focus = Some(AddTaskModalState {
            current_field: TaskField::Name,
            field_values: TaskFieldValues::default(),
        });
    }

    pub fn get_add_task_focused_field(&self) -> Option<TaskField> {
        let Some(state) = &self.add_task_focus else {
            return None;
        };

        Some(state.current_field.clone())
    }

    pub fn cycle_add_task_field(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.next_field();
        }
    }

    pub fn add_to_name(&mut self, c: char) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.field_values.add_to_name(c);
        }
    }

    pub fn insert_to_name(&mut self, idx: usize, c: char) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.field_values.insert_to_name(idx, c);
        }
    }

    pub fn pop_name(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.field_values.pop_name();
        }
    }

    pub fn remove_from_name(&mut self, idx: usize) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.field_values.remove_char_name(idx);
        }
    }

    pub fn add_to_description(&mut self, c: char) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.field_values.add_to_description(c);
        }
    }

    pub fn insert_to_description(&mut self, idx: usize, c: char) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.field_values.insert_to_description(idx, c);
        }
    }

    pub fn pop_description(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.field_values.pop_description();
        }
    }

    pub fn remove_from_description(&mut self, idx: usize) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.field_values.remove_char_description(idx);
        }
    }

    pub fn prev_status(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.field_values.prev_status();
        }
    }
    pub fn prev_priority(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.field_values.prev_priority();
        }
    }

    pub fn next_status(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.field_values.next_status();
        }
    }
    pub fn next_priority(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.field_values.next_priority();
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
        self.kanban_focus = None;
    }

    pub fn remove_move_task_focus(&mut self) {
        if !self.is_moving_task() {
            return;
        }
        self.modal_focus = None;
        self.active_pane = Pane::Kanban(TaskStatus::Pending);
    }

    pub fn remove_add_task_focus(&mut self) {
        if !self.is_focused_add_task() {
            return;
        }
        self.add_task_focus = None;
        self.active_pane = Pane::Kanban(TaskStatus::Pending);
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

    pub fn save_new_task(&mut self) {
        let Some(add_task_state) = &mut self.add_task_focus else {
            return;
        };

        let task = Task::from(add_task_state.field_values.clone());
        let task_model = TaskModel::from(task.clone());
        // TODO: Handle errors.
        let _ = self.db.add_task(task_model);
        self.add_task(task);
    }

    pub fn add_task(&mut self, task: Task) {
        if let Some(task_list) = self.tasks.get_mut(&task.status) {
            task_list.push(task);
        }
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
        let _ = self.db.update_task(TaskModel::from(task));
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

    // TODO: Handle error
    pub fn remove_selected_task(&mut self) -> Option<()> {
        let task = self.get_focused_task()?;
        let target_task_list = self.tasks.get_mut(&task.status)?;

        let _ = self.db.delete_task(task.id.to_string());

        target_task_list.retain(|t| t.id != task.id);

        Some(())
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
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Pending,
            TaskPriority::Low,
        ));

        assert_eq!(Some(1), app.get_task_size_by_status(&TaskStatus::Pending));
    }

    #[test]
    fn should_not_be_focused_kanban() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Pending,
            TaskPriority::Low,
        ));

        assert_eq!(None, app.kanban_focus);
    }

    #[test]
    fn should_not_be_focused_kanban2() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Pending,
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
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Pending,
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
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Pending,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::InProgress,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::InProgress,
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
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Pending,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::InProgress,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::InProgress,
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
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Pending,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::InProgress,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::InProgress,
            TaskPriority::Low,
        ));

        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Completed,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Completed,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Completed,
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
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Pending,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::InProgress,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::InProgress,
            TaskPriority::Low,
        ));

        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Completed,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Completed,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Completed,
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
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Pending,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::InProgress,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::InProgress,
            TaskPriority::Low,
        ));

        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Completed,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Completed,
            TaskPriority::Low,
        ));
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Completed,
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
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Pending,
            TaskPriority::Low,
        ));

        assert_eq!(Some(TaskStatus::Pending), app.get_status_by_pane());
    }

    #[test]
    fn pane_should_be_in_progress() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Pending,
            TaskPriority::Low,
        ));
        app.cycle_pane();

        assert_eq!(Some(TaskStatus::InProgress), app.get_status_by_pane());
    }

    #[test]
    fn pane_should_be_completed() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.add_task(Task::new_custom(
            String::from("task1"),
            String::from("heyhey"),
            TaskStatus::Pending,
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

        let expected_value = AddTaskModalState {
            current_field: TaskField::Description,
            field_values: TaskFieldValues::default(),
        };

        assert_eq!(Some(expected_value), app.add_task_focus);
    }

    #[test]
    fn name_field_is_ac() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.focus_add_task_modal();

        let mut default_field_values = TaskFieldValues::default();
        default_field_values.name = "ac".to_string();

        let expected_value = AddTaskModalState {
            current_field: TaskField::Name,
            field_values: default_field_values,
        };

        app.add_to_name('a');
        app.add_to_name('c');

        assert_eq!(Some(expected_value), app.add_task_focus);
    }

    #[test]
    fn name_field_is_abc() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.focus_add_task_modal();

        let mut default_field_values = TaskFieldValues::default();
        default_field_values.name = "abc".to_string();

        let expected_value = AddTaskModalState {
            current_field: TaskField::Name,
            field_values: default_field_values,
        };

        app.add_to_name('a');
        app.add_to_name('c');
        app.insert_to_name(1, 'b');

        assert_eq!(Some(expected_value), app.add_task_focus);
    }

    #[test]
    fn name_field_is_ab_from_pop() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.focus_add_task_modal();

        let mut default_field_values = TaskFieldValues::default();
        default_field_values.name = "ab".to_string();

        let expected_value = AddTaskModalState {
            current_field: TaskField::Name,
            field_values: default_field_values,
        };

        app.add_to_name('a');
        app.add_to_name('c');
        app.insert_to_name(1, 'b');
        app.pop_name();

        assert_eq!(Some(expected_value), app.add_task_focus);
    }

    #[test]
    fn name_field_is_ac_from_remove() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.focus_add_task_modal();

        let mut default_field_values = TaskFieldValues::default();
        default_field_values.name = "ac".to_string();

        let expected_value = AddTaskModalState {
            current_field: TaskField::Name,
            field_values: default_field_values,
        };

        app.add_to_name('a');
        app.add_to_name('c');
        app.insert_to_name(1, 'b');
        app.remove_from_name(1);

        assert_eq!(Some(expected_value), app.add_task_focus);
    }
}
