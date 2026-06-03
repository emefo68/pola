use std::path::Path;

use rusqlite::{Connection, Error};

const INITIAL_SCHEMA: &str = include_str!("schema.sql");

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let mut conn = Connection::open(path)?;

        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;",
        )?;

        apply_schema(&mut conn)?;

        Ok(Database { conn })
    }
}

fn apply_schema(conn: &mut Connection) -> Result<(), Error> {
    let current_version: i32 = conn.query_row("PRAGMA user_version;", [], |row| row.get(0))?;

    if current_version < 1 {
        let tx = conn.transaction()?;

        tx.execute_batch(INITIAL_SCHEMA)?;

        tx.execute_batch("PRAGMA user_version = 1;")?;
        tx.commit()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_initialization() {
        let mut conn = Connection::open_in_memory().unwrap();

        conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();

        let result = apply_schema(&mut conn);

        assert!(result.is_ok(), "Failed to apply schema: {:?}", result.err());

        let version: i32 = conn
            .query_row("PRAGMA user_version;", [], |row| row.get(0))
            .unwrap();
        assert_eq!(version, 1);

        let mut stmt = conn
            .prepare(
                "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%';",
            )
            .unwrap();

        let table_iter = stmt.query_map([], |row| row.get::<_, String>(0)).unwrap();

        let actual_tables: Vec<String> = table_iter.map(Result::unwrap).collect();

        assert!(
            actual_tables.contains(&"workspaces".to_string()),
            "workspaces table missing"
        );
        assert!(
            actual_tables.contains(&"domains".to_string()),
            "domains table missing"
        );
        assert!(
            actual_tables.contains(&"files".to_string()),
            "files table missing"
        );
        assert!(
            actual_tables.contains(&"vocabulary".to_string()),
            "vocabulary table missing"
        );
        assert!(
            actual_tables.contains(&"class_term_counts".to_string()),
            "class_term_counts table missing"
        );
        assert!(
            actual_tables.contains(&"source_folders".to_string()),
            "source_folders table missing"
        );
        assert!(
            actual_tables.contains(&"workspace_source_folders".to_string()),
            "workspace_source_folders table missing"
        );
        assert!(
            actual_tables.contains(&"keywords".to_string()),
            "keywords table missing"
        );
        assert!(
            actual_tables.contains(&"file_tokens".to_string()),
            "file_tokens table missing"
        );
        assert!(
            actual_tables.contains(&"classifications".to_string()),
            "classifications table missing"
        );
        assert!(
            actual_tables.contains(&"files_fts".to_string()),
            "files_fts table missing"
        );
    }
}
