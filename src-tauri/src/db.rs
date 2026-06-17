use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::models::*;

pub struct DbState {
    pub conn: Mutex<Connection>,
}

unsafe impl Send for DbState {}
unsafe impl Sync for DbState {}

const SCHEMA_VERSION: i64 = 1;

pub fn init_db(study_data_dir: &PathBuf) -> Result<Connection, String> {
    let db_path = study_data_dir.join("study.db");
    let is_new = !db_path.exists();

    let mut conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .map_err(|e| e.to_string())?;

    create_tables(&conn)?;

    if is_new {
        let seed_path = study_data_dir.join("seed.json");
        if seed_path.exists() {
            let json = std::fs::read_to_string(&seed_path).map_err(|e| e.to_string())?;
            import_seed_data_from_json(&mut conn, &json)?;
        } else {
            let embedded_seed: &str = include_str!("../../data/seed.json");
            import_seed_data_from_json(&mut conn, embedded_seed)?;
        }
    } else {
        run_migrations(&conn)?;
    }

    Ok(conn)
}

fn create_tables(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY
        );

        CREATE TABLE IF NOT EXISTS course_outcomes (
            id          TEXT PRIMARY KEY,
            code        TEXT NOT NULL,
            description TEXT NOT NULL,
            order_index INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS chapters (
            id                 TEXT PRIMARY KEY,
            title              TEXT NOT NULL,
            description        TEXT NOT NULL DEFAULT '',
            order_index        INTEGER NOT NULL,
            course_outcome_id  TEXT NOT NULL REFERENCES course_outcomes(id)
        );

        CREATE TABLE IF NOT EXISTS knowledge_points (
            id          TEXT PRIMARY KEY,
            chapter_id  TEXT NOT NULL REFERENCES chapters(id),
            name        TEXT NOT NULL,
            order_index INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS tasks (
            id                 TEXT PRIMARY KEY,
            chapter_id         TEXT NOT NULL REFERENCES chapters(id),
            course_outcome_id  TEXT NOT NULL REFERENCES course_outcomes(id),
            difficulty         TEXT NOT NULL CHECK(difficulty IN ('foundation','applied','advanced')),
            scenario           TEXT NOT NULL,
            reference          TEXT NOT NULL,
            source             TEXT,
            created_at         TEXT NOT NULL DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_tasks_chapter ON tasks(chapter_id);
        CREATE INDEX IF NOT EXISTS idx_tasks_difficulty ON tasks(difficulty);
        CREATE INDEX IF NOT EXISTS idx_tasks_outcome ON tasks(course_outcome_id);

        CREATE TABLE IF NOT EXISTS task_knowledge_points (
            task_id             TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
            knowledge_point_id  TEXT NOT NULL REFERENCES knowledge_points(id) ON DELETE CASCADE,
            PRIMARY KEY (task_id, knowledge_point_id)
        );

        CREATE TABLE IF NOT EXISTS task_requirements (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id     TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
            req_index   INTEGER NOT NULL,
            content     TEXT NOT NULL,
            UNIQUE(task_id, req_index)
        );
        CREATE INDEX IF NOT EXISTS idx_req_task ON task_requirements(task_id);

        CREATE TABLE IF NOT EXISTS task_sources (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id         TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
            university      TEXT NOT NULL,
            year            TEXT,
            exam_type       TEXT NOT NULL CHECK(exam_type IN ('final','postgraduate','obe','adapted')),
            original_text   TEXT,
            note            TEXT
        );
        CREATE INDEX IF NOT EXISTS idx_source_task ON task_sources(task_id);

        CREATE TABLE IF NOT EXISTS answer_records (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id     TEXT NOT NULL REFERENCES tasks(id),
            chapter_id  TEXT NOT NULL REFERENCES chapters(id),
            answer_text TEXT NOT NULL DEFAULT '',
            created_at  TEXT NOT NULL DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_answer_task ON answer_records(task_id);

        CREATE TABLE IF NOT EXISTS requirement_assessments (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id     TEXT NOT NULL REFERENCES tasks(id),
            req_index   INTEGER NOT NULL,
            status      TEXT NOT NULL CHECK(status IN ('mastered','needs_work')),
            created_at  TEXT NOT NULL DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_assess_task ON requirement_assessments(task_id);

        CREATE TABLE IF NOT EXISTS task_assessments (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id     TEXT NOT NULL REFERENCES tasks(id),
            chapter_id  TEXT NOT NULL REFERENCES chapters(id),
            assessment  TEXT NOT NULL CHECK(assessment IN ('mastered','needs_work')),
            created_at  TEXT NOT NULL DEFAULT (datetime('now'))
        );
        CREATE INDEX IF NOT EXISTS idx_task_assess ON task_assessments(task_id);
        CREATE INDEX IF NOT EXISTS idx_assess_task_req ON requirement_assessments(task_id, req_index, id DESC);
        CREATE INDEX IF NOT EXISTS idx_task_assess_latest ON task_assessments(task_id, id DESC);
        ",
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

fn run_migrations(conn: &Connection) -> Result<(), String> {
    let current: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    if current < SCHEMA_VERSION {
        conn.execute(
            "INSERT OR REPLACE INTO schema_version (version) VALUES (?1)",
            [SCHEMA_VERSION],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn import_seed_data_from_json(conn: &mut Connection, json: &str) -> Result<(), String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))
        .unwrap_or(0);

    if count > 0 {
        return Ok(());
    }

    let seed: SeedData = serde_json::from_str(json).map_err(|e| e.to_string())?;

    let tx = conn.transaction().map_err(|e| e.to_string())?;

    for co in &seed.courseOutcomes {
        tx.execute(
            "INSERT OR IGNORE INTO course_outcomes (id, code, description, order_index) VALUES (?1, ?2, ?3, ?4)",
            [&co.id, &co.code, &co.description, &co.orderIndex.to_string()],
        )
        .map_err(|e| e.to_string())?;
    }

    for ch in &seed.chapters {
        tx.execute(
            "INSERT OR IGNORE INTO chapters (id, title, description, order_index, course_outcome_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            [&ch.id, &ch.title, &ch.description, &ch.orderIndex.to_string(), &ch.courseOutcomeId],
        )
        .map_err(|e| e.to_string())?;
    }

    for kp in &seed.knowledgePoints {
        tx.execute(
            "INSERT OR IGNORE INTO knowledge_points (id, chapter_id, name, order_index) VALUES (?1, ?2, ?3, ?4)",
            [&kp.id, &kp.chapterId, &kp.name, &kp.orderIndex.to_string()],
        )
        .map_err(|e| e.to_string())?;
    }

    for task in &seed.tasks {
        tx.execute(
            "INSERT OR IGNORE INTO tasks (id, chapter_id, course_outcome_id, difficulty, scenario, reference) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [&task.id, &task.chapterId, &task.courseOutcomeId, &task.difficulty, &task.scenario, &task.reference],
        )
        .map_err(|e| e.to_string())?;

        for (i, req) in task.requirements.iter().enumerate() {
            let idx = i as i64;
            tx.execute(
                "INSERT OR IGNORE INTO task_requirements (task_id, req_index, content) VALUES (?1, ?2, ?3)",
                [&task.id, &idx.to_string(), req],
            )
            .map_err(|e| e.to_string())?;
        }

        for kp_id in &task.knowledgePointIds {
            tx.execute(
                "INSERT OR IGNORE INTO task_knowledge_points (task_id, knowledge_point_id) VALUES (?1, ?2)",
                [&task.id, kp_id],
            )
            .map_err(|e| e.to_string())?;
        }

        for src in &task.sources {
            tx.execute(
                "INSERT INTO task_sources (task_id, university, year, exam_type, original_text, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![
                    &task.id,
                    &src.university,
                    &src.year,
                    &src.examType,
                    &src.originalText,
                    &src.note,
                ],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    tx.execute(
        "INSERT INTO schema_version (version) VALUES (?1)",
        [SCHEMA_VERSION],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
        create_tables(&conn).unwrap();
        conn
    }

    #[test]
    fn test_create_tables() {
        let conn = test_conn();
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(count >= 5, "Should create at least 5 tables");
    }

    #[test]
    fn test_seed_import_empty_db() {
        let mut conn = test_conn();
        let embedded_seed: &str = include_str!("../../data/seed.json");
        let result = import_seed_data_from_json(&mut conn, embedded_seed);
        assert!(result.is_ok(), "Seed import should succeed on empty db");
        let task_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))
            .unwrap();
        assert!(task_count > 0, "Should have tasks after seed import");
    }

    #[test]
    fn test_seed_import_idempotent() {
        let mut conn = test_conn();
        let embedded_seed: &str = include_str!("../../data/seed.json");
        import_seed_data_from_json(&mut conn, embedded_seed).unwrap();
        let first_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))
            .unwrap();
        import_seed_data_from_json(&mut conn, embedded_seed).unwrap();
        let second_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))
            .unwrap();
        assert_eq!(first_count, second_count, "Second import should not duplicate data");
    }
}