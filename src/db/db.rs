use rusqlite::{Connection, Error, Result};

use crate::{
    components::{Task, TaskPriority},
    db::{TaskModel, TaskUpdateModel},
};

pub trait Db {
    fn get_tasks(&self) -> Result<Vec<TaskModel>, Error>;
    fn add_task(&self, task: TaskModel) -> Result<usize, rusqlite::Error>;
    fn update_task(&self, task: TaskModel) -> Result<usize, rusqlite::Error>;
}

pub struct SqliteDb {
    conn: Connection,
}

impl SqliteDb {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("kanban.db")?;

        Ok(SqliteDb { conn })
    }

    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;

        Ok(SqliteDb { conn })
    }

    pub fn init_db(&self) -> Result<usize, Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                uuid TEXT PRIMARY KEY,
                name TEXT,
                description TEXT,
                status TEXT,
                priority TEXT
            );",
            (),
        )
    }
}

impl Db for SqliteDb {
    fn add_task(&self, task: TaskModel) -> Result<usize, rusqlite::Error> {
        let res = self.conn.execute("INSERT INTO tasks(uuid, name, description, status, priority) VALUES(?1, ?2, ?3, ?4 ,?5)", (
                task.id,
                task.name,
                task.description,
                task.status,
                task.priority
                ))?;

        Ok(res)
    }

    fn update_task(&self, task: TaskModel) -> Result<usize, rusqlite::Error> {
        let res = self.conn.execute("UPDATE tasks SET name = ?1, description = ?2, status = ?3, priority = ?4 WHERE uuid = ?5", (
                task.name,
                task.description,
                task.status,
                task.priority,
                task.id
                ))?;

        Ok(res)
    }

    fn get_tasks(&self) -> Result<Vec<TaskModel>, Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM tasks")?;
        let tasks_iter = stmt.query_map([], |row| {
            Ok(TaskModel {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
            })
        })?;

        let mut tasks = vec![];
        for task in tasks_iter {
            tasks.push(task.unwrap());
        }

        Ok(tasks)
    }
}
