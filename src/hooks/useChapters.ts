import { useState, useEffect } from "react";
import { api } from "../services/api";
import { getChapters } from "../services/cache";
import type { Chapter, CourseOutcome } from "../types";

export function useChapters() {
  const [chapters, setChapters] = useState<Chapter[]>([]);
  const [outcomes, setOutcomes] = useState<CourseOutcome[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    setError(null);
    Promise.all([getChapters(), api.listCourseOutcomes()])
      .then(([chs, cos]) => {
        if (!cancelled) {
          setChapters(chs);
          setOutcomes(cos);
        }
      })
      .catch((e) => {
        console.error("加载章节失败", e);
        if (!cancelled) {
          setChapters([]);
          setOutcomes([]);
          setError("加载章节失败");
        }
      })
      .finally(() => {
        if (!cancelled) {
          setLoading(false);
        }
      });
    return () => { cancelled = true; };
  }, []);

  return { chapters, outcomes, loading, error };
}