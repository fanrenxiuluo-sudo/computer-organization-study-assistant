import { useState, useCallback, useRef, useEffect } from "react";
import { ErrorBoundary } from "./components/ErrorBoundary";
import { Sidebar } from "./components/layout/Sidebar";
import { MetricGrid } from "./components/overview/MetricGrid";
import { TaskCard } from "./components/practice/TaskCard";
import { ChapterProgressPanel } from "./components/overview/ChapterProgress";
import { useTasks } from "./hooks/useTasks";
import { useTaskDetail } from "./hooks/useTaskDetail";
import { useStats } from "./hooks/useStats";
import { api } from "./services/api";
import { AppProvider, useAppContext } from "./contexts/AppContext";
import type { TaskDetail, Assessment } from "./types";

function AppContent() {
  const {
    activeChapterId, setActiveChapterId,
    practiceMode: mode, switchMode, selectKnowledgePoint, selectedKnowledgePointId,
    chapters, outcomes, chaptersError,
    overall, overallLoading, overallError, refreshOverall,
  } = useAppContext();

  const workspaceRef = useRef<HTMLElement>(null);
  const [activeTaskIndex, setActiveTaskIndex] = useState(0);

  const { tasks, total, loading: tasksLoading, error: tasksError } = useTasks(
    activeChapterId,
    mode,
    selectedKnowledgePointId,
    0,
    500
  );

  const currentTaskId: string | null =
    tasks.length > 0 && activeTaskIndex < tasks.length
      ? tasks[activeTaskIndex].task.id
      : null;

  const {
    detail,
    loading: detailLoading,
    currentAnswer,
    revealed,
    reqStatuses,
    assessing,
    updateAnswer,
    saveCurrentAnswer,
    revealReference,
    assessReq,
    assessWholeTask,
  } = useTaskDetail(currentTaskId);

  const { chapter: chapterProgress, refresh: refreshStats } = useStats(activeChapterId);

  const handleSwitchChapter = useCallback(
    async (chapterId: string) => {
      await saveCurrentAnswer();
      setActiveChapterId(chapterId);
      setActiveTaskIndex(0);
      if (mode === "knowledge-point") {
        switchMode("sequential");
      }
      workspaceRef.current?.scrollTo({ top: 0, behavior: "smooth" });
    },
    [saveCurrentAnswer, setActiveChapterId, mode, switchMode]
  );

  const handleSwitchMode = useCallback(
    (newMode: typeof mode) => {
      switchMode(newMode);
      setActiveTaskIndex(0);
    },
    [switchMode]
  );

  const handleAssessTask = useCallback(
    async (assessment: Assessment) => {
      await assessWholeTask(assessment);
      refreshOverall();
      refreshStats(activeChapterId);
    },
    [assessWholeTask, refreshOverall, refreshStats, activeChapterId]
  );

  const currentDetail: TaskDetail | null = detail;

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLTextAreaElement || e.target instanceof HTMLInputElement) return;
      if (e.key === "ArrowLeft") {
        saveCurrentAnswer();
        setActiveTaskIndex((i) => Math.max(i - 1, 0));
      } else if (e.key === "ArrowRight") {
        saveCurrentAnswer();
        setActiveTaskIndex((i) => Math.min(i + 1, tasks.length - 1));
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [saveCurrentAnswer, tasks.length]);

  return (
    <ErrorBoundary>
      <main className="app-shell">
        <Sidebar
          chapters={chapters}
          outcomes={outcomes}
          activeChapterId={activeChapterId}
          practiceMode={mode}
          overallMastered={overall?.mastered ?? 0}
          overallTotal={overall?.totalTasks ?? 0}
          outcomeProgress={overall?.outcomes ?? []}
          onSwitchChapter={handleSwitchChapter}
          onSwitchMode={handleSwitchMode}
          onResetProgress={async () => {
            await api.resetProgress();
            refreshOverall();
            refreshStats(activeChapterId);
          }}
        />
        {chaptersError && <div className="error-banner">{chaptersError}</div>}

        <section className="workspace" ref={workspaceRef}>
          <header className="top-bar">
            <div>
              <p className="eyebrow">v{__APP_VERSION__} 生产版</p>
              <h2>按课程目标完成综合作答，逐条自评掌握程度</h2>
            </div>
            <div className="exam-chip">期末通过优先</div>
          </header>

          <MetricGrid
            overall={overall}
            chapterCount={chapters.length}
            loading={overallLoading}
            error={overallError}
          />

          <section className="content-grid">
            {currentDetail && !detailLoading ? (
              <TaskCard
                detail={currentDetail}
                currentAnswer={currentAnswer}
                revealed={revealed}
                reqStatuses={reqStatuses}
                assessing={assessing}
                onAnswerChange={updateAnswer}
                onReveal={revealReference}
                onAssessReq={assessReq}
                onAssessTask={handleAssessTask}
                onNext={() => {
                  saveCurrentAnswer();
                  setActiveTaskIndex((i) => Math.min(i + 1, tasks.length - 1));
                }}
                onPrev={() => {
                  saveCurrentAnswer();
                  setActiveTaskIndex((i) => Math.max(i - 1, 0));
                }}
                currentIndex={activeTaskIndex}
                totalCount={tasks.length}
                totalAvailable={total}
              />
            ) : tasksError ? (
              <div className="empty-block error">{tasksError}</div>
            ) : (
              <div className="empty-block">
                {tasksLoading || detailLoading ? "加载中…" :
                  mode === "weak" ? '当前章节暂无薄弱题目，请先完成一些练习并标记"需加强"。' :
                  mode === "random" ? "当前章节暂无题目，后续会继续补充题库。" :
                  mode === "knowledge-point" ? "请先在学习路线面板选择一个考点。" :
                  "当前章节暂无题目，后续会继续补充题库。"}
              </div>
            )}

            <ChapterProgressPanel
              chapters={chapters}
              activeChapterId={activeChapterId}
              chapterProgress={chapterProgress}
              onSelectKnowledgePoint={(kpId: string) => {
                selectKnowledgePoint(kpId);
              }}
            />
          </section>

          <button
            className="back-top-button"
            type="button"
            onClick={() => workspaceRef.current?.scrollTo({ top: 0, behavior: "smooth" })}
          >
            回到顶部
          </button>
        </section>
      </main>
    </ErrorBoundary>
  );
}

function App() {
  return (
    <AppProvider>
      <AppContent />
    </AppProvider>
  );
}

export default App;