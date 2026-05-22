use rusqlite::{Connection, Error, Result};

use crate::{
    components::{Task, TaskPriority},
    db::TaskModel,
};

pub trait Db {
    fn get_tasks(&self) -> Result<Vec<TaskModel>, Error>;
    fn add_task(&self, task: TaskModel) -> Result<usize, rusqlite::Error>;
    fn update_task(&self);
}

pub struct SqliteDb {
    conn: Connection,
}

impl SqliteDb {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("kanban.db")?;

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

    pub fn test_init(&self) -> Result<usize, rusqlite::Error> {
        let task = TaskModel::from_task(Task::new(
            String::from("task1"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        let task2 = TaskModel::from_task(Task::new(
            String::from("task2"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        let task3 = TaskModel::from_task(Task::new(
            String::from("task3"),
            String::from("heyhey"),
            TaskPriority::Low,
        ));
        let task4 = TaskModel::from_task(Task::new(
            String::from("task4"),
            String::from("heyhey"),
            TaskPriority::High,
        ));

        let res = self.conn.execute("INSERT INTO tasks(uuid, name, description, status, priority) VALUES(?1, ?2, ?3, ?4 ,?5)", (
                task.id,
                task.name,
                task.description,
                task.status,
                task.priority
                ))?;

        let res = self.conn.execute("INSERT INTO tasks(uuid, name, description, status, priority) VALUES(?1, ?2, ?3, ?4 ,?5)", (
                task2.id,
                task2.name,
                task2.description,
                task2.status,
                task2.priority
                ))?;

        let res = self.conn.execute("INSERT INTO tasks(uuid, name, description, status, priority) VALUES(?1, ?2, ?3, ?4 ,?5)", (
                task3.id,
                task3.name,
                task3.description,
                task3.status,
                task3.priority
                ))?;

        let res = self.conn.execute("INSERT INTO tasks(uuid, name, description, status, priority) VALUES(?1, ?2, ?3, ?4 ,?5)", (
                task4.id,
                task4.name,
                task4.description,
                task4.status,
                task4.priority
                ))?;

        Ok(res)
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

    fn update_task(&self) {
        todo!()
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
