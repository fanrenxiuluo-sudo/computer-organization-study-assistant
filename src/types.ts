export type Difficulty = "foundation" | "applied" | "advanced";
export type Assessment = "mastered" | "needs_work";
export type ReqStatus = "unassessed" | "mastered" | "needs_work";
export type ExamType = "final" | "postgraduate" | "obe" | "adapted";

export interface RequirementStatus {
  reqIndex: number;
  status: "unassessed" | "mastered" | "needs_work";
}

export interface CourseOutcome {
  id: string;
  code: string;
  description: string;
  orderIndex: number;
}

export interface Chapter {
  id: string;
  title: string;
  description: string;
  orderIndex: number;
  courseOutcomeId: string;
}

export interface KnowledgePoint {
  id: string;
  chapterId: string;
  name: string;
  orderIndex: number;
}

export interface Task {
  id: string;
  chapterId: string;
  courseOutcomeId: string;
  difficulty: Difficulty;
  scenario: string;
  reference: string;
  source?: string;
}

export interface TaskRequirement {
  id: number;
  taskId: string;
  reqIndex: number;
  content: string;
}

export interface TaskKnowledgePoint {
  taskId: string;
  knowledgePointId: string;
}

export interface TaskSource {
  university: string;
  year: string | null;
  examType: ExamType;
  originalText?: string;
  note?: string;
}

export interface TaskDetail {
  task: Task;
  requirements: TaskRequirement[];
  knowledgePoints: KnowledgePoint[];
  sources: TaskSource[];
  latestAssessment?: Assessment;
  latestAnswer?: string;
  requirementStatuses: RequirementStatus[];
}

export interface TaskPage {
  items: TaskDetail[];
  total: number;
  offset: number;
}

export interface KnowledgePointProgress {
  knowledgePointId: string;
  name: string;
  totalTasks: number;
  mastered: number;
  masteryPercent: number;
}

export interface ChapterProgress {
  totalTasks: number;
  assessed: number;
  mastered: number;
  needsWork: number;
  knowledgePoints: KnowledgePointProgress[];
}

export interface OutcomeProgress {
  outcomeId: string;
  code: string;
  description: string;
  masteryPercent: number;
}

export interface OverallProgress {
  totalTasks: number;
  mastered: number;
  needsWork: number;
  outcomes: OutcomeProgress[];
}

export interface ImportResult {
  imported: number;
  skipped: number;
}

export type PracticeMode = "sequential" | "weak" | "random" | "knowledge-point";

export interface SeedTask {
  id: string;
  chapterId: string;
  courseOutcomeId: string;
  knowledgePointIds: string[];
  difficulty: Difficulty;
  scenario: string;
  requirements: string[];
  reference: string;
  sources: TaskSource[];
}

export interface SeedData {
  courseOutcomes: CourseOutcome[];
  chapters: Chapter[];
  knowledgePoints: KnowledgePoint[];
  tasks: SeedTask[];
}