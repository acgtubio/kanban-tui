use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

impl TaskStatus {
    pub fn to_string(&self) -> String {
        let title = match self {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Completed => "Completed",
        };

        title.into()
    }
}

pub struct Task {
    id: Uuid,
    pub name: String,
    description: String,
    status: TaskStatus,
}

impl Task {
    pub fn new(name: String, description: String) -> Self {
        Task {
            id: Uuid::new_v4(),
            name: name,
            description: description,
            status: TaskStatus::Pending,
        }
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
