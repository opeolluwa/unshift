use std::path::Path;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::errors::AppError;

#[derive(Clone)]
pub struct Db {
    conn: Arc<Mutex<Connection>>,
}

impl Db {
    pub fn open(data_dir: &Path) -> Result<Self, AppError> {
        if !data_dir.is_dir() {
            std::fs::create_dir_all(data_dir)
                .map_err(|err| AppError::DbError(format!("failed to create data dir: {err}")))?;
        }

        let db_path = data_dir.join("unshift.db");
        let conn = Connection::open(&db_path)
            .map_err(|err| AppError::DbError(format!("failed to open database: {err}")))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS saved_messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                label TEXT,
                topic TEXT NOT NULL,
                key TEXT NOT NULL DEFAULT '',
                payload TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );",
        )
        .map_err(|err| AppError::DbError(format!("failed to run migrations: {err}")))?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn list(&self) -> Result<Vec<SavedMessage>, AppError> {
        let conn = self.conn.lock().map_err(|err| AppError::DbError(err.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT id, label, topic, key, payload, created_at FROM saved_messages ORDER BY created_at DESC")
            .map_err(|err| AppError::DbError(err.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                Ok(SavedMessage {
                    id: row.get(0)?,
                    label: row.get(1)?,
                    topic: row.get(2)?,
                    key: row.get(3)?,
                    payload: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })
            .map_err(|err| AppError::DbError(err.to_string()))?;

        let mut messages = Vec::new();
        for row in rows {
            messages.push(row.map_err(|err| AppError::DbError(err.to_string()))?);
        }
        Ok(messages)
    }

    pub fn insert(
        &self,
        label: Option<&str>,
        topic: &str,
        key: &str,
        payload: &str,
    ) -> Result<SavedMessage, AppError> {
        let conn = self.conn.lock().map_err(|err| AppError::DbError(err.to_string()))?;
        conn.execute(
            "INSERT INTO saved_messages (label, topic, key, payload) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![label, topic, key, payload],
        )
        .map_err(|err| AppError::DbError(err.to_string()))?;

        let id = conn.last_insert_rowid();
        let mut stmt = conn
            .prepare("SELECT id, label, topic, key, payload, created_at FROM saved_messages WHERE id = ?1")
            .map_err(|err| AppError::DbError(err.to_string()))?;

        stmt.query_row([id], |row| {
            Ok(SavedMessage {
                id: row.get(0)?,
                label: row.get(1)?,
                topic: row.get(2)?,
                key: row.get(3)?,
                payload: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|err| AppError::DbError(err.to_string()))
    }

    pub fn delete(&self, id: i64) -> Result<bool, AppError> {
        let conn = self.conn.lock().map_err(|err| AppError::DbError(err.to_string()))?;
        let affected = conn
            .execute("DELETE FROM saved_messages WHERE id = ?1", [id])
            .map_err(|err| AppError::DbError(err.to_string()))?;
        Ok(affected > 0)
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedMessage {
    pub id: i64,
    pub label: Option<String>,
    pub topic: String,
    pub key: String,
    pub payload: String,
    pub created_at: String,
}
