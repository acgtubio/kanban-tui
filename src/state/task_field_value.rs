use crate::components::{TaskPriority, TaskStatus};

#[derive(Debug, PartialEq, Clone)]
pub struct TaskFieldValues {
    pub name: String,
    pub description: String,
    pub task_status: TaskStatus,
    pub task_priority: TaskPriority,
}

impl TaskFieldValues {
    pub fn add_to_name(&mut self, c: char) {
        self.name.push(c);
    }

    pub fn insert_to_name(&mut self, idx: usize, c: char) {
        self.name.insert(idx, c);
    }

    pub fn pop_name(&mut self) {
        self.name.pop();
    }

    pub fn remove_char_name(&mut self, idx: usize) {
        self.name.remove(idx);
    }

    pub fn add_to_description(&mut self, c: char) {
        self.description.push(c);
    }

    pub fn insert_to_description(&mut self, idx: usize, c: char) {
        self.description.insert(idx, c);
    }

    pub fn pop_description(&mut self) {
        self.description.pop();
    }

    pub fn remove_char_description(&mut self, idx: usize) {
        self.description.remove(idx);
    }

    pub fn next_status(&mut self) {
        self.task_status = match self.task_status {
            TaskStatus::Pending => TaskStatus::InProgress,
            TaskStatus::InProgress => TaskStatus::Completed,
            TaskStatus::Completed => TaskStatus::Pending,
        }
    }

    pub fn prev_status(&mut self) {
        self.task_status = match self.task_status {
            TaskStatus::Pending => TaskStatus::Completed,
            TaskStatus::InProgress => TaskStatus::Pending,
            TaskStatus::Completed => TaskStatus::InProgress,
        };
    }

    pub fn next_priority(&mut self) {
        self.task_priority = match self.task_priority {
            TaskPriority::Low => TaskPriority::Normal,
            TaskPriority::Normal => TaskPriority::High,
            TaskPriority::High => TaskPriority::Critical,
            TaskPriority::Critical => TaskPriority::Low,
        };
    }

    pub fn prev_priority(&mut self) {
        self.task_priority = match self.task_priority {
            TaskPriority::Low => TaskPriority::Critical,
            TaskPriority::Normal => TaskPriority::Low,
            TaskPriority::High => TaskPriority::Normal,
            TaskPriority::Critical => TaskPriority::High,
        };
    }
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
