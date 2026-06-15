use crate::db::DbState;
use tauri::State;

#[tauri::command]
pub fn save_answer(
    state: State<DbState>,
    task_id: String,
    chapter_id: String,
    answer_text: String,
) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO answer_records (task_id, chapter_id, answer_text) VALUES (?1, ?2, ?3)",
        rusqlite::params![task_id, chapter_id, answer_text],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn assess_requirement(
    state: State<DbState>,
    task_id: String,
    req_index: i64,
    status: String,
) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO requirement_assessments (task_id, req_index, status) VALUES (?1, ?2, ?3)",
        rusqlite::params![task_id, req_index, status],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn assess_task(
    state: State<DbState>,
    task_id: String,
    chapter_id: String,
    assessment: String,
) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO task_assessments (task_id, chapter_id, assessment) VALUES (?1, ?2, ?3)",
        rusqlite::params![task_id, chapter_id, assessment],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}