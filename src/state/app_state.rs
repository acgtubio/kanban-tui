use std::collections::HashMap;

use crate::{
    components::{Task, TaskConvertError, TaskStatus},
    db::{Db, ProjectModel, SqliteDb, TaskModel},
    state::{
        add_task_state::AddTaskModalState, task_field::TaskField, task_field_value::TaskFieldValues,
    },
};

#[derive(PartialEq, Clone)]
pub enum Pane {
    ProjectList,
    Preview,
    MoveTaskModal,
    DeleteConfirmModal,
    AddTask,
    Column,
    Kanban(TaskStatus),
}

#[derive(Debug, PartialEq, Clone)]
pub struct KanbanFocus {
    pub column: TaskStatus,
    pub task_idx: Option<isize>,
}

pub struct AppState {
    pub projects: Vec<ProjectModel>,
    pub project_focus: usize,
    /// The project whose board is currently open; `tasks` holds only this project's tasks.
    pub current_project: Option<ProjectModel>,
    pub tasks: HashMap<TaskStatus, Vec<Task>>,
    pub active_pane: Pane,
    pub kanban_focus: Option<KanbanFocus>,
    pub modal_focus: Option<TaskStatus>,
    pub add_task_focus: Option<AddTaskModalState>,
    pub delete_confirm_focus: Option<bool>,
    db: SqliteDb,
}

impl AppState {
    pub fn new(db: SqliteDb) -> Self {
        let mut app_state = AppState {
            projects: vec![],
            project_focus: 0,
            current_project: None,
            tasks: HashMap::new(),
            active_pane: Pane::ProjectList,
            kanban_focus: None,
            modal_focus: None,
            add_task_focus: None,
            delete_confirm_focus: None,
            db: db,
        };

        app_state.tasks.insert(TaskStatus::Backlog, vec![]);
        app_state.tasks.insert(TaskStatus::Pending, vec![]);
        app_state.tasks.insert(TaskStatus::InProgress, vec![]);
        app_state.tasks.insert(TaskStatus::Completed, vec![]);

        app_state
    }

    pub fn init_projects(&mut self) {
        self.projects = self.db.get_projects().expect("Unable to fetch projects.");
        self.project_focus = 0;
    }

    pub fn update_project_selection(&mut self, increment: isize) {
        let len = self.projects.len() as isize;
        if len == 0 {
            return;
        }

        self.project_focus = (self.project_focus as isize + increment).rem_euclid(len) as usize;
    }

    pub fn open_selected_project(&mut self) {
        let Some(project) = self.projects.get(self.project_focus).cloned() else {
            return;
        };

        self.clear_tasks();
        self.load_tasks(&project.id);
        self.current_project = Some(project);
        self.kanban_focus = None;
        self.active_pane = Pane::Kanban(TaskStatus::Pending);
    }

    pub fn close_project(&mut self) {
        self.clear_tasks();
        self.current_project = None;
        self.kanban_focus = None;
        self.active_pane = Pane::ProjectList;
    }

    fn clear_tasks(&mut self) {
        self.tasks.values_mut().for_each(|tasks| tasks.clear());
    }

    fn load_tasks(&mut self, project_id: &str) {
        let tasks_raw = self
            .db
            .get_tasks(project_id)
            .expect("Unable to fetch kanban data.");

        let tasks = tasks_raw
            .iter()
            .map(|task| Task::from_task_model(task))
            .collect::<Result<Vec<Task>, TaskConvertError>>()
            .expect("Unable to convert to service models.");

        tasks.into_iter().for_each(|task| self.add_task(task));
    }

    pub fn cycle_focus(&mut self) {
        match self.kanban_focus {
            Some(_) => self.cycle_kanban_focus(1),
            None => self.cycle_pane(),
        };
    }

    pub fn is_focused_kanban(&self) -> bool {
        self.kanban_focus != None
    }

    pub fn is_moving_task(&self) -> bool {
        self.active_pane == Pane::MoveTaskModal
    }

    pub fn is_confirming_delete(&self) -> bool {
        self.active_pane == Pane::DeleteConfirmModal
    }

    pub fn open_delete_confirm_modal(&mut self) {
        if self.get_focused_task().is_none() {
            return;
        }
        self.active_pane = Pane::DeleteConfirmModal;
        self.delete_confirm_focus = Some(false);
    }

    pub fn close_delete_confirm_modal(&mut self) {
        self.delete_confirm_focus = None;
        self.active_pane = Pane::Column;
    }

    pub fn toggle_delete_confirm_focus(&mut self) {
        if let Some(focus) = self.delete_confirm_focus {
            self.delete_confirm_focus = Some(!focus);
        }
    }

    pub fn confirm_delete_focused_task(&mut self) {
        if self.delete_confirm_focus == Some(true) {
            self.archive_selected_task();
        }
        self.close_delete_confirm_modal();
    }

