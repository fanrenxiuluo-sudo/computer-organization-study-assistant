use crate::db::DbState;
use tauri::State;

#[tauri::command]
pub fn save_answer(
    state: State<DbState>,
    task_id: String,
    chapter_id: String,
    answer_text: String,
) -> Result<(), String> {
    let mut conn = state.conn.write().map_err(|e| e.to_string())?;
    let latest_answer: Option<String> = conn
        .query_row(
            "SELECT answer_text FROM answer_records WHERE task_id = ?1 ORDER BY id DESC LIMIT 1",
            [&task_id],
            |row| row.get(0),
        )
        .ok();

    if latest_answer.as_deref() == Some(answer_text.as_str()) {
        return Ok(());
    }

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
    if status != "mastered" && status != "needs_work" {
        return Err(format!("status 必须为 mastered 或 needs_work，收到: {status}"));
    }
    let conn = state.conn.write().map_err(|e| e.to_string())?;
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
    if assessment != "mastered" && assessment != "needs_work" {
        return Err(format!("assessment 必须为 mastered 或 needs_work，收到: {assessment}"));
    }
    let conn = state.conn.write().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO task_assessments (task_id, chapter_id, assessment) VALUES (?1, ?2, ?3)",
        rusqlite::params![task_id, chapter_id, assessment],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}