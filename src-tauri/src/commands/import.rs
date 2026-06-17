use crate::db::DbState;
use crate::models::ImportResult;
use tauri::State;

#[tauri::command]
pub fn import_tasks(
    state: State<DbState>,
    json_string: String,
) -> Result<ImportResult, String> {
    let seed: crate::models::SeedData =
        serde_json::from_str(&json_string).map_err(|e| e.to_string())?;

    let mut conn = state.conn.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Insert prerequisite data first (course outcomes, chapters, knowledge points)
    // so that task foreign keys don't fail.
    for co in &seed.courseOutcomes {
        tx.execute(
            "INSERT OR IGNORE INTO course_outcomes (id, code, description, order_index) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![&co.id, &co.code, &co.description, &co.orderIndex],
        )
        .map_err(|e| e.to_string())?;
    }

    for ch in &seed.chapters {
        tx.execute(
            "INSERT OR IGNORE INTO chapters (id, title, description, order_index, course_outcome_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![&ch.id, &ch.title, &ch.description, &ch.orderIndex, &ch.courseOutcomeId],
        )
        .map_err(|e| e.to_string())?;
    }

    for kp in &seed.knowledgePoints {
        tx.execute(
            "INSERT OR IGNORE INTO knowledge_points (id, chapter_id, name, order_index) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![&kp.id, &kp.chapterId, &kp.name, &kp.orderIndex],
        )
        .map_err(|e| e.to_string())?;
    }

    let mut imported: i64 = 0;
    let mut skipped: i64 = 0;

    for task in &seed.tasks {
        // INSERT OR IGNORE handles the duplicate check — no need for a separate SELECT COUNT
        let rows = tx.execute(
            "INSERT OR IGNORE INTO tasks (id, chapter_id, course_outcome_id, difficulty, scenario, reference) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                &task.id,
                &task.chapterId,
                &task.courseOutcomeId,
                &task.difficulty,
                &task.scenario,
                &task.reference,
            ],
        )
        .map_err(|e| e.to_string())?;

        if rows == 0 {
            skipped += 1;
            continue;
        }

        for (i, req) in task.requirements.iter().enumerate() {
            let idx = i as i64;
            tx.execute(
                "INSERT OR IGNORE INTO task_requirements (task_id, req_index, content) VALUES (?1, ?2, ?3)",
                rusqlite::params![&task.id, idx, req],
            )
            .map_err(|e| e.to_string())?;
        }

        for kp_id in &task.knowledgePointIds {
            tx.execute(
                "INSERT OR IGNORE INTO task_knowledge_points (task_id, knowledge_point_id) VALUES (?1, ?2)",
                rusqlite::params![&task.id, kp_id],
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

        imported += 1;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(ImportResult { imported, skipped })
}
