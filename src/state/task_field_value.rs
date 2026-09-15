use crate::components::{Task, TaskPriority, TaskStatus};

#[derive(Debug, PartialEq, Clone)]
pub struct TaskFieldValues {
    pub name: String,
    pub description: String,
    pub task_status: TaskStatus,
    pub task_priority: TaskPriority,
}

impl TaskFieldValues {
    pub fn from_task(task: &Task) -> Self {
        Self {
            name: task.name.clone(),
            description: task.description.clone(),
            task_status: task.status,
            task_priority: task.priority,
        }
    }

    pub fn add_to_name(&mut self, c: char) {
        self.name.push(c);
    }

    pub fn insert_to_name(&mut self, idx: usize, c: char) {
        let byte_idx = Self::char_idx_to_byte_idx(&self.name, idx);
        self.name.insert(byte_idx, c);
    }

    pub fn pop_name(&mut self) {
        self.name.pop();
    }

    pub fn remove_char_name(&mut self, idx: usize) {
        let byte_idx = Self::char_idx_to_byte_idx(&self.name, idx);
        if byte_idx < self.name.len() {
            self.name.remove(byte_idx);
        }
    }

    pub fn add_to_description(&mut self, c: char) {
        self.description.push(c);
    }

    pub fn insert_to_description(&mut self, idx: usize, c: char) {
        let byte_idx = Self::char_idx_to_byte_idx(&self.description, idx);
        self.description.insert(byte_idx, c);
    }

    pub fn pop_description(&mut self) {
        self.description.pop();
    }

    pub fn remove_char_description(&mut self, idx: usize) {
        let byte_idx = Self::char_idx_to_byte_idx(&self.description, idx);
        if byte_idx < self.description.len() {
            self.description.remove(byte_idx);
        }
    }

    fn char_idx_to_byte_idx(s: &str, idx: usize) -> usize {
        s.char_indices().nth(idx).map(|(b, _)| b).unwrap_or(s.len())
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
