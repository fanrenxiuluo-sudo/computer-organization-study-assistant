use crate::db::DbState;
use crate::models::*;
use tauri::State;

#[tauri::command]
pub fn list_tasks(
    state: State<DbState>,
    chapter_id: String,
    difficulty: Option<String>,
    knowledge_point_id: Option<String>,
    offset: i64,
    limit: i64,
) -> Result<TaskPage, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;

    let limit = limit.clamp(1, 50);
    let offset = offset.max(0);

    let mut where_clauses = vec!["t.chapter_id = ?1".to_string()];
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = vec![Box::new(chapter_id.clone())];
    let mut param_idx = 2;

    if let Some(ref diff) = difficulty {
        where_clauses.push(format!("t.difficulty = ?{}", param_idx));
        param_values.push(Box::new(diff.clone()));
        param_idx += 1;
    }

    if let Some(ref kp_id) = knowledge_point_id {
        where_clauses.push(format!(
            "t.id IN (SELECT task_id FROM task_knowledge_points WHERE knowledge_point_id = ?{})",
            param_idx
        ));
        param_values.push(Box::new(kp_id.clone()));
        param_idx += 1;
    }

    let where_sql = where_clauses.join(" AND ");

    let count_sql = format!("SELECT COUNT(*) FROM tasks t WHERE {}", where_sql);
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
    let total: i64 = conn.query_row(&count_sql, params_refs.as_slice(), |row| row.get(0)).map_err(|e| e.to_string())?;

    let query_sql = format!(
        "SELECT id, chapter_id, course_outcome_id, difficulty, scenario, reference, source FROM tasks t WHERE {} ORDER BY id LIMIT ?{} OFFSET ?{}",
        where_sql, param_idx, param_idx + 1
    );
    param_values.push(Box::new(limit));
    param_values.push(Box::new(offset));
    let params_refs2: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn.prepare(&query_sql).map_err(|e| e.to_string())?;
    let task_rows = stmt.query_map(params_refs2.as_slice(), |row| {
        Ok(Task {
            id: row.get(0)?,
            chapter_id: row.get(1)?,
            course_outcome_id: row.get(2)?,
            difficulty: row.get(3)?,
            scenario: row.get(4)?,
            reference: row.get(5)?,
            source: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for row in task_rows {
        let task = row.map_err(|e| e.to_string())?;
        let detail = build_task_detail(&conn, &task)?;
        items.push(detail);
    }

    Ok(TaskPage {
        items,
        total,
        offset,
    })
}

#[tauri::command]
pub fn get_task_detail(state: State<DbState>, task_id: String) -> Result<TaskDetail, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;

    let task = conn.query_row(
        "SELECT id, chapter_id, course_outcome_id, difficulty, scenario, reference, source FROM tasks WHERE id = ?1",
        [&task_id],
        |row| {
            Ok(Task {
                id: row.get(0)?,
                chapter_id: row.get(1)?,
                course_outcome_id: row.get(2)?,
                difficulty: row.get(3)?,
                scenario: row.get(4)?,
                reference: row.get(5)?,
                source: row.get(6)?,
            })
        },
    ).map_err(|e| e.to_string())?;

    build_task_detail(&conn, &task)
}

#[tauri::command]
pub fn get_random_tasks(
    state: State<DbState>,
    chapter_id: Option<String>,
    difficulty: Option<String>,
    count: i64,
    only_weak: bool,
) -> Result<Vec<TaskDetail>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let count = count.clamp(1, 20);

    let mut conditions = vec![];
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];
    let mut param_idx = 1;

    if let Some(ref ch_id) = chapter_id {
        conditions.push(format!("t.chapter_id = ?{}", param_idx));
        param_values.push(Box::new(ch_id.clone()));
        param_idx += 1;
    }

    if let Some(ref diff) = difficulty {
        conditions.push(format!("t.difficulty = ?{}", param_idx));
        param_values.push(Box::new(diff.clone()));
        param_idx += 1;
    }

    if only_weak {
        conditions.push(format!(
            "t.id IN (SELECT task_id FROM task_assessments WHERE assessment = 'needs_work')"
        ));
    }

    let where_sql = if conditions.is_empty() {
        "1=1".to_string()
    } else {
        conditions.join(" AND ")
    };

    let sql = format!(
        "SELECT id, chapter_id, course_outcome_id, difficulty, scenario, reference, source FROM tasks t WHERE {} ORDER BY RANDOM() LIMIT ?{}",
        where_sql, param_idx
    );
    param_values.push(Box::new(count));
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let task_rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(Task {
            id: row.get(0)?,
            chapter_id: row.get(1)?,
            course_outcome_id: row.get(2)?,
            difficulty: row.get(3)?,
            scenario: row.get(4)?,
            reference: row.get(5)?,
            source: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for row in task_rows {
        let task = row.map_err(|e| e.to_string())?;
        let detail = build_task_detail(&conn, &task)?;
        items.push(detail);
    }

    Ok(items)
}

#[tauri::command]
pub fn list_knowledge_points(
    state: State<DbState>,
    chapter_id: Option<String>,
) -> Result<Vec<KnowledgePoint>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    if let Some(ch_id) = chapter_id {
        let mut stmt = conn
            .prepare("SELECT id, chapter_id, name, order_index FROM knowledge_points WHERE chapter_id = ?1 ORDER BY order_index")
            .map_err(|e| e.to_string())?;
        let rows = stmt.query_map([&ch_id], |row| {
            Ok(KnowledgePoint {
                id: row.get(0)?,
                chapter_id: row.get(1)?,
                name: row.get(2)?,
                order_index: row.get(3)?,
            })
        }).map_err(|e| e.to_string())?;
        for row in rows {
            result.push(row.map_err(|e| e.to_string())?);
        }
    } else {
        let mut stmt = conn
            .prepare("SELECT id, chapter_id, name, order_index FROM knowledge_points ORDER BY chapter_id, order_index")
            .map_err(|e| e.to_string())?;
        let rows = stmt.query_map([], |row| {
            Ok(KnowledgePoint {
                id: row.get(0)?,
                chapter_id: row.get(1)?,
                name: row.get(2)?,
                order_index: row.get(3)?,
            })
        }).map_err(|e| e.to_string())?;
        for row in rows {
            result.push(row.map_err(|e| e.to_string())?);
        }
    }

    Ok(result)
}

fn build_task_detail(conn: &rusqlite::Connection, task: &Task) -> Result<TaskDetail, String> {
    let task_id = &task.id;

    let mut req_stmt = conn.prepare("SELECT id, task_id, req_index, content FROM task_requirements WHERE task_id = ?1 ORDER BY req_index")
        .map_err(|e| e.to_string())?;
    let requirements: Vec<TaskRequirement> = req_stmt.query_map([task_id], |row| {
        Ok(TaskRequirement {
            id: row.get(0)?,
            task_id: row.get(1)?,
            req_index: row.get(2)?,
            content: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();

    let mut kp_stmt = conn.prepare(
        "SELECT kp.id, kp.chapter_id, kp.name, kp.order_index FROM knowledge_points kp INNER JOIN task_knowledge_points tkp ON kp.id = tkp.knowledge_point_id WHERE tkp.task_id = ?1 ORDER BY kp.order_index"
    ).map_err(|e| e.to_string())?;
    let knowledge_points: Vec<KnowledgePoint> = kp_stmt.query_map([task_id], |row| {
        Ok(KnowledgePoint {
            id: row.get(0)?,
            chapter_id: row.get(1)?,
            name: row.get(2)?,
            order_index: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();

    let mut src_stmt = conn.prepare("SELECT id, task_id, university, year, exam_type, original_text, note FROM task_sources WHERE task_id = ?1")
        .map_err(|e| e.to_string())?;
    let sources: Vec<TaskSource> = src_stmt.query_map([task_id], |row| {
        Ok(TaskSource {
            id: row.get(0)?,
            task_id: row.get(1)?,
            university: row.get(2)?,
            year: row.get(3)?,
            exam_type: row.get(4)?,
            original_text: row.get(5)?,
            note: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();

    let latest_assessment: Option<String> = conn.query_row(
        "SELECT assessment FROM task_assessments WHERE task_id = ?1 ORDER BY created_at DESC LIMIT 1",
        [task_id],
        |row| row.get(0),
    ).ok();

    let latest_answer: Option<String> = conn.query_row(
        "SELECT answer_text FROM answer_records WHERE task_id = ?1 ORDER BY created_at DESC LIMIT 1",
        [task_id],
        |row| row.get(0),
    ).ok();

    let mut req_statuses: Vec<ReqStatus> = Vec::new();
    for req in &requirements {
        let status: Option<String> = conn.query_row(
            "SELECT status FROM requirement_assessments WHERE task_id = ?1 AND req_index = ?2 ORDER BY created_at DESC LIMIT 1",
            rusqlite::params![task_id, req.req_index],
            |row| row.get(0),
        ).ok();
        req_statuses.push(ReqStatus {
            req_index: req.req_index,
            status: status.unwrap_or_else(|| "unassessed".to_string()),
        });
    }

    Ok(TaskDetail {
        task: task.clone(),
        requirements,
        knowledge_points,
        sources,
        latest_assessment,
        latest_answer,
        requirement_statuses: req_statuses,
    })
}