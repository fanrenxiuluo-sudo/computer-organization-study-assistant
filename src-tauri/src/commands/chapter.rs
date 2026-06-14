use crate::db::DbState;
use crate::models::{Chapter, CourseOutcome};
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub fn list_chapters(state: State<DbState>) -> Result<Vec<Chapter>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, title, description, order_index, course_outcome_id FROM chapters ORDER BY order_index")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Chapter {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                order_index: row.get(3)?,
                course_outcome_id: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn list_course_outcomes(state: State<DbState>) -> Result<Vec<CourseOutcome>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, code, description, order_index FROM course_outcomes ORDER BY order_index")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(CourseOutcome {
                id: row.get(0)?,
                code: row.get(1)?,
                description: row.get(2)?,
                order_index: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}