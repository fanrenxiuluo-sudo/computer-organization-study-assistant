import { useMemo, useRef, useState } from "react";
import seedData from "../data/seed_questions.json";
import type { SeedData } from "./types";

const data = seedData as SeedData;

const roadmap = [
  { label: "OBE 任务", status: "当前阶段", percent: 35 },
  { label: "作答复盘", status: "下一阶段", percent: 0 },
  { label: "学习统计", status: "规划中", percent: 0 },
  { label: "模拟考核", status: "规划中", percent: 0 }
];

function App() {
  const workspaceRef = useRef<HTMLElement>(null);
  const [activeChapterId, setActiveChapterId] = useState(data.chapters[0].id);
  const [studentAnswer, setStudentAnswer] = useState("");
  const [showReference, setShowReference] = useState(false);

  const activeChapter = data.chapters.find((chapter) => chapter.id === activeChapterId);
  const chapterTasks = useMemo(
    () => data.tasks.filter((task) => task.chapterId === activeChapterId),
    [activeChapterId]
  );
  const currentTask = chapterTasks[0];

  const switchChapter = (chapterId: string) => {
    setActiveChapterId(chapterId);
    setStudentAnswer("");
    setShowReference(false);
    workspaceRef.current?.scrollTo({ top: 0, behavior: "smooth" });
  };

  return (
    <main className="app-shell">
      <aside className="side-panel">
        <div className="brand-block">
          <span className="brand-mark">CO</span>
          <div>
            <p className="eyebrow">Network Engineering Project</p>
            <h1>计组备考助手</h1>
          </div>
        </div>

        <nav className="chapter-list" aria-label="章节列表">
          {data.chapters.map((chapter) => (
            <button
              key={chapter.id}
              className={`chapter-button ${chapter.id === activeChapterId ? "is-active" : ""}`}
              type="button"
              onClick={() => switchChapter(chapter.id)}
            >
              <span>{chapter.orderIndex.toString().padStart(2, "0")}</span>
              {chapter.title}
            </button>
          ))}
        </nav>
      </aside>

      <section className="workspace" ref={workspaceRef}>
        <header className="top-bar">
          <div>
            <p className="eyebrow">v0.1 OBE 任务练习版</p>
            <h2>按课程目标完成综合作答，再扩展复盘和统计</h2>
          </div>
          <div className="exam-chip">期末通过优先</div>
        </header>

        <section className="metric-grid" aria-label="项目状态">
          <div className="metric-card">
            <span>章节数</span>
            <strong>{data.chapters.length}</strong>
          </div>
          <div className="metric-card">
            <span>OBE 任务</span>
            <strong>{data.tasks.length}</strong>
          </div>
          <div className="metric-card">
            <span>当前目标</span>
            <strong>v0.1</strong>
          </div>
        </section>

        <section className="content-grid">
          <article className="question-panel">
            <div className="panel-heading">
              <span className="section-index">01</span>
              <div>
                <p className="eyebrow">{activeChapter?.title ?? "章节"}</p>
                <h3>{currentTask?.scenario ?? "本章节 OBE 综合任务正在补充"}</h3>
              </div>
            </div>

            {currentTask ? (
              <>
                <div className="outcome-block">
                  <span>课程目标</span>
                  <p>{currentTask.outcome}</p>
                </div>

                <div className="requirement-block">
                  <span>作答要求</span>
                  <ul>
                    {currentTask.requirements.map((requirement) => (
                      <li key={requirement}>{requirement}</li>
                    ))}
                  </ul>
                </div>

                <label className="answer-editor">
                  <span>我的作答</span>
                  <textarea
                    value={studentAnswer}
                    onWheel={(event) => {
                      const workspace = workspaceRef.current;
                      if (!workspace) return;
                      workspace.scrollTop += event.deltaY;
                    }}
                    onChange={(event) => {
                      setStudentAnswer(event.target.value);
                      setShowReference(false);
                    }}
                    placeholder="按作答要求分点写出你的分析过程。"
                  />
                </label>

                <div className="action-row">
                  <button
                    className="primary-button"
                    type="button"
                    disabled={studentAnswer.trim().length < 10}
                    onClick={() => setShowReference(true)}
                  >
                    对照参考答案
                  </button>
                  <span className="answer-pending">
                    已输入 {studentAnswer.trim().length} 字，至少输入 10 字后可查看参考答案
                  </span>
                </div>

                {showReference ? (
                  <div className="analysis-block">
                    <span>参考答案</span>
                    <p>{currentTask.reference}</p>
                  </div>
                ) : null}
              </>
            ) : (
              <div className="empty-block">当前章节没有题目，后续步骤会继续补充题库。</div>
            )}
          </article>

          <article className="roadmap-panel">
            <p className="eyebrow">开发进度</p>
            <h3>桌面学习系统路线</h3>
            <div className="roadmap-list">
              {roadmap.map((item) => (
                <div className="roadmap-item" key={item.label}>
                  <div className="roadmap-row">
                    <span>{item.label}</span>
                    <em>{item.status}</em>
                  </div>
                  <div className="progress-track">
                    <div style={{ width: `${item.percent}%` }} />
                  </div>
                </div>
              ))}
            </div>
          </article>
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
  );
}

export default App;
