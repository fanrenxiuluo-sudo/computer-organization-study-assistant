import { useState, useEffect, useCallback } from "react";
import { api } from "../services/api";
import type { OverallProgress, ChapterProgress } from "../types";

export function useStats(chapterId?: string) {
  const [overall, setOverall] = useState<OverallProgress | null>(null);
  const [chapter, setChapter] = useState<ChapterProgress | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async (targetChapterId?: string) => {
    setLoading(true);
    setError(null);
    try {
      const o = await api.getOverallProgress();
      setOverall(o);
      const cid = targetChapterId ?? chapterId;
      if (cid) {
        const c = await api.getChapterProgress({ chapterId: cid });
        setChapter(c);
      } else {
        setChapter(null);
      }
    } catch (e) {
      console.error("加载统计失败", e);
      setOverall(null);
      setChapter(null);
      setError("加载统计数据失败");
    } finally {
      setLoading(false);
    }
  }, [chapterId]);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(null);

    (async () => {
      try {
        const o = await api.getOverallProgress();
        if (cancelled) return;
        setOverall(o);

        if (chapterId) {
          const c = await api.getChapterProgress({ chapterId });
          if (cancelled) return;
          setChapter(c);
        } else {
          setChapter(null);
        }
      } catch (e) {
        if (cancelled) return;
        console.error("加载统计失败", e);
        setOverall(null);
        setChapter(null);
        setError("加载统计数据失败");
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();

    return () => { cancelled = true; };
  }, [chapterId]);

  return { overall, chapter, loading, error, refresh };
}