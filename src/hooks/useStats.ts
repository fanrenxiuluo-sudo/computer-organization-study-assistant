import { useState, useEffect, useCallback } from "react";
import { api } from "../services/api";
import type { OverallProgress, ChapterProgress } from "../types";

export function useStats(chapterId?: string) {
  const [overall, setOverall] = useState<OverallProgress | null>(null);
  const [chapter, setChapter] = useState<ChapterProgress | null>(null);
  const [loading, setLoading] = useState(true);

  const refresh = useCallback(async () => {
    setLoading(true);
    try {
      const o = await api.getOverallProgress();
      setOverall(o);
      if (chapterId) {
        const c = await api.getChapterProgress({ chapterId });
        setChapter(c);
      }
    } finally {
      setLoading(false);
    }
  }, [chapterId]);

  useEffect(() => {
    refresh();
  }, [refresh]);

  return { overall, chapter, loading, refresh };
}