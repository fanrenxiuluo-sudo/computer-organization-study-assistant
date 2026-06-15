import { api } from "./api";
import type { Chapter, KnowledgePoint } from "../types";

let chaptersCache: Chapter[] | null = null;
let knowledgePointsCache = new Map<string, KnowledgePoint[]>();

export async function getChapters(): Promise<Chapter[]> {
  if (!chaptersCache) {
    chaptersCache = await api.listChapters();
  }
  return chaptersCache;
}

export async function getKnowledgePoints(chapterId: string): Promise<KnowledgePoint[]> {
  if (!knowledgePointsCache.has(chapterId)) {
    const kps = await api.listKnowledgePoints({ chapterId });
    knowledgePointsCache.set(chapterId, kps);
  }
  return knowledgePointsCache.get(chapterId)!;
}

export async function getAllKnowledgePoints(): Promise<KnowledgePoint[]> {
  const all = await api.listKnowledgePoints({});
  return all;
}

export function invalidateCache(): void {
  chaptersCache = null;
  knowledgePointsCache.clear();
}