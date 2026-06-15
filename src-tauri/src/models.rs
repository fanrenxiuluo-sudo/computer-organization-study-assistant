use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseOutcome {
    pub id: String,
    pub code: String,
    pub description: String,
    pub order_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub description: String,
    pub order_index: i64,
    pub course_outcome_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgePoint {
    pub id: String,
    pub chapter_id: String,
    pub name: String,
    pub order_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub chapter_id: String,
    pub course_outcome_id: String,
    pub difficulty: String,
    pub scenario: String,
    pub reference: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequirement {
    pub id: i64,
    pub task_id: String,
    pub req_index: i64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TaskKnowledgePoint {
    pub task_id: String,
    pub knowledge_point_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSource {
    pub id: i64,
    pub task_id: String,
    pub university: String,
    pub year: Option<String>,
    pub exam_type: String,
    pub original_text: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDetail {
    pub task: Task,
    pub requirements: Vec<TaskRequirement>,
    pub knowledge_points: Vec<KnowledgePoint>,
    pub sources: Vec<TaskSource>,
    pub latest_assessment: Option<String>,
    pub latest_answer: Option<String>,
    pub requirement_statuses: Vec<ReqStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReqStatus {
    pub req_index: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPage {
    pub items: Vec<TaskDetail>,
    pub total: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgePointProgress {
    pub knowledge_point_id: String,
    pub name: String,
    pub total_tasks: i64,
    pub mastered: i64,
    pub mastery_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterProgress {
    pub total_tasks: i64,
    pub assessed: i64,
    pub mastered: i64,
    pub needs_work: i64,
    pub knowledge_points: Vec<KnowledgePointProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomeProgress {
    pub outcome_id: String,
    pub code: String,
    pub description: String,
    pub mastery_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallProgress {
    pub total_tasks: i64,
    pub mastered: i64,
    pub needs_work: i64,
    pub outcomes: Vec<OutcomeProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub imported: i64,
    pub skipped: i64,
}

// ─── Seed data types (match seed.json) ─────────────────

#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
pub struct SeedCourseOutcome {
    pub id: String,
    pub code: String,
    pub description: String,
    pub orderIndex: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
pub struct SeedChapter {
    pub id: String,
    pub title: String,
    pub description: String,
    pub orderIndex: i64,
    pub courseOutcomeId: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
pub struct SeedKnowledgePoint {
    pub id: String,
    pub chapterId: String,
    pub name: String,
    pub orderIndex: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
pub struct SeedTaskSource {
    pub university: String,
    #[serde(default)]
    pub year: Option<String>,
    pub examType: String,
    #[serde(default)]
    pub originalText: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
pub struct SeedTask {
    pub id: String,
    pub chapterId: String,
    pub courseOutcomeId: String,
    pub knowledgePointIds: Vec<String>,
    pub difficulty: String,
    pub scenario: String,
    pub requirements: Vec<String>,
    pub reference: String,
    #[serde(default)]
    pub sources: Vec<SeedTaskSource>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
pub struct SeedData {
    pub courseOutcomes: Vec<SeedCourseOutcome>,
    pub chapters: Vec<SeedChapter>,
    pub knowledgePoints: Vec<SeedKnowledgePoint>,
    pub tasks: Vec<SeedTask>,
}