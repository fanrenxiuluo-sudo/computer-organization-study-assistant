import type { OverallProgress } from "../../types";

type Props = {
  overall: OverallProgress | null;
  chapterCount: number;
  loading: boolean;
  error?: string | null;
};

export function MetricGrid({ overall, chapterCount, loading, error }: Props) {
  if (error) {
    return (
      <section className="metric-grid" aria-label="项目状态">
        <div className="metric-card metric-card--error">
          <span>{error}</span>
        </div>
      </section>
    );
  }

  if (loading || !overall) {
    return (
      <section className="metric-grid" aria-label="项目状态">
        <div className="metric-card">
          <span>加载中…</span>
        </div>
      </section>
    );
  }

  return (
    <section className="metric-grid" aria-label="项目状态">
      <div className="metric-card">
        <span>章节数</span>
        <strong>{chapterCount}</strong>
      </div>
      <div className="metric-card">
        <span>总题数</span>
        <strong>{overall.totalTasks}</strong>
      </div>
      <div className="metric-card metric-card--accent">
        <span>已掌握</span>
        <strong>{overall.mastered}</strong>
      </div>
      <div className="metric-card">
        <span>需加强</span>
        <strong>{overall.needsWork}</strong>
      </div>
      <div className="metric-card">
        <span>当前版本</span>
        <strong>v0.2</strong>
      </div>
    </section>
  );
}