use crate::db::DbState;
use tauri::State;

#[tauri::command]
pub fn reset_progress(state: State<DbState>) -> Result<(), String> {
    let conn = state.conn.write().map_err(|e| e.to_string())?;
    conn.execute_batch(
        "DELETE FROM answer_records;
         DELETE FROM requirement_assessments;
         DELETE FROM task_assessments;"
    ).map_err(|e| e.to_string())?;
    Ok(())
}