import { createContext, useContext, useState, useCallback, useRef, useEffect, type ReactNode } from "react";
import { api } from "../services/api";
import type { Chapter, CourseOutcome, PracticeMode, OverallProgress } from "../types";

interface AppContextValue {
  activeChapterId: string;
  setActiveChapterId: (id: string) => void;
  practiceMode: PracticeMode;
  switchMode: (mode: PracticeMode) => void;
  selectedKnowledgePointId: string | null;
  selectKnowledgePoint: (kpId: string) => void;
  chapters: Chapter[];
  outcomes: CourseOutcome[];
  chaptersLoading: boolean;
  chaptersError: string | null;
  overall: OverallProgress | null;
  overallLoading: boolean;
  overallError: string | null;
  refreshOverall: () => void;
}

const AppContext = createContext<AppContextValue | null>(null);

const STORAGE_KEY_CHAPTER = "jizubeikao_activeChapterId";

export function AppProvider({ children }: { children: ReactNode }) {
  const [activeChapterId, setActiveChapterIdState] = useState(() => {
    try {
      const stored = localStorage.getItem(STORAGE_KEY_CHAPTER);
      return stored ?? "";
    } catch {
      return "";
    }
  });
  const [practiceMode, setPracticeMode] = useState<PracticeMode>("sequential");
  const [selectedKnowledgePointId, setSelectedKnowledgePointId] = useState<string | null>(null);
  const [chapters, setChapters] = useState<Chapter[]>([]);
  const [outcomes, setOutcomes] = useState<CourseOutcome[]>([]);
  const [chaptersLoading, setChaptersLoading] = useState(true);
  const [chaptersError, setChaptersError] = useState<string | null>(null);
  const [overall, setOverall] = useState<OverallProgress | null>(null);
  const [overallLoading, setOverallLoading] = useState(true);
  const [overallError, setOverallError] = useState<string | null>(null);

  const chaptersFetchedRef = useRef(false);

  useEffect(() => {
    if (chaptersFetchedRef.current) return;
    chaptersFetchedRef.current = true;
    let cancelled = false;
    setChaptersError(null);
    Promise.all([api.listChapters(), api.listCourseOutcomes()])
      .then(([chs, cos]) => {
        if (!cancelled) {
          setChapters(chs);
          setOutcomes(cos);
          if (chs.length > 0 && !activeChapterId) {
            setActiveChapterId(chs[0].id);
          }
        }
      })
      .catch((e) => {
        if (!cancelled) {
          setChapters([]);
          setOutcomes([]);
          setChaptersError("加载章节失败");
          console.error("加载章节失败", e);
        }
      })
      .finally(() => {
        if (!cancelled) setChaptersLoading(false);
      });
    return () => { cancelled = true; };
  }, []);

  const refreshOverall = useCallback(() => {
    setOverallLoading(true);
    setOverallError(null);
    api.getOverallProgress()
      .then((o) => { setOverall(o); })
      .catch((e) => {
        setOverall(null);
        setOverallError("加载统计数据失败");
        console.error("加载统计失败", e);
      })
      .finally(() => { setOverallLoading(false); });
  }, []);

  useEffect(() => {
    api.getOverallProgress()
      .then((o) => { setOverall(o); })
      .catch((e) => {
        setOverall(null);
        setOverallError("加载统计数据失败");
        console.error("加载统计失败", e);
      })
      .finally(() => { setOverallLoading(false); });
  }, []);

  const setActiveChapterId = useCallback((id: string) => {
    setActiveChapterIdState(id);
    try { localStorage.setItem(STORAGE_KEY_CHAPTER, id); } catch {}
  }, []);

  const switchMode = useCallback((newMode: PracticeMode) => {
    setPracticeMode(newMode);
    setSelectedKnowledgePointId(null);
  }, []);

  const selectKnowledgePoint = useCallback((kpId: string) => {
    setPracticeMode("knowledge-point");
    setSelectedKnowledgePointId(kpId);
  }, []);

  return (
    <AppContext.Provider value={{
      activeChapterId,
      setActiveChapterId,
      practiceMode,
      switchMode,
      selectedKnowledgePointId,
      selectKnowledgePoint,
      chapters,
      outcomes,
      chaptersLoading,
      chaptersError,
      overall,
      overallLoading,
      overallError,
      refreshOverall,
    }}>
      {children}
    </AppContext.Provider>
  );
}

export function useAppContext() {
  const ctx = useContext(AppContext);
  if (!ctx) throw new Error("useAppContext must be used within AppProvider");
  return ctx;
}