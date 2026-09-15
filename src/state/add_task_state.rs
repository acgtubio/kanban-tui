use uuid::Uuid;

use crate::state::{task_field::TaskField, task_field_value::TaskFieldValues};

#[derive(Debug, PartialEq, Clone)]
pub struct AddTaskModalState {
    pub current_field: TaskField,
    pub field_values: TaskFieldValues,
    pub editing_task_id: Option<Uuid>,
    pub name_cursor: usize,
    pub description_cursor: usize,
}

impl AddTaskModalState {
    pub fn next_field(&mut self) {
        self.current_field = match self.current_field {
            TaskField::Name => TaskField::Description,
            TaskField::Description => TaskField::TaskStatus,
            TaskField::TaskStatus => TaskField::TaskPriority,
            TaskField::TaskPriority => TaskField::Name,
        };
    }

    pub fn prev_field(&mut self) {
        self.current_field = match self.current_field {
            TaskField::Name => TaskField::TaskPriority,
            TaskField::Description => TaskField::Name,
            TaskField::TaskStatus => TaskField::Description,
            TaskField::TaskPriority => TaskField::TaskStatus,
        };
    }
}
