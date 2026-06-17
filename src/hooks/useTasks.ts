import { useState, useEffect, useCallback } from "react";
import { api } from "../services/api";
import type { TaskDetail, PracticeMode } from "../types";

export function useTasks(
  chapterId: string,
  practiceMode: PracticeMode,
  selectedKnowledgePointId: string | null,
  offset: number = 0,
  limit: number = 200
) {
  const [tasks, setTasks] = useState<TaskDetail[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchTasks = useCallback(async (signal?: AbortSignal) => {
    if (!chapterId) {
      setTasks([]);
      setTotal(0);
      setLoading(false);
      return;
    }

    setLoading(true);
    setError(null);
    try {
      if (practiceMode === "random") {
        const items = await api.getRandomTasks({
          chapterId,
          count: 10,
          onlyWeak: false,
        });
        if (!signal?.aborted) {
          setTasks(items);
          setTotal(items.length);
        }
      } else if (practiceMode === "weak") {
        const items = await api.getRandomTasks({
          chapterId,
          count: 10,
          onlyWeak: true,
        });
        if (!signal?.aborted) {
          setTasks(items);
          setTotal(items.length);
        }
      } else if (practiceMode === "knowledge-point") {
        if (!selectedKnowledgePointId) {
          setTasks([]);
          setTotal(0);
          setLoading(false);
        } else {
          const page = await api.listTasks({
            chapterId,
            knowledgePointId: selectedKnowledgePointId,
            offset,
            limit,
          });
          if (!signal?.aborted) {
            setTasks(page.items);
            setTotal(page.total);
          }
        }
      } else {
        // sequential mode
        const page = await api.listTasks({ chapterId, offset, limit });
        if (!signal?.aborted) {
          setTasks(page.items);
          setTotal(page.total);
        }
      }
    } catch (e) {
      console.error("加载题目失败", e);
      if (!signal?.aborted) {
        setTasks([]);
        setTotal(0);
        setError("加载题目失败");
      }
    } finally {
      if (!signal?.aborted) {
        setLoading(false);
      }
    }
  }, [chapterId, practiceMode, selectedKnowledgePointId, offset, limit]);

  useEffect(() => {
    const controller = new AbortController();
    fetchTasks(controller.signal);
    return () => controller.abort();
  }, [fetchTasks]);

  const refresh = useCallback(() => {
    fetchTasks();
  }, [fetchTasks]);

  return { tasks, total, loading, error, refresh };
}