    pub fn update_kanban_selection(&mut self, increment: isize) {
        self.cycle_kanban_focus(increment);
    }

    fn cycle_kanban_focus(&mut self, increment: isize) {
        let (status, index) = {
            let Some(focus) = &mut self.kanban_focus else {
                return;
            };

            let status = focus.column;

            let Some(index) = focus.task_idx else {
                focus.task_idx = Some(0);
                return;
            };

            (status, index)
        };

        let Some(max_len) = self.get_task_size_by_status(&status) else {
            return;
        };

        let Some(focus) = &mut self.kanban_focus else {
            return;
        };

        if index < max_len.clone() as isize - 1 && index + increment >= 0 {
            focus.task_idx = Some(index as isize + increment);
        } else {
            focus.task_idx = Some(0);
        }
    }

    pub fn cycle_task_status_focus(&mut self) {
        if let Some(modal_focus) = self.modal_focus {
            self.modal_focus = Some(modal_focus.next());
        }
    }

    pub fn prev_task_status_focus(&mut self) {
        if let Some(modal_focus) = self.modal_focus {
            self.modal_focus = Some(modal_focus.prev());
        }
    }

    pub fn cycle_pane(&mut self) {
        self.active_pane = match self.active_pane {
            Pane::Preview => Pane::Kanban(TaskStatus::Pending),
            Pane::Kanban(task_status) => Pane::Kanban(task_status.next()),
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

        self.active_pane = Pane::AddTask;
        self.add_task_focus = Some(AddTaskModalState {
            current_field: TaskField::Name,
            field_values: TaskFieldValues::default(),
            editing_task_id: None,
            name_cursor: 0,
            description_cursor: 0,
        });
    }

    pub fn focus_edit_task_modal(&mut self) {
        if self.is_focused_add_task() {
            return;
        }

        let Some(task) = self.get_focused_task() else {
            return;
        };

        self.active_pane = Pane::AddTask;
        let name_cursor = task.name.chars().count();
        let description_cursor = task.description.chars().count();
        self.add_task_focus = Some(AddTaskModalState {
            current_field: TaskField::Name,
            field_values: TaskFieldValues::from_task(&task),
            editing_task_id: Some(task.id),
            name_cursor,
            description_cursor,
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

    pub fn insert_char_at_name_cursor(&mut self, c: char) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            let idx = add_task_focus.name_cursor;
            add_task_focus.field_values.insert_to_name(idx, c);
            add_task_focus.name_cursor += 1;
        }
    }

    pub fn backspace_at_name_cursor(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            if add_task_focus.name_cursor > 0 {
                add_task_focus.name_cursor -= 1;
                let idx = add_task_focus.name_cursor;
                add_task_focus.field_values.remove_char_name(idx);
            }
        }
    }

    pub fn move_name_cursor_left(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.name_cursor = add_task_focus.name_cursor.saturating_sub(1);
        }
    }

