import { api } from "./api";
import type { Chapter, KnowledgePoint } from "../types";

const CACHE_TTL = 5 * 60 * 1000;

interface CacheEntry<T> {
  data: T;
  timestamp: number;
}

let chaptersCache: CacheEntry<Chapter[]> | null = null;
const knowledgePointsCache = new Map<string, CacheEntry<KnowledgePoint[]>>();

function isExpired<T>(entry: CacheEntry<T> | null): boolean {
  return !entry || Date.now() - entry.timestamp > CACHE_TTL;
}

export async function getChapters(): Promise<Chapter[]> {
  if (isExpired(chaptersCache)) {
    const data = await api.listChapters();
    chaptersCache = { data, timestamp: Date.now() };
  }
  return chaptersCache!.data;
}

export async function getKnowledgePoints(chapterId: string): Promise<KnowledgePoint[]> {
  if (isExpired(knowledgePointsCache.get(chapterId) ?? null)) {
    const kps = await api.listKnowledgePoints({ chapterId });
    knowledgePointsCache.set(chapterId, { data: kps, timestamp: Date.now() });
  }
  return knowledgePointsCache.get(chapterId)!.data;
}

export async function getAllKnowledgePoints(): Promise<KnowledgePoint[]> {
  const all = await api.listKnowledgePoints({});
  return all;
}

export function invalidateCache(): void {
  chaptersCache = null;
  knowledgePointsCache.clear();
}