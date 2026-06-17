import type { Chapter, CourseOutcome, PracticeMode } from "../../types";

type Props = {
  chapters: Chapter[];
  outcomes: CourseOutcome[];
  activeChapterId: string;
  practiceMode: PracticeMode;
  overallMastered: number;
  overallTotal: number;
  outcomeProgress: { outcomeId: string; code: string; description: string; masteryPercent: number }[];
  onSwitchChapter: (chapterId: string) => void;
  onSwitchMode: (mode: PracticeMode) => void;
  onResetProgress: () => void;
};

const MODES: { value: PracticeMode; label: string }[] = [
  { value: "sequential", label: "顺序练习" },
  { value: "weak", label: "薄弱专练" },
  { value: "random", label: "随机抽查" },
  { value: "knowledge-point", label: "考点定向" },
];

export function Sidebar({
  chapters,
  outcomes,
  activeChapterId,
  practiceMode,
  overallMastered,
  overallTotal,
  outcomeProgress,
  onSwitchChapter,
  onSwitchMode,
  onResetProgress,
}: Props) {
  return (
    <aside className="side-panel">
      <div className="brand-block">
        <span className="brand-mark">CO</span>
        <div>
          <p className="eyebrow">OBE 课程目标</p>
          <h1>计组备考助手</h1>
        </div>
      </div>

      <section className="mode-section">
        <p className="eyebrow">练习模式</p>
        <div className="mode-list">
          {MODES.map((m) => (
            <button
              key={m.value}
              className={`mode-button ${practiceMode === m.value ? "is-active" : ""}`}
              type="button"
              onClick={() => onSwitchMode(m.value)}
            >
              {m.label}
            </button>
          ))}
        </div>
      </section>

      <nav className="chapter-list" aria-label="章节列表">
        <p className="eyebrow">章节</p>
        {chapters.map((ch) => (
          <button
            key={ch.id}
            className={`chapter-button ${ch.id === activeChapterId ? "is-active" : ""}`}
            type="button"
            onClick={() => onSwitchChapter(ch.id)}
          >
            <span>{ch.orderIndex.toString().padStart(2, "0")}</span>
            {ch.title}
          </button>
        ))}
      </nav>

      <section className="outcome-section">
        <p className="eyebrow">课程目标达成</p>
        <div className="outcome-list">
          {outcomeProgress.map((op) => {
            const outcome = outcomes.find((item) => item.id === op.outcomeId);
            return (
              <div
                key={op.outcomeId}
                className="outcome-row"
                title={outcome?.description ?? op.description}
              >
                <span className="outcome-code">{op.code}</span>
                <div className="outcome-bar-track">
                  <div
                    className="outcome-bar-fill"
                    style={{ width: `${Math.min(op.masteryPercent, 100)}%` }}
                  />
                </div>
                <span className="outcome-pct">{Math.round(op.masteryPercent)}%</span>
              </div>
            );
          })}
        </div>
      </section>

      <div className="progress-summary">
        总掌握 {overallMastered} / {overallTotal}
      </div>

      <button
        className="reset-button"
        type="button"
        onClick={() => {
          if (window.confirm("确定要重置所有学习进度吗？此操作不可撤销。")) {
            onResetProgress();
          }
        }}
      >
        重置学习进度
      </button>
    </aside>
  );
}