    pub fn move_name_cursor_right(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            let max = add_task_focus.field_values.name.chars().count();
            add_task_focus.name_cursor = (add_task_focus.name_cursor + 1).min(max);
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

    pub fn insert_char_at_description_cursor(&mut self, c: char) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            let idx = add_task_focus.description_cursor;
            add_task_focus.field_values.insert_to_description(idx, c);
            add_task_focus.description_cursor += 1;
        }
    }

    pub fn backspace_at_description_cursor(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            if add_task_focus.description_cursor > 0 {
                add_task_focus.description_cursor -= 1;
                let idx = add_task_focus.description_cursor;
                add_task_focus.field_values.remove_char_description(idx);
            }
        }
    }

    pub fn move_description_cursor_left(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            add_task_focus.description_cursor = add_task_focus.description_cursor.saturating_sub(1);
        }
    }

    pub fn move_description_cursor_right(&mut self) {
        if let Some(add_task_focus) = &mut self.add_task_focus {
            let max = add_task_focus.field_values.description.chars().count();
            add_task_focus.description_cursor = (add_task_focus.description_cursor + 1).min(max);
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

        self.active_pane = Pane::Column;
        self.kanban_focus = Some(KanbanFocus {
            column: status,
            task_idx: Some(0),
        });
    }

    pub fn remove_kanban_focus(&mut self) {
        self.kanban_focus = None;
        self.active_pane = Pane::Kanban(TaskStatus::Pending);
    }

    pub fn remove_move_task_focus(&mut self) {
        if !self.is_moving_task() {
            return;
        }
        self.modal_focus = None;
        self.active_pane = Pane::Column;
    }

    pub fn remove_add_task_focus(&mut self) {
        if !self.is_focused_add_task() {
            return;
        }

        let was_editing = self
            .add_task_focus
            .as_ref()
            .is_some_and(|s| s.editing_task_id.is_some());

        self.add_task_focus = None;
        self.active_pane = if was_editing {
            Pane::Column
        } else {
            Pane::Kanban(TaskStatus::Pending)
        };
    }

    fn get_task_size_by_status(&self, status: &TaskStatus) -> Option<usize> {
        self.tasks.get(&status).map(|t| t.len())
    }

    fn get_status_by_pane(&self) -> Option<TaskStatus> {
        match self.active_pane {
            Pane::Kanban(task_status) => Some(task_status),
            _ => None,
        }
    }

    pub fn save_task_form(&mut self) {
        let Some(form_state) = &self.add_task_focus else {
            return;
        };
        let editing_task_id = form_state.editing_task_id;
        let field_values = form_state.field_values.clone();

        match editing_task_id {
            Some(_) => self.save_edited_task(field_values),
            None => self.save_created_task(field_values),
        }
    }

    fn save_created_task(&mut self, field_values: TaskFieldValues) {
        let Some(project) = &self.current_project else {
            return;
        };
        let task = Task::from(field_values);
        let task_model = TaskModel::from(task.clone());
        // TODO: Handle errors.
        let _ = self.db.add_task(task_model, &project.id);
        self.add_task(task);
    }

    fn save_edited_task(&mut self, field_values: TaskFieldValues) {
        let Some(existing) = self.get_focused_task() else {
            return;
        };

        if field_values.task_status != existing.status {
            let source_task = Task {
                id: existing.id,
                name: field_values.name,
                description: field_values.description,
                status: existing.status,
                priority: field_values.task_priority,
                archived: existing.archived,
            };
            self.move_task(source_task, field_values.task_status);
        } else {
            let updated = Task {
                id: existing.id,
                name: field_values.name,
                description: field_values.description,
                status: existing.status,
                priority: field_values.task_priority,
                archived: existing.archived,
            };
            if let Some(list) = self.tasks.get_mut(&existing.status)
                && let Some(t) = list.iter_mut().find(|t| t.id == updated.id)
            {
                *t = updated.clone();
            }
            self.update_task_on_db(updated);
        }
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
            archived: task.archived,
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

        let task = tasks.get(selected_index as usize)?;

        Some(task.clone())
    }

    // TODO: Handle error
    pub fn archive_selected_task(&mut self) -> Option<()> {
        let task = self.get_focused_task()?;
        let target_task_list = self.tasks.get_mut(&task.status)?;

        let _ = self.db.archive_task(task.id.to_string());

        target_task_list.retain(|t| t.id != task.id);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use crate::components::TaskPriority;

    use super::*;

    /// AppState starts on the project list; most tests exercise the board directly.
    fn board_state(db: SqliteDb) -> AppState {
        let mut app = AppState::new(db);
        app.active_pane = Pane::Kanban(TaskStatus::Pending);
        app
    }

    #[test]
    fn pending_task_size_should_1() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = board_state(db);
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

        let mut app = board_state(db);
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

        let mut app = board_state(db);
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

        let mut app = board_state(db);
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

        let mut app = board_state(db);
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
        app.cycle_kanban_focus(1);

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

        let mut app = board_state(db);
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
        app.cycle_kanban_focus(1);
        app.cycle_kanban_focus(1);

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

        let mut app = board_state(db);
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

        app.cycle_kanban_focus(1);
        app.cycle_kanban_focus(1);

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

        let mut app = board_state(db);
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

        app.cycle_kanban_focus(1);

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

        let mut app = board_state(db);
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

        app.cycle_kanban_focus(1);
        app.cycle_kanban_focus(1);
        app.cycle_kanban_focus(1);

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

        let mut app = board_state(db);
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

        let mut app = board_state(db);
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

        let mut app = board_state(db);
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

        let mut app = board_state(db);
        app.focus_add_task_modal();
        app.cycle_add_task_field();

        let expected_value = AddTaskModalState {
            current_field: TaskField::Description,
            field_values: TaskFieldValues::default(),
            editing_task_id: None,
            name_cursor: 0,
            description_cursor: 0,
        };

        assert_eq!(Some(expected_value), app.add_task_focus);
    }

    #[test]
    fn name_field_is_ac() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = board_state(db);
        app.focus_add_task_modal();

        let mut default_field_values = TaskFieldValues::default();
        default_field_values.name = "ac".to_string();

        let expected_value = AddTaskModalState {
            current_field: TaskField::Name,
            field_values: default_field_values,
            editing_task_id: None,
            name_cursor: 0,
            description_cursor: 0,
        };

        app.add_to_name('a');
        app.add_to_name('c');

        assert_eq!(Some(expected_value), app.add_task_focus);
    }

    #[test]
    fn name_field_is_abc() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = board_state(db);
        app.focus_add_task_modal();

        let mut default_field_values = TaskFieldValues::default();
        default_field_values.name = "abc".to_string();

        let expected_value = AddTaskModalState {
            current_field: TaskField::Name,
            field_values: default_field_values,
            editing_task_id: None,
            name_cursor: 0,
            description_cursor: 0,
        };

        app.add_to_name('a');
        app.add_to_name('c');
        app.insert_to_name(1, 'b');

        assert_eq!(Some(expected_value), app.add_task_focus);
    }

    #[test]
    fn name_field_is_ab_from_pop() {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");

        let mut app = board_state(db);
        app.focus_add_task_modal();

        let mut default_field_values = TaskFieldValues::default();
        default_field_values.name = "ab".to_string();

        let expected_value = AddTaskModalState {
            current_field: TaskField::Name,
            field_values: default_field_values,
            editing_task_id: None,
            name_cursor: 0,
            description_cursor: 0,
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

        let mut app = board_state(db);
        app.focus_add_task_modal();

        let mut default_field_values = TaskFieldValues::default();
        default_field_values.name = "ac".to_string();

        let expected_value = AddTaskModalState {
            current_field: TaskField::Name,
            field_values: default_field_values,
            editing_task_id: None,
            name_cursor: 0,
            description_cursor: 0,
        };

        app.add_to_name('a');
        app.add_to_name('c');
        app.insert_to_name(1, 'b');
        app.remove_from_name(1);

        assert_eq!(Some(expected_value), app.add_task_focus);
    }

    fn project_state() -> AppState {
        let db = SqliteDb::new_in_memory().expect("Should not throw error");
        db.init_db().expect("Should not throw error");

        let mut app = AppState::new(db);
        app.init_projects();
        app
    }

    #[test]
    fn should_start_on_project_list_with_default_project() {
        let app = project_state();

        assert!(app.active_pane == Pane::ProjectList);
        assert_eq!(1, app.projects.len());
        assert_eq!("Default", app.projects[0].name);
        assert_eq!(None, app.current_project);
    }

    #[test]
    fn should_open_selected_project() {
        let mut app = project_state();

        app.open_selected_project();

        assert!(app.active_pane == Pane::Kanban(TaskStatus::Pending));
        assert_eq!(Some("Default"), app.current_project.as_ref().map(|p| p.name.as_str()));
    }

    #[test]
    fn should_load_only_open_projects_tasks_and_persist_created_tasks() {
        let mut app = project_state();
        app.open_selected_project();
        app.focus_add_task_modal();
        app.insert_to_name(0, 'a');
        app.save_task_form();
        assert_eq!(Some(1), app.get_task_size_by_status(&TaskStatus::Pending));

        app.close_project();
        assert_eq!(Some(0), app.get_task_size_by_status(&TaskStatus::Pending));

        app.open_selected_project();
        assert_eq!(Some(1), app.get_task_size_by_status(&TaskStatus::Pending));
    }

    #[test]
    fn cycle_pane_should_visit_backlog_and_wrap() {
        let mut app = project_state();
        app.open_selected_project();

        let mut visited = vec![app.get_status_by_pane().unwrap()];
        for _ in 0..4 {
            app.cycle_pane();
            visited.push(app.get_status_by_pane().unwrap());
        }

        assert_eq!(
            vec![
                TaskStatus::Pending,
                TaskStatus::InProgress,
                TaskStatus::Completed,
                TaskStatus::Backlog,
                TaskStatus::Pending,
            ],
            visited
        );
    }

    #[test]
    fn should_persist_backlog_task_and_reload_into_backlog_column() {
        let mut app = project_state();
        app.open_selected_project();
        app.focus_add_task_modal();
        app.insert_to_name(0, 'a');
        // Default status is Pending, so one step back lands on Backlog.
        app.add_task_focus.as_mut().unwrap().field_values.prev_status();
        assert_eq!(
            TaskStatus::Backlog,
            app.add_task_focus.as_ref().unwrap().field_values.task_status
        );
        app.save_task_form();
        assert_eq!(Some(1), app.get_task_size_by_status(&TaskStatus::Backlog));

        app.close_project();
        app.open_selected_project();

        assert_eq!(Some(1), app.get_task_size_by_status(&TaskStatus::Backlog));
        assert_eq!(Some(0), app.get_task_size_by_status(&TaskStatus::Pending));
    }

    #[test]
    fn should_return_to_project_list_on_close_project() {
        let mut app = project_state();
        app.open_selected_project();

        app.close_project();

        assert!(app.active_pane == Pane::ProjectList);
        assert_eq!(None, app.current_project);
        assert_eq!(None, app.kanban_focus);
    }

    #[test]
    fn project_selection_should_wrap() {
        let mut app = project_state();

        app.update_project_selection(1);
        assert_eq!(0, app.project_focus);

        app.update_project_selection(-1);
        assert_eq!(0, app.project_focus);
    }
}
