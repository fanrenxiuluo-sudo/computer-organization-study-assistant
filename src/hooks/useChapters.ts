import { useState, useEffect } from "react";
import { api } from "../services/api";
import { getChapters } from "../services/cache";
import type { Chapter, CourseOutcome } from "../types";

export function useChapters() {
  const [chapters, setChapters] = useState<Chapter[]>([]);
  const [outcomes, setOutcomes] = useState<CourseOutcome[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;
    Promise.all([getChapters(), api.listCourseOutcomes()]).then(([chs, cos]) => {
      if (!cancelled) {
        setChapters(chs);
        setOutcomes(cos);
        setLoading(false);
      }
    });
    return () => { cancelled = true; };
  }, []);

  return { chapters, outcomes, loading };
}