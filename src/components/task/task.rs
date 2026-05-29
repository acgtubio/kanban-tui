use std::str::FromStr;

use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use uuid::Uuid;

use crate::db::TaskModel;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

impl FromSql for TaskStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(items) => match items {
                b"PENDING" => Ok(TaskStatus::Pending),
                b"IN_PROGRESS" => Ok(TaskStatus::InProgress),
                b"COMPLETED" => Ok(TaskStatus::Completed),
                _ => Err(FromSqlError::InvalidType),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl TaskStatus {
    pub const ALL: [TaskStatus; 3] = [
        TaskStatus::Pending,
        TaskStatus::InProgress,
        TaskStatus::Completed,
    ];

    fn from_string(s: &str) -> Result<TaskStatus, ()> {
        match s {
            "PENDING" => Ok(TaskStatus::Pending),
            "IN_PROGRESS" => Ok(TaskStatus::InProgress),
            "COMPLETED" => Ok(TaskStatus::Completed),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl FromSql for TaskPriority {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(items) => match items {
                b"LOW" => Ok(TaskPriority::Low),
                b"NORMAL" => Ok(TaskPriority::Normal),
                b"HIGH" => Ok(TaskPriority::High),
                b"CRITICAL" => Ok(TaskPriority::Critical),
                _ => Err(FromSqlError::InvalidType),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl TaskStatus {
    pub fn to_readable_string(&self) -> String {
        let title = match self {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Completed => "Completed",
        };

        title.into()
    }

    pub fn to_string(&self) -> String {
        let title = match self {
            TaskStatus::Pending => "PENDING",
            TaskStatus::InProgress => "IN_PROGRESS",
            TaskStatus::Completed => "COMPLETED",
        };

        title.into()
    }
}

impl TaskPriority {
    pub fn short_str(&self) -> String {
        let text = match self {
            TaskPriority::Normal => "N",
            TaskPriority::Low => "L",
            TaskPriority::High => "H",
            TaskPriority::Critical => "!!",
        };

        text.into()
    }

    pub fn to_string(&self) -> String {
        let text = match self {
            TaskPriority::Normal => "NORMAL",
            TaskPriority::Low => "LOW",
            TaskPriority::High => "HIGH",
            TaskPriority::Critical => "CRITICAL",
        };

        text.into()
    }

    pub fn to_readable_string(&self) -> String {
        let text = match self {
            TaskPriority::Normal => "Normal",
            TaskPriority::Low => "Low",
            TaskPriority::High => "High",
            TaskPriority::Critical => "Critical",
        };

        text.into()
    }

    pub fn from_string(s: &str) -> Result<Self, ()> {
        match s {
            "NORMAL" => Ok(TaskPriority::Normal),
            "LOW" => Ok(TaskPriority::Low),
            "HIGH" => Ok(TaskPriority::High),
            "CRITICAL" => Ok(TaskPriority::Critical),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum TaskConvertError {
    Err(String),
}

impl From<uuid::Error> for TaskConvertError {
    fn from(value: uuid::Error) -> Self {
        TaskConvertError::Err(value.to_string())
    }
}

impl From<()> for TaskConvertError {
    fn from(value: ()) -> Self {
        TaskConvertError::Err("Error converting from string to enum".to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Task {
    pub fn new(name: String, description: String, prio: TaskPriority) -> Self {
        Task {
            id: Uuid::new_v4(),
            name: name,
            description: description,
            status: TaskStatus::Pending,
            priority: prio,
        }
    }

    pub fn from_task_model(task_model: &TaskModel) -> Result<Self, TaskConvertError> {
        Ok(Task {
            id: Uuid::from_str(task_model.id.as_str())?,
            name: task_model.name.clone(),
            description: task_model.description.clone(),
            status: TaskStatus::from_string(task_model.status.as_str())?,
            priority: TaskPriority::from_string(task_model.priority.as_str())?,
        })
    }

    pub fn get_priority(&self) -> TaskPriority {
        self.priority
    }

    pub fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    pub fn get_status(&self) -> TaskStatus {
        self.status
    }

    pub fn is_pending(&self) -> bool {
        self.status == TaskStatus::Pending
    }

    pub fn is_ongoing(&self) -> bool {
        self.status == TaskStatus::InProgress
    }

    pub fn is_done(&self) -> bool {
        self.status == TaskStatus::Completed
    }
}
