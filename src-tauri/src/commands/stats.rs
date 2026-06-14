use crate::db::DbState;
use crate::models::*;
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub fn get_chapter_progress(
    state: State<DbState>,
    chapter_id: String,
) -> Result<ChapterProgress, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;

    let total_tasks: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tasks WHERE chapter_id = ?1",
            [&chapter_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let mastered: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT task_id) FROM task_assessments WHERE chapter_id = ?1 AND assessment = 'mastered'",
            [&chapter_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let assessed: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT task_id) FROM task_assessments WHERE chapter_id = ?1",
            [&chapter_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let needs_work: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT task_id) FROM task_assessments WHERE chapter_id = ?1 AND assessment = 'needs_work'",
            [&chapter_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let mut kp_stmt = conn.prepare(
        "SELECT kp.id, kp.name FROM knowledge_points kp WHERE kp.chapter_id = ?1 ORDER BY kp.order_index"
    ).map_err(|e| e.to_string())?;
    let kp_rows = kp_stmt.query_map([&chapter_id], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    }).map_err(|e| e.to_string())?;

    let mut knowledge_points = Vec::new();
    for kp_row in kp_rows {
        let (kp_id, kp_name) = kp_row.map_err(|e| e.to_string())?;
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM task_knowledge_points tkp WHERE tkp.knowledge_point_id = ?1",
            [&kp_id],
            |row| row.get(0),
        ).map_err(|e| e.to_string())?;

        let kp_mastered: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT ta.task_id) FROM task_assessments ta INNER JOIN task_knowledge_points tkp ON ta.task_id = tkp.task_id WHERE tkp.knowledge_point_id = ?1 AND ta.assessment = 'mastered'",
            [&kp_id],
            |row| row.get(0),
        ).map_err(|e| e.to_string())?;

        let pct = if total > 0 { (kp_mastered as f64 / total as f64) * 100.0 } else { 0.0 };

        knowledge_points.push(KnowledgePointProgress {
            knowledge_point_id: kp_id,
            name: kp_name,
            total_tasks: total,
            mastered: kp_mastered,
            mastery_percent: pct,
        });
    }

    Ok(ChapterProgress {
        total_tasks: total_tasks,
        assessed: assessed,
        mastered: mastered,
        needs_work: needs_work,
        knowledge_points: knowledge_points,
    })
}

#[tauri::command]
pub fn get_overall_progress(state: State<DbState>) -> Result<OverallProgress, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;

    let total_tasks: i64 = conn.query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0)).map_err(|e| e.to_string())?;
    let mastered: i64 = conn.query_row("SELECT COUNT(DISTINCT task_id) FROM task_assessments WHERE assessment = 'mastered'", [], |row| row.get(0)).map_err(|e| e.to_string())?;
    let needs_work: i64 = conn.query_row("SELECT COUNT(DISTINCT task_id) FROM task_assessments WHERE assessment = 'needs_work'", [], |row| row.get(0)).map_err(|e| e.to_string())?;

    let mut outcome_stmt = conn.prepare(
        "SELECT id, code, description FROM course_outcomes ORDER BY order_index"
    ).map_err(|e| e.to_string())?;
    let outcome_rows = outcome_stmt.query_map([], |row| {
        Ok(OutcomeProgress {
            outcome_id: row.get(0)?,
            code: row.get(1)?,
            description: row.get(2)?,
            mastery_percent: 0.0,
        })
    }).map_err(|e| e.to_string())?;

    let mut outcomes = Vec::new();
    for row in outcome_rows {
        let mut op = row.map_err(|e| e.to_string())?;
        let co_total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE course_outcome_id = ?1",
            [&op.outcome_id],
            |row| row.get(0),
        ).map_err(|e| e.to_string())?;
        let co_mastered: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT ta.task_id) FROM task_assessments ta INNER JOIN tasks t ON ta.task_id = t.id WHERE t.course_outcome_id = ?1 AND ta.assessment = 'mastered'",
            [&op.outcome_id],
            |row| row.get(0),
        ).map_err(|e| e.to_string())?;
        op.mastery_percent = if co_total > 0 { (co_mastered as f64 / co_total as f64) * 100.0 } else { 0.0 };
        outcomes.push(op);
    }

    Ok(OverallProgress {
        total_tasks: total_tasks,
        mastered: mastered,
        needs_work: needs_work,
        outcomes: outcomes,
    })
}