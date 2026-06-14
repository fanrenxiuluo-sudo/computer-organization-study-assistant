import { useState, useEffect, useCallback } from "react";
import { api } from "../services/api";
import type { TaskDetail } from "../types";

export function useTasks(
  chapterId: string,
  offset: number = 0,
  limit: number = 20
) {
  const [tasks, setTasks] = useState<TaskDetail[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    api.listTasks({ chapterId, offset, limit }).then((page) => {
      if (!cancelled) {
        setTasks(page.items);
        setTotal(page.total);
        setLoading(false);
      }
    });
    return () => { cancelled = true; };
  }, [chapterId, offset, limit]);

  const refresh = useCallback(() => {
    api.listTasks({ chapterId, offset, limit }).then((page) => {
      setTasks(page.items);
      setTotal(page.total);
    });
  }, [chapterId, offset, limit]);

  return { tasks, total, loading, refresh };
}