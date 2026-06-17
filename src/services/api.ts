import { invoke } from "@tauri-apps/api/core";
import type {
  CourseOutcome,
  Chapter,
  KnowledgePoint,
  TaskDetail,
  TaskPage,
  ChapterProgress,
  OverallProgress,
  ImportResult,
} from "../types";

export const api = {
  listChapters: () => invoke<Chapter[]>("list_chapters"),
  listCourseOutcomes: () => invoke<CourseOutcome[]>("list_course_outcomes"),
  listKnowledgePoints: (p: { chapterId?: string }) =>
    invoke<KnowledgePoint[]>("list_knowledge_points", {
      chapterId: p.chapterId ?? null,
    }),

  listTasks: (p: {
    chapterId: string;
    difficulty?: string;
    knowledgePointId?: string;
    offset: number;
    limit: number;
  }) => invoke<TaskPage>("list_tasks", p),

  getTaskDetail: (p: { taskId: string }) =>
    invoke<TaskDetail>("get_task_detail", p),

  getRandomTasks: (p: {
    chapterId?: string;
    difficulty?: string;
    count: number;
    onlyWeak: boolean;
  }) => invoke<TaskDetail[]>("get_random_tasks", p),

  saveAnswer: (p: { taskId: string; chapterId: string; answerText: string }) =>
    invoke<void>("save_answer", p),

  assessRequirement: (p: {
    taskId: string;
    reqIndex: number;
    status: string;
  }) => invoke<void>("assess_requirement", p),

  assessTask: (p: {
    taskId: string;
    chapterId: string;
    assessment: string;
  }) => invoke<void>("assess_task", p),

  getChapterProgress: (p: { chapterId: string }) =>
    invoke<ChapterProgress>("get_chapter_progress", p),

  getOverallProgress: () =>
    invoke<OverallProgress>("get_overall_progress"),

  importTasks: (p: { jsonString: string }) =>
    invoke<ImportResult>("import_tasks", p),

  resetProgress: () => invoke<void>("reset_progress"),
};