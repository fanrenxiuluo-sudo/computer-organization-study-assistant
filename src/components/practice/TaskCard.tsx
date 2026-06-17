import type { TaskDetail, Assessment, RequirementStatus } from "../../types";

const DIFF_LABELS: Record<string, string> = {
  foundation: "基础",
  applied: "应用",
  advanced: "进阶",
};

const DIFF_CLASSES: Record<string, string> = {
  foundation: "difficulty-foundation",
  applied: "difficulty-applied",
  advanced: "difficulty-advanced",
};

type Props = {
  detail: TaskDetail;
  currentAnswer: string;
  revealed: boolean;
  reqStatuses: Record<string, RequirementStatus>;
  assessing?: boolean;
  onAnswerChange: (value: string) => void;
  onReveal: () => void;
  onAssessReq: (reqIndex: number, status: Assessment) => void;
  onAssessTask: (assessment: Assessment) => void;
  onNext: () => void;
  onPrev: () => void;
  currentIndex: number;
  totalCount: number;
  totalAvailable: number;
};

export function TaskCard({
  detail,
  currentAnswer,
  revealed,
  reqStatuses,
  assessing,
  onAnswerChange,
  onReveal,
  onAssessReq,
  onAssessTask,
  onNext,
  onPrev,
  currentIndex,
  totalCount,
  totalAvailable,
}: Props) {
  const { task, requirements, knowledgePoints, sources } = detail;

  return (
    <article className="question-panel">
      <div className="panel-heading">
        <span className="section-index">
          {String(currentIndex + 1).padStart(2, "0")}
        </span>
        <div>
          <p className="eyebrow">{knowledgePoints.length > 0 ? knowledgePoints.map((kp) => kp.name).join(" · ") : "综合"}</p>
          <h3>{task.scenario}</h3>
        </div>
      </div>

      <div className="tag-row">
        <span className={`difficulty-tag ${DIFF_CLASSES[task.difficulty] || ""}`}>
          {DIFF_LABELS[task.difficulty] || task.difficulty}
        </span>
        {sources.map((src, i) => (
          <span key={i} className="type-tag">
            {src.university}{src.year ? ` ${src.year}` : ""}
          </span>
        ))}
      </div>

      {totalCount > 1 && (
        <div className="task-nav">
          <button className="task-nav-button" type="button" disabled={currentIndex <= 0} onClick={onPrev}>
            ← 上一题
          </button>
          <span className="task-nav-label">
            {currentIndex + 1} / {totalCount}{totalAvailable > totalCount ? `（共 ${totalAvailable} 题）` : ""}
          </span>
          <button className="task-nav-button" type="button" disabled={currentIndex >= totalCount - 1} onClick={onNext}>
            下一题 →
          </button>
        </div>
      )}

      <div className="requirement-block">
        <span>作答要求</span>
        <ul>
          {requirements.map((req) => {
            const key = `${task.id}-${req.reqIndex}`;
            const status = reqStatuses[key]?.status;
            return (
              <li key={req.id} className={`req-item ${status === "mastered" ? "req-mastered" : status === "needs_work" ? "req-needs-work" : ""}`}>
                <span className="req-index">{req.reqIndex + 1}.</span>
                {req.content}
                {status && (
                  <span className="req-status-badge">
                    {status === "mastered" ? " ✓" : " ✗"}
                  </span>
                )}
              </li>
            );
          })}
        </ul>
      </div>

      <label className="answer-editor">
        <span>作答区</span>
        <textarea
          value={currentAnswer}
          onChange={(e) => onAnswerChange(e.target.value)}
          placeholder="在此写下你的作答…"
        />
      </label>

      <div className="action-row">
        <button
          className="primary-button"
          type="button"
          disabled={currentAnswer.trim().length < 5}
          onClick={onReveal}
        >
          对照参考答案
        </button>
        <span className="answer-pending">
          {currentAnswer.trim().length < 5
            ? `已输入 ${currentAnswer.trim().length} 字，至少 5 字后可查看`
            : "可以查看参考答案"}
        </span>
      </div>

      {revealed && (
        <>
          <div className="analysis-block">
            <span>参考答案</span>
            <p>{task.reference}</p>
          </div>

          <div className="self-assess-row">
            <span>逐条评估你的掌握程度</span>
            <div className="req-assess-list">
              {requirements.map((req) => {
                const key = `${task.id}-${req.reqIndex}`;
                const status = reqStatuses[key]?.status;
                return (
                  <div key={req.id} className="req-assess-item">
                    <span className="req-assess-label">
                      要求 {req.reqIndex + 1}: {status === "mastered" ? "已掌握 ✓" : status === "needs_work" ? "需加强 ✗" : "未评"}
                    </span>
                    {status !== "mastered" && (
                      <button
                        className="assess-button assess-mastered"
                        type="button"
                        disabled={assessing}
                        onClick={() => onAssessReq(req.reqIndex, "mastered")}
                      >
                        ✓ 掌握
                      </button>
                    )}
                    {status !== "needs_work" && (
                      <button
                        className="assess-button assess-needs-work"
                        type="button"
                        disabled={assessing}
                        onClick={() => onAssessReq(req.reqIndex, "needs_work")}
                      >
                        ✗ 需加强
                      </button>
                    )}
                  </div>
                );
              })}
            </div>
          </div>

          <div className="self-assess-row">
            <span>整体评价</span>
            <div className="self-assess-buttons">
              <button
                className="assess-button assess-mastered"
                type="button"
                disabled={assessing}
                onClick={() => onAssessTask("mastered")}
              >
                ✓ 整体已掌握
              </button>
              <button
                className="assess-button assess-needs-work"
                type="button"
                disabled={assessing}
                onClick={() => onAssessTask("needs_work")}
              >
                ✗ 整体需加强
              </button>
            </div>
          </div>

          {sources.length > 0 && (
            <div className="source-block">
              <span>题目来源</span>
              {sources.map((src, i) => (
                <p key={i}>
                  {src.university}{src.year ? ` ${src.year}年` : ""} · {src.examType === "final" ? "期末" : src.examType === "postgraduate" ? "考研" : src.examType === "obe" ? "OBE" : "改编"}
                  {src.originalText && (
                    <><br /><em className="source-original">原题：{src.originalText}</em></>
                  )}
                  {src.note && (
                    <><br /><em className="source-note">改编说明：{src.note}</em></>
                  )}
                </p>
              ))}
            </div>
          )}
        </>
      )}
    </article>
  );
}