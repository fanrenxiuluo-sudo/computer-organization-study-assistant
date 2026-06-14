import { useState, useCallback, useRef, useEffect } from "react";
import { ErrorBoundary } from "./components/ErrorBoundary";
import { Sidebar } from "./components/layout/Sidebar";
import { MetricGrid } from "./components/overview/MetricGrid";
import { TaskCard } from "./components/practice/TaskCard";
import { ChapterProgressPanel } from "./components/overview/ChapterProgress";
import { useChapters } from "./hooks/useChapters";
import { useTasks } from "./hooks/useTasks";
import { useTaskDetail } from "./hooks/useTaskDetail";
import { useStats } from "./hooks/useStats";
import { usePracticeMode } from "./hooks/usePracticeMode";
import { api } from "./services/api";
import type { TaskDetail, Assessment } from "./types";

function App() {
  const workspaceRef = useRef<HTMLElement>(null);
  const { chapters, outcomes, loading: chaptersLoading } = useChapters();
  const { mode, selectedKnowledgePointId, switchMode, selectKnowledgePoint } = usePracticeMode();

  const [activeChapterId, setActiveChapterId] = useState("");
  const [activeTaskIndex, setActiveTaskIndex] = useState(0);

  useEffect(() => {
    if (chapters.length > 0 && !activeChapterId) {
      setActiveChapterId(chapters[0].id);
    }
  }, [chapters, activeChapterId]);

  const { tasks, total, loading: tasksLoading, refresh: refreshTasks } = useTasks(
    activeChapterId,
    0,
    200
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
    updateAnswer,
    saveCurrentAnswer,
    revealReference,
    assessReq,
    assessWholeTask,
  } = useTaskDetail(currentTaskId);

  const { overall, chapter: chapterProgress, refresh: refreshStats } = useStats(activeChapterId);

  const handleSwitchChapter = useCallback(
    (chapterId: string) => {
      saveCurrentAnswer();
      setActiveChapterId(chapterId);
      setActiveTaskIndex(0);
      refreshTasks();
      refreshStats();
      workspaceRef.current?.scrollTo({ top: 0, behavior: "smooth" });
    },
    [saveCurrentAnswer, refreshTasks, refreshStats]
  );

  const handleSwitchMode = useCallback(
    (newMode: typeof mode) => {
      switchMode(newMode);
      setActiveTaskIndex(0);
      refreshTasks();
    },
    [switchMode, refreshTasks]
  );

  const handleAssessTask = useCallback(
    async (assessment: Assessment) => {
      await assessWholeTask(assessment);
      refreshStats();
    },
    [assessWholeTask, refreshStats]
  );

  const currentDetail: TaskDetail | null = detail;

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
        />

        <section className="workspace" ref={workspaceRef}>
          <header className="top-bar">
            <div>
              <p className="eyebrow">v0.1 OBE 任务练习版</p>
              <h2>按课程目标完成综合作答，逐条自评掌握程度</h2>
            </div>
            <div className="exam-chip">期末通过优先</div>
          </header>

          <MetricGrid
            overall={overall}
            chapterCount={chapters.length}
            loading={chaptersLoading}
          />

          <section className="content-grid">
            {currentDetail && !detailLoading ? (
              <TaskCard
                detail={currentDetail}
                currentAnswer={currentAnswer}
                revealed={revealed}
                reqStatuses={reqStatuses}
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
              />
            ) : (
              <div className="empty-block">
                {tasksLoading || detailLoading ? "加载中…" : "当前章节暂无题目，后续会继续补充题库。"}
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

export default App;