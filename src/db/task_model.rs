use crate::components::Task;

pub struct TaskModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub priority: String,
}

impl TaskModel {
    pub fn from_task(task: Task) -> Self {
        TaskModel {
            id: task.id.to_string(),
            name: task.name,
            description: task.description,
            status: task.status.to_string(),
            priority: task.priority.to_string(),
        }
    }
}

pub struct TaskUpdateModel {
    pub name: String,
    pub description: String,
    pub status: String,
    pub priority: String,
}

impl TaskUpdateModel {
    pub fn from_task(task: Task) -> Self {
        TaskUpdateModel {
            name: task.name,
            description: task.description,
            status: task.status.to_string(),
            priority: task.priority.to_string(),
        }
    }
}
