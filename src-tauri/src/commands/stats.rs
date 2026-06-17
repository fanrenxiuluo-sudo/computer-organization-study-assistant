use crate::db::DbState;
use crate::models::*;
use tauri::State;

const LATEST_TASK_ASSESSMENTS_SQL: &str = "
    SELECT ta.task_id, ta.chapter_id, ta.assessment
    FROM task_assessments ta
    INNER JOIN (
        SELECT task_id, MAX(id) AS max_id
        FROM task_assessments
        GROUP BY task_id
    ) latest ON latest.max_id = ta.id
";

#[tauri::command]
pub fn get_chapter_progress(
    state: State<DbState>,
    chapter_id: String,
) -> Result<ChapterProgress, String> {
    let conn = state.conn.read().map_err(|e| e.to_string())?;

    let total_tasks: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tasks WHERE chapter_id = ?1",
            [&chapter_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let assessed: i64 = conn
        .query_row(
            &format!("SELECT COUNT(*) FROM ({LATEST_TASK_ASSESSMENTS_SQL}) latest WHERE latest.chapter_id = ?1"),
            [&chapter_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let mastered: i64 = conn
        .query_row(
            &format!("SELECT COUNT(*) FROM ({LATEST_TASK_ASSESSMENTS_SQL}) latest WHERE latest.chapter_id = ?1 AND latest.assessment = 'mastered'"),
            [&chapter_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let needs_work: i64 = conn
        .query_row(
            &format!("SELECT COUNT(*) FROM ({LATEST_TASK_ASSESSMENTS_SQL}) latest WHERE latest.chapter_id = ?1 AND latest.assessment = 'needs_work'"),
            [&chapter_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Batch query: all knowledge points + their task counts + mastered counts in one go
    let mut kp_stmt = conn
        .prepare(
            "SELECT kp.id, kp.name FROM knowledge_points kp WHERE kp.chapter_id = ?1 ORDER BY kp.order_index"
        )
        .map_err(|e| e.to_string())?;
    let kp_rows: Vec<(String, String)> = kp_stmt
        .query_map([&chapter_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    if kp_rows.is_empty() {
        return Ok(ChapterProgress {
            total_tasks,
            assessed,
            mastered,
            needs_work,
            knowledge_points: vec![],
        });
    }

    // Build IN clause for batch querying all KPs at once
    let placeholders: Vec<String> = (0..kp_rows.len()).map(|i| format!("?{}", i + 1)).collect();
    let in_clause = placeholders.join(", ");
    let kp_ids: Vec<&str> = kp_rows.iter().map(|(id, _)| id.as_str()).collect();
    let params: Vec<&dyn rusqlite::types::ToSql> = kp_ids.iter().map(|id| id as &dyn rusqlite::types::ToSql).collect();

    // Batch: total tasks and mastered count per knowledge point
    let batch_sql = format!(
        "SELECT tkp.knowledge_point_id,
                COUNT(tkp.task_id) AS total,
                SUM(CASE WHEN latest.assessment = 'mastered' THEN 1 ELSE 0 END) AS mastered
         FROM task_knowledge_points tkp
         LEFT JOIN ({LATEST_TASK_ASSESSMENTS_SQL}) latest ON latest.task_id = tkp.task_id
         WHERE tkp.knowledge_point_id IN ({in_clause})
         GROUP BY tkp.knowledge_point_id"
    );

    let mut batch_stmt = conn.prepare(&batch_sql).map_err(|e| e.to_string())?;
    let batch_results: Vec<(String, i64, i64)> = batch_stmt
        .query_map(params.as_slice(), |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    // Build lookup map from batch results
    let mut lookup: std::collections::HashMap<String, (i64, i64)> = std::collections::HashMap::new();
    for (kp_id, total, mastered) in &batch_results {
        lookup.insert(kp_id.clone(), (*total, *mastered));
    }

    let knowledge_points: Vec<KnowledgePointProgress> = kp_rows
        .into_iter()
        .map(|(kp_id, kp_name)| {
            let (total, kp_mastered) = lookup.remove(&kp_id).unwrap_or((0, 0));
            let pct = if total > 0 {
                (kp_mastered as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            KnowledgePointProgress {
                knowledge_point_id: kp_id,
                name: kp_name,
                total_tasks: total,
                mastered: kp_mastered,
                mastery_percent: pct,
            }
        })
        .collect();

    Ok(ChapterProgress {
        total_tasks,
        assessed,
        mastered,
        needs_work,
        knowledge_points,
    })
}

#[tauri::command]
pub fn get_overall_progress(state: State<DbState>) -> Result<OverallProgress, String> {
    let conn = state.conn.read().map_err(|e| e.to_string())?;

    let total_tasks: i64 = conn
        .query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    let mastered: i64 = conn
        .query_row(
            &format!("SELECT COUNT(*) FROM ({LATEST_TASK_ASSESSMENTS_SQL}) latest WHERE latest.assessment = 'mastered'"),
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    let needs_work: i64 = conn
        .query_row(
            &format!("SELECT COUNT(*) FROM ({LATEST_TASK_ASSESSMENTS_SQL}) latest WHERE latest.assessment = 'needs_work'"),
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let mut outcome_stmt = conn
        .prepare("SELECT id, code, description FROM course_outcomes ORDER BY order_index")
        .map_err(|e| e.to_string())?;
    let outcome_rows: Vec<(String, String, String)> = outcome_stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    if outcome_rows.is_empty() {
        return Ok(OverallProgress {
            total_tasks,
            mastered,
            needs_work,
            outcomes: vec![],
        });
    }

    // Batch query: total and mastered per course outcome
    let placeholders: Vec<String> = (0..outcome_rows.len()).map(|i| format!("?{}", i + 1)).collect();
    let in_clause = placeholders.join(", ");
    let outcome_ids: Vec<&str> = outcome_rows.iter().map(|(id, _, _)| id.as_str()).collect();
    let params: Vec<&dyn rusqlite::types::ToSql> = outcome_ids.iter().map(|id| id as &dyn rusqlite::types::ToSql).collect();

    let batch_sql = format!(
        "SELECT t.course_outcome_id,
                COUNT(t.id) AS total,
                SUM(CASE WHEN latest.assessment = 'mastered' THEN 1 ELSE 0 END) AS mastered
         FROM tasks t
         LEFT JOIN ({LATEST_TASK_ASSESSMENTS_SQL}) latest ON latest.task_id = t.id
         WHERE t.course_outcome_id IN ({in_clause})
         GROUP BY t.course_outcome_id"
    );

    let mut batch_stmt = conn.prepare(&batch_sql).map_err(|e| e.to_string())?;
    let batch_results: Vec<(String, i64, i64)> = batch_stmt
        .query_map(params.as_slice(), |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    let mut lookup: std::collections::HashMap<String, (i64, i64)> = std::collections::HashMap::new();
    for (oid, total, mastered) in &batch_results {
        lookup.insert(oid.clone(), (*total, *mastered));
    }

    let outcomes: Vec<OutcomeProgress> = outcome_rows
        .into_iter()
        .map(|(outcome_id, code, description)| {
            let (co_total, co_mastered) = lookup.remove(&outcome_id).unwrap_or((0, 0));
            let pct = if co_total > 0 {
                (co_mastered as f64 / co_total as f64) * 100.0
            } else {
                0.0
            };
            OutcomeProgress {
                outcome_id,
                code,
                description,
                mastery_percent: pct,
            }
        })
        .collect();

    Ok(OverallProgress {
        total_tasks,
        mastered,
        needs_work,
        outcomes,
    })
}
