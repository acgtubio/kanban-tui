use rusqlite::{Connection, Error, Result};

use crate::db::{ProjectModel, TaskModel};

pub const DEFAULT_PROJECT_ID: &str = "default";
const DEFAULT_PROJECT_NAME: &str = "Default";

pub trait Db {
    fn get_projects(&self) -> Result<Vec<ProjectModel>, Error>;
    fn add_task(&self, task: TaskModel, project_id: &str) -> Result<usize, rusqlite::Error>;
    fn get_tasks(&self, project_id: &str) -> Result<Vec<TaskModel>, Error>;
    fn update_task(&self, task: TaskModel) -> Result<usize, rusqlite::Error>;
    fn archive_task(&self, uuid: String) -> Result<usize, rusqlite::Error>;
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
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL
            );",
            (),
        )?;

        self.conn.execute(
            "INSERT OR IGNORE INTO projects(id, name) VALUES(?1, ?2)",
            (DEFAULT_PROJECT_ID, DEFAULT_PROJECT_NAME),
        )?;

        let res = self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                uuid TEXT PRIMARY KEY,
                name TEXT,
                description TEXT,
                status TEXT,
                priority TEXT,
                archived INTEGER NOT NULL DEFAULT 0,
                project_id TEXT NOT NULL DEFAULT 'default'
            );",
            (),
        )?;

        self.migrate_archived_column()?;
        self.migrate_project_column()?;

        Ok(res)
    }

    fn migrate_project_column(&self) -> Result<(), Error> {
        let mut stmt = self.conn.prepare("PRAGMA table_info(tasks)")?;
        let has_project_id = stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .filter_map(Result::ok)
            .any(|name| name == "project_id");

        if !has_project_id {
            // Existing rows are backfilled with the default project via the column default.
            self.conn.execute(
                "ALTER TABLE tasks ADD COLUMN project_id TEXT NOT NULL DEFAULT 'default'",
                (),
            )?;
        }

        Ok(())
    }

    fn migrate_archived_column(&self) -> Result<(), Error> {
        let mut stmt = self.conn.prepare("PRAGMA table_info(tasks)")?;
        let has_archived = stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .filter_map(Result::ok)
            .any(|name| name == "archived");

        if !has_archived {
            self.conn.execute(
                "ALTER TABLE tasks ADD COLUMN archived INTEGER NOT NULL DEFAULT 0",
                (),
            )?;
        }

        Ok(())
    }
}

impl Db for SqliteDb {
    fn get_projects(&self) -> Result<Vec<ProjectModel>, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM projects ORDER BY rowid")?;
        let projects = stmt
            .query_map([], |row| {
                Ok(ProjectModel {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(projects)
    }

    fn add_task(&self, task: TaskModel, project_id: &str) -> Result<usize, rusqlite::Error> {
        let res = self.conn.execute("INSERT INTO tasks(uuid, name, description, status, priority, project_id) VALUES(?1, ?2, ?3, ?4, ?5, ?6)", (
                task.id,
                task.name,
                task.description,
                task.status,
                task.priority,
                project_id,
                ))?;

        Ok(res)
    }

    fn get_tasks(&self, project_id: &str) -> Result<Vec<TaskModel>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT uuid, name, description, status, priority, archived
             FROM tasks WHERE archived = 0 AND project_id = ?1",
        )?;
        let tasks_iter = stmt.query_map([project_id], |row| {
            Ok(TaskModel {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
                archived: row.get(5)?,
            })
        })?;

        let mut tasks = vec![];
        for task in tasks_iter {
            tasks.push(task.unwrap());
        }

        Ok(tasks)
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

    fn archive_task(&self, uuid: String) -> Result<usize, Error> {
        let res = self
            .conn
            .execute("UPDATE tasks SET archived = 1 WHERE uuid = ?1", (uuid,))?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task_model(id: &str) -> TaskModel {
        TaskModel {
            id: id.to_string(),
            name: "name".to_string(),
            description: "desc".to_string(),
            status: "PENDING".to_string(),
            priority: "LOW".to_string(),
            archived: false,
        }
    }

    #[test]
    fn init_db_should_seed_default_project() {
        let db = SqliteDb::new_in_memory().unwrap();
        db.init_db().unwrap();

        let projects = db.get_projects().unwrap();

        assert_eq!(
            projects,
            vec![ProjectModel {
                id: DEFAULT_PROJECT_ID.to_string(),
                name: "Default".to_string()
            }]
        );
    }

    #[test]
    fn init_db_should_be_idempotent() {
        let db = SqliteDb::new_in_memory().unwrap();
        db.init_db().unwrap();
        db.init_db().unwrap();

        assert_eq!(db.get_projects().unwrap().len(), 1);
    }

    #[test]
    fn init_db_should_migrate_tasks_without_project_id_to_default_project() {
        let db = SqliteDb::new_in_memory().unwrap();
        db.conn
            .execute(
                "CREATE TABLE tasks (
                    uuid TEXT PRIMARY KEY,
                    name TEXT,
                    description TEXT,
                    status TEXT,
                    priority TEXT,
                    archived INTEGER NOT NULL DEFAULT 0
                );",
                (),
            )
            .unwrap();
        db.conn
            .execute(
                "INSERT INTO tasks(uuid, name, description, status, priority) VALUES('old', 'n', 'd', 'PENDING', 'LOW')",
                (),
            )
            .unwrap();

        db.init_db().unwrap();

        let tasks = db.get_tasks(DEFAULT_PROJECT_ID).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, "old");
    }

    #[test]
    fn get_tasks_should_be_scoped_by_project() {
        let db = SqliteDb::new_in_memory().unwrap();
        db.init_db().unwrap();
        db.conn
            .execute(
                "INSERT INTO projects(id, name) VALUES('other', 'Other')",
                (),
            )
            .unwrap();

        db.add_task(task_model("a"), DEFAULT_PROJECT_ID).unwrap();
        db.add_task(task_model("b"), "other").unwrap();

        let default_tasks = db.get_tasks(DEFAULT_PROJECT_ID).unwrap();
        let other_tasks = db.get_tasks("other").unwrap();

        assert_eq!(default_tasks.len(), 1);
        assert_eq!(default_tasks[0].id, "a");
        assert_eq!(other_tasks.len(), 1);
        assert_eq!(other_tasks[0].id, "b");
    }
}
