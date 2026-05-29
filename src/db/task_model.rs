use crate::components::Task;

pub struct TaskModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub priority: String,
}

impl From<Task> for TaskModel {
    fn from(value: Task) -> Self {
        TaskModel {
            id: value.id.to_string(),
            name: value.name,
            description: value.description,
            status: value.status.to_string(),
            priority: value.priority.to_string(),
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
