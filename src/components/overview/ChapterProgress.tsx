import type { Chapter, ChapterProgress, KnowledgePoint } from "../../types";

type Props = {
  chapters: Chapter[];
  activeChapterId: string;
  chapterProgress: ChapterProgress | null;
  onSelectKnowledgePoint: (kpId: string) => void;
};

const DIFF_LABELS: Record<string, string> = {
  foundation: "基础",
  applied: "应用",
  advanced: "进阶",
};

export function ChapterProgressPanel({
  chapters,
  activeChapterId,
  chapterProgress,
  onSelectKnowledgePoint,
}: Props) {
  const activeChapter = chapters.find((c) => c.id === activeChapterId);

  return (
    <article className="roadmap-panel">
      <p className="eyebrow">学习进度</p>
      <h3>{activeChapter?.title ?? "章节"}掌握情况</h3>

      {chapterProgress && (
        <div className="progress-summary-row">
          <span>总题数: {chapterProgress.totalTasks}</span>
          <span>已掌握: {chapterProgress.mastered}</span>
          <span>需加强: {chapterProgress.needsWork}</span>
        </div>
      )}

      {chapterProgress && chapterProgress.knowledgePoints.length > 0 && (
        <div className="kp-progress-list">
          <p className="eyebrow" style={{ marginTop: 16 }}>考点掌握度</p>
          {chapterProgress.knowledgePoints.map((kp) => (
            <button
              key={kp.knowledgePointId}
              className="kp-progress-item"
              type="button"
              onClick={() => onSelectKnowledgePoint(kp.knowledgePointId)}
            >
              <span className="kp-name">{kp.name}</span>
              <div className="kp-bar-track">
                <div
                  className="kp-bar-fill"
                  style={{ width: `${Math.min(kp.masteryPercent, 100)}%` }}
                />
              </div>
              <span className="kp-pct">{Math.round(kp.masteryPercent)}%</span>
            </button>
          ))}
        </div>
      )}

      <div className="roadmap-list" style={{ marginTop: 16 }}>
        {chapters.map((chapter) => {
          const pct = chapterProgress && chapter.id === activeChapterId
            ? (chapterProgress.totalTasks > 0
              ? (chapterProgress.mastered / chapterProgress.totalTasks) * 100
              : 0)
            : 0;
          return (
            <div className="roadmap-item" key={chapter.id}>
              <div className="roadmap-row">
                <span>
                  {chapter.orderIndex.toString().padStart(2, "0")} {chapter.title}
                </span>
                <em>
                  {chapter.id === activeChapterId && chapterProgress
                    ? `${chapterProgress.mastered}/${chapterProgress.totalTasks}`
                    : ""}
                </em>
              </div>
              {chapter.id === activeChapterId && chapterProgress && (
                <div className="progress-track">
                  <div style={{ width: `${pct}%` }} />
                </div>
              )}
            </div>
          );
        })}
      </div>
    </article>
  );
}