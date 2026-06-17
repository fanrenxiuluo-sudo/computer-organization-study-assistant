# 修复与解决方案

基于项目审查报告，按优先级从高到低排列，共分 6 个阶段。

---

## 阶段一：紧急 Bug 修复（阻塞级）

### 步骤 1.1：修复前端 API 命名与后端不匹配

**问题**：`api.ts` 调用 `invoke("import_seed_data")`，后端注册的是 `import_tasks`，调用直接报错。

**文件**：`src/services/api.ts:31`

**修改**：
```ts
// 修改前
importSeedData: (p: { jsonData: string }) =>
  invoke<ImportResult>("import_seed_data", p),

// 修改后
importSeedData: (p: { jsonData: string }) =>
  invoke<ImportResult>("import_tasks", p),
```

**验证**：在 Tauri 开发环境中调用 `api.importSeedData()` 不再报 Command not found。

---

### 步骤 1.2：修复 list_tasks 后端 limit 硬限与前端请求不一致

**问题**：后端 `limit.clamp(1, 50)` 硬限 50 条，前端请求 200 条，导致顺序模式下最多只能看到 50 题，但 `total` 显示真实总数。

**文件**：`src-tauri/src/commands/task.rs:15`

**修改**：
```rust
// 修改前
let limit = limit.clamp(1, 50);

// 修改后
let limit = limit.clamp(1, 500);
```

**验证**：顺序模式下能导航到所有题目。

---

### 步骤 1.3：修复 saveCurrentAnswer 未 await 导致作答丢失

**问题**：`handleSwitchChapter` 中 `saveCurrentAnswer()` 没有 await，切换章节时保存请求可能未发出。

**文件**：`src/App.tsx:59`

**修改**：
```tsx
// 修改前
const handleSwitchChapter = useCallback(
  (chapterId: string) => {
    saveCurrentAnswer();
    setActiveChapterId(chapterId);
    setActiveTaskIndex(0);
    workspaceRef.current?.scrollTo({ top: 0, behavior: "smooth" });
  },
  [saveCurrentAnswer]
);

// 修改后
const handleSwitchChapter = useCallback(
  async (chapterId: string) => {
    await saveCurrentAnswer();
    setActiveChapterId(chapterId);
    setActiveTaskIndex(0);
    workspaceRef.current?.scrollTo({ top: 0, behavior: "smooth" });
  },
  [saveCurrentAnswer]
);
```

同步修改 `Sidebar.tsx` 的 `onSwitchChapter` prop 类型：
```tsx
onSwitchChapter: (chapterId: string) => void;  // 改为兼容 async
```

> 注：`void` 返回类型兼容 `Promise<void>`，调用方无需改动。但需确认 Sidebar 中点击回调不依赖返回值。

**验证**：切换章节后在数据库中确认上一题答案已保存。

---

### 步骤 1.4：修复考点专练模式 selectedKnowledgePointId 为 null 时降级为顺序模式

**问题**：UI 显示"考点专练"已激活，但 `knowledgePointId` 为 null 时走 else 分支加载顺序题目。

**文件**：`src/hooks/useTasks.ts:31-36`

**修改**：
```ts
// 修改前
} else if (practiceMode === "knowledge-point" && selectedKnowledgePointId) {
  // ...
} else {
  // 顺序模式
}

// 修改后
} else if (practiceMode === "knowledge-point") {
  if (!selectedKnowledgePointId) {
    setTasks([]);
    setTotal(0);
    setLoading(false);
  } else {
    const page = await api.listTasks({
      chapterId,
      knowledgePointId: selectedKnowledgePointId,
      offset,
      limit,
    });
    if (!signal?.aborted) {
      setTasks(page.items);
      setTotal(page.total);
    }
  }
} else {
  // 顺序模式
}
```

**同步修改** `src/App.tsx` 空状态文案：
```tsx
mode === "knowledge-point" ? "请先在学习路线面板选择一个考点。" :
```

**验证**：考点专练模式下未选考点时显示空状态提示而非错误题目。

---

## 阶段二：安装体验修复（桌面文件夹问题）

### 步骤 2.1：修改 NSIS 安装配置，桌面只创建快捷方式

**问题**：安装后在桌面创建项目文件夹而非单个快捷方式图标。

**文件**：`src-tauri/tauri.conf.json`

**修改**：
```json
"windows": {
  "nsis": {
    "installMode": "currentUser",
    "displayLanguageSelector": false,
    "startMenuFolder": "计组备考助手",
    "allowToChangeInstallationDirectory": true,
    "deleteAppDataOnUninstall": false,
    "installerHooks": "installer.nsi"
  }
}
```

### 步骤 2.2：创建 NSIS 自定义 Hook 脚本

**文件**：新建 `src-tauri/installer.nsi`

```nsi
!macro customInstall
  ; 只在桌面创建单个快捷方式，不创建文件夹
  CreateShortCut "$DESKTOP\计组备考助手.lnk" "$INSTDIR\计组备考助手.exe" "" "$INSTDIR\计组备考助手.exe" 0
!macroend

!macro customUnInstall
  ; 卸载时删除桌面快捷方式
  Delete "$DESKTOP\计组备考助手.lnk"
!macroend

!macro customRemoveFiles
  ; 不自动删除用户数据目录
!macroend
```

### 步骤 2.3：确保应用数据目录不在桌面

**文件**：`src-tauri/src/lib.rs`

现有 `ensure_not_on_desktop` 已做检查，需增加日志提示：

```rust
fn ensure_not_on_desktop(app: &tauri::App, path: &Path) -> Result<(), String> {
    if let Ok(desktop_dir) = app.path().desktop_dir() {
        if path.starts_with(&desktop_dir) {
            eprintln!(
                "[严重] 数据目录 {} 位于桌面 {} 下，已拒绝启动",
                path.display(),
                desktop_dir.display()
            );
            return Err(format!(
                "应用数据目录异常：{} 位于桌面目录 {} 下，已拒绝启动以避免污染桌面。",
                path.display(),
                desktop_dir.display()
            ));
        }
    }
    Ok(())
}
```

### 步骤 2.4：增强旧版桌面数据迁移（完成后自动提示清理）

**文件**：`src-tauri/src/lib.rs`

```rust
fn migrate_legacy_desktop_db(app: &tauri::App, study_data_dir: &Path) -> Result<bool, String> {
    let new_db = study_data_dir.join("study.db");
    if new_db.exists() {
        return Ok(false);
    }

    let desktop_dir = match app.path().desktop_dir() {
        Ok(path) => path,
        Err(_) => return Ok(false),
    };
    let legacy_dir = desktop_dir.join("计组备考助手");
    let legacy_db = legacy_dir.join("study.db");
    if !legacy_db.exists() {
        return Ok(false);
    }

    std::fs::create_dir_all(study_data_dir).map_err(|e| e.to_string())?;
    std::fs::copy(&legacy_db, new_db).map_err(|e| e.to_string())?;

    for suffix in ["study.db-wal", "study.db-shm"] {
        let old_file = legacy_dir.join(suffix);
        if old_file.exists() {
            let _ = std::fs::copy(&old_file, study_data_dir.join(suffix));
        }
    }

    // 返回 true 表示已迁移，调用方可提示用户手动删除桌面旧文件夹
    Ok(true)
}
```

在 `run()` 中增加迁移成功提示：
```rust
let migrated = migrate_legacy_desktop_db(app, &study_data_dir)?;
if migrated {
    eprintln!("[迁移] 已将桌面旧版数据复制到新目录，可手动删除桌面上的「计组备考助手」文件夹。");
}
```

**验证**：全新安装后桌面只有一个快捷方式图标，无文件夹。升级安装后旧数据自动迁移并提示清理。

---

## 阶段三：架构优化

### 步骤 3.1：拆分 App.tsx 为子页面组件

将 `App.tsx` 中的工作区内容拆分为独立组件：

```
src/components/
├── pages/
│   ├── OverviewPage.tsx      # 概览统计页
│   ├── PracticePage.tsx      # 练习主页面（含 TaskCard + ChapterProgress）
│   └── EmptyState.tsx        # 空状态提示
```

`App.tsx` 只负责：
- 全局状态管理（章节、模式）
- 布局骨架（Sidebar + Workspace）
- 页面路由切换

### 步骤 3.2：引入 React Context 管理跨组件状态

新建 `src/contexts/AppContext.tsx`：

```tsx
interface AppContextValue {
  activeChapterId: string;
  setActiveChapterId: (id: string) => void;
  practiceMode: PracticeMode;
  switchMode: (mode: PracticeMode) => void;
  selectedKnowledgePointId: string | null;
  selectKnowledgePoint: (kpId: string) => void;
  overall: OverallProgress | null;
  refreshOverall: () => void;
}
```

消除 App → MetricGrid / ChapterProgressPanel / TaskCard 之间的多层 prop drilling。

### 步骤 3.3：引入前端路由

安装 `react-router-dom`，为不同视图分配路由：

| 路径 | 页面 |
|------|------|
| `/` | 概览统计 |
| `/practice/sequential/:chapterId` | 顺序练习 |
| `/practice/weak/:chapterId` | 薄弱加强 |
| `/practice/random/:chapterId` | 随机抽查 |
| `/practice/kp/:chapterId/:kpId` | 考点专练 |

好处：
- 支持深链接，可直接跳转到特定章节/题目
- 浏览器前进后退按钮可用
- 状态与 URL 同步，刷新不丢失位置

### 步骤 3.4：修复缓存层使其真正生效

**文件**：`src/services/cache.ts`

```ts
const CACHE_TTL = 5 * 60 * 1000; // 5分钟

interface CacheEntry<T> {
  data: T;
  timestamp: number;
}

let chaptersCache: CacheEntry<Chapter[]> | null = null;
const knowledgePointsCache = new Map<string, CacheEntry<KnowledgePoint[]>>();

function isExpired<T>(entry: CacheEntry<T> | null): boolean {
  return !entry || Date.now() - entry.timestamp > CACHE_TTL;
}

export async function getChapters(): Promise<Chapter[]> {
  if (isExpired(chaptersCache)) {
    const data = await api.listChapters();
    chaptersCache = { data, timestamp: Date.now() };
  }
  return chaptersCache!.data;
}

export function invalidateCache(): void {
  chaptersCache = null;
  knowledgePointsCache.clear();
}
```

在 `useChapters.ts` 中调用 `invalidateCache()`：
```ts
// 当 import_seed_data 成功后调用
invalidateCache();
```

### 步骤 3.5：后端引入连接池（r2d2 或多连接策略）

**方案 A（推荐）**：使用 `r2d2` + `r2d2_sqlite` 连接池

```toml
# Cargo.toml 新增
r2d2 = "0.8"
r2d2_sqlite = "0.25"
```

```rust
// db.rs
pub struct DbState {
    pub pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
}
```

**方案 B（轻量替代）**：保持单连接但将 Mutex 改为 RwLock，允许并发读：

```rust
use std::sync::RwLock;

pub struct DbState {
    pub conn: RwLock<Connection>,
}
```

读操作用 `conn.read()`，写操作用 `conn.write()`，配合 SQLite WAL 模式实现读写并发。

### 步骤 3.6：引入结构化错误类型

**文件**：新建 `src-tauri/src/errors.rs`

```rust
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub code: String,
    pub message: String,
}

impl AppError {
    pub fn not_found(entity: &str, id: &str) -> Self {
        Self { code: "NOT_FOUND".into(), message: format!("{entity} {id} 不存在") }
    }
    pub fn db_error(msg: &str) -> Self {
        Self { code: "DB_ERROR".into(), message: msg.into() }
    }
    pub fn validation(msg: &str) -> Self {
        Self { code: "VALIDATION".into(), message: msg.into() }
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        Self::db_error(&e.to_string())
    }
}
```

命令返回 `Result<T, AppError>`，前端根据 `code` 显示不同提示。

### 步骤 3.7：CSS 模块化

将全局 CSS 改为 CSS Modules：

```
src/components/layout/Sidebar.module.css
src/components/overview/MetricGrid.module.css
src/components/practice/TaskCard.module.css
```

引入设计 Token 文件 `src/tokens.css`：
```css
:root {
  --color-bg: #f8f4ed;
  --color-surface: #ffffff;
  --color-dark: #202829;
  --color-accent: #c9ff5a;
  --color-text: #202829;
  --color-muted: #6e7b73;
  --font-weight-heavy: 900;
  --radius: 6px;
  --shadow: 6px 6px 0 var(--color-dark);
}
```

---

## 阶段四：中等 Bug 修复

### 步骤 4.1：修复 answers 状态无限增长

**文件**：`src/hooks/useTaskDetail.ts`

限制 `answers` Record 大小，采用 LRU 策略保留最近 30 题的答案：

```ts
const MAX_CACHED_ANSWERS = 30;

const updateAnswer = useCallback(
  (value: string) => {
    if (!taskId) return;
    setAnswers((prev) => {
      const next = { ...prev, [taskId]: value };
      const keys = Object.keys(next);
      if (keys.length > MAX_CACHED_ANSWERS) {
        delete next[keys[0]]; // 移除最早的
      }
      answersRef.current = next;
      return next;
    });
  },
  [taskId]
);
```

### 步骤 4.2：修复评估按钮不可撤销问题

**文件**：`src/components/practice/TaskCard.tsx`

将条件渲染改为始终显示两个按钮，已选状态高亮：

```tsx
<button
  className={`assess-button assess-mastered ${status === "mastered" ? "is-selected" : ""}`}
  type="button"
  disabled={assessing}
  onClick={() => onAssessReq(req.reqIndex, "mastered")}
>
  ✓ 掌握
</button>
<button
  className={`assess-button assess-needs-work ${status === "needs_work" ? "is-selected" : ""}`}
  type="button"
  disabled={assessing}
  onClick={() => onAssessReq(req.reqIndex, "needs_work")}
>
  ✗ 需加强
</button>
```

CSS 新增：
```css
.assess-button.is-selected {
  outline: 3px solid currentColor;
  outline-offset: 2px;
}
```

### 步骤 4.3：修复 saveCurrentAnswer 空答案不覆盖旧记录

**文件**：`src/hooks/useTaskDetail.ts`

当用户清空作答时，应保存空字符串以覆盖旧答案：

```ts
// 修改前
if (answer.trim().length > 0) {

// 修改后：始终保存，空字符串也覆盖
if (answer !== undefined) {
```

同步修改后端 `commands/practice.rs` 的去重逻辑：只有当新旧答案**完全相同**时才跳过，空字符串也正常写入。

### 步骤 4.4：修复 useStats.refresh 闭包陈旧值

**文件**：`src/hooks/useStats.ts`

将 `chapterId` 通过 refresh 参数传入而非闭包捕获：

```ts
const refresh = useCallback(async (targetChapterId?: string) => {
  setLoading(true);
  setError(null);
  try {
    const o = await api.getOverallProgress();
    setOverall(o);
    const cid = targetChapterId ?? chapterId;
    if (cid) {
      const c = await api.getChapterProgress({ chapterId: cid });
      setChapter(c);
    } else {
      setChapter(null);
    }
  } catch (e) {
    // ...
  } finally {
    setLoading(false);
  }
}, [chapterId]);
```

调用方：
```ts
const handleAssessTask = useCallback(
  async (assessment: Assessment) => {
    await assessWholeTask(assessment);
    refreshStats(activeChapterId); // 显式传入当前 chapterId
  },
  [assessWholeTask, refreshStats, activeChapterId]
);
```

### 步骤 4.5：修复 useTasks.refresh 无 AbortSignal

**文件**：`src/hooks/useTasks.ts`

```ts
const refresh = useCallback(() => {
  const controller = new AbortController();
  fetchTasks(controller.signal);
  // 注意：调用方需在组件卸载时 abort，或使用 ref 追踪 controller
}, [fetchTasks]);
```

### 步骤 4.6：修复 MetricGrid 硬编码版本号

**文件**：`src/components/overview/MetricGrid.tsx:36`

```tsx
// 修改前
<strong>v0.2</strong>

// 修改后：从 package.json 读取或用 prop 传入
<strong>v{__APP_VERSION__}</strong>
```

在 `vite.config.ts` 中定义：
```ts
define: {
  __APP_VERSION__: JSON.stringify("0.2.1"),
}
```

### 步骤 4.7：增强 ErrorBoundary 支持恢复

**文件**：`src/components/ErrorBoundary.tsx`

```tsx
type State = { hasError: boolean; error: Error | null };

export class ErrorBoundary extends Component<Props, State> {
  state: State = { hasError: false, error: null };

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  handleReset = () => {
    this.setState({ hasError: false, error: null });
  };

  render() {
    if (this.state.hasError) {
      return (
        <div style={{ padding: 48, textAlign: "center" }}>
          <h2>页面出错了</h2>
          <p>{this.state.error?.message ?? "未知错误"}</p>
          <button onClick={this.handleReset} style={{ marginTop: 16 }}>
            重试
          </button>
          <p style={{ marginTop: 8 }}>或刷新页面。</p>
        </div>
      );
    }
    return this.props.children;
  }
}
```

---

## 阶段五：功能补全

### 步骤 5.1：难度筛选 UI

在 `Sidebar.tsx` 的模式区域下方增加难度筛选：

```tsx
<section className="difficulty-section">
  <p className="eyebrow">难度筛选</p>
  <div className="difficulty-list">
    {(["all", "foundation", "applied", "advanced"] as const).map((d) => (
      <button key={d} className={`diff-button ${activeDifficulty === d ? "is-active" : ""}`}
        onClick={() => onFilterDifficulty(d)}>
        {DIFF_LABELS[d]}
      </button>
    ))}
  </div>
</section>
```

将 `difficulty` 参数传入 `useTasks`。

### 步骤 5.2：随机/薄弱模式支持"加载更多"

**文件**：`src/hooks/useTasks.ts`

```ts
const [randomBatch, setRandomBatch] = useState<TaskDetail[]>([]);

const loadMore = useCallback(async () => {
  const more = await api.getRandomTasks({
    chapterId,
    count: 10,
    onlyWeak: mode === "weak",
  });
  setRandomBatch(prev => [...prev, ...more]);
}, [chapterId, mode]);
```

### 步骤 5.3：章节切换时重置考点选择

**文件**：`src/App.tsx`

```ts
const handleSwitchChapter = useCallback(
  async (chapterId: string) => {
    await saveCurrentAnswer();
    setActiveChapterId(chapterId);
    setActiveTaskIndex(0);
    if (mode === "knowledge-point") {
      switchMode("sequential"); // 切换章节时退出考点模式
    }
    workspaceRef.current?.scrollTo({ top: 0, behavior: "smooth" });
  },
  [saveCurrentAnswer, mode, switchMode]
);
```

### 步骤 5.4：练习位置持久化

将 `activeChapterId`、`activeTaskIndex`、`practiceMode` 保存到 `localStorage`：

```ts
// usePracticeMode.ts
const [mode, setMode] = useState<PracticeMode>(() => {
  return (localStorage.getItem("practiceMode") as PracticeMode) ?? "sequential";
});

const switchMode = (newMode: PracticeMode) => {
  setMode(newMode);
  localStorage.setItem("practiceMode", newMode);
  setSelectedKnowledgePointId(null);
};
```

### 步骤 5.5：键盘快捷键

在 `App.tsx` 中监听键盘事件：

```ts
useEffect(() => {
  const handler = (e: KeyboardEvent) => {
    if (e.target instanceof HTMLTextAreaElement) return; // 作答区不拦截
    if (e.key === "ArrowLeft") onPrev();
    if (e.key === "ArrowRight") onNext();
  };
  window.addEventListener("keydown", handler);
  return () => window.removeEventListener("keydown", handler);
}, [onPrev, onNext]);
```

### 步骤 5.6：数据导入 UI

在 Sidebar 底部增加"导入题库"按钮：

```tsx
<button className="import-button" type="button" onClick={handleImport}>
  导入题库 JSON
</button>
<input ref={fileInputRef} type="file" accept=".json" hidden onChange={onFileSelected} />
```

读取 JSON 文件后调用 `api.importSeedData({ jsonData: text })`。

### 步骤 5.7：数据导出功能

后端新增命令 `export_data`：

```rust
#[tauri::command]
pub fn export_data(state: State<DbState>) -> Result<String, AppError> {
    // 导出所有 answer_records 和 task_assessments 为 JSON
}
```

前端增加"导出学习记录"按钮，将结果保存为 `.json` 文件（使用 Tauri 的 `dialog.save` + `fs.writeTextFile`）。

### 步骤 5.8：数据重置功能

在 Sidebar 设置区增加"重置学习进度"按钮，二次确认后调用后端清空评估和作答记录：

```rust
#[tauri::command]
pub fn reset_progress(state: State<DbState>) -> Result<(), AppError> {
    let conn = state.conn.lock().map_err(|e| AppError::db_error(&e.to_string()))?;
    conn.execute_batch("DELETE FROM answer_records; DELETE FROM requirement_assessments; DELETE FROM task_assessments;")
        .map_err(|e| AppError::db_error(&e.to_string()))?;
    Ok(())
}
```

---

## 阶段六：工程化改进

### 步骤 6.1：添加后端单元测试

新建 `src-tauri/src/db_test.rs`（或使用 `#[cfg(test)]` 模块）：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();
        create_tables(&conn).unwrap();
        conn
    }

    #[test]
    fn test_seed_import() { /* ... */ }

    #[test]
    fn test_list_tasks_with_filters() { /* ... */ }

    #[test]
    fn test_assess_task_updates_progress() { /* ... */ }
}
```

### 步骤 6.2：添加前端测试

安装 `vitest` + `@testing-library/react`：

```bash
npm install -D vitest @testing-library/react @testing-library/jest-dom jsdom
```

为关键 hooks 编写测试：
- `useTasks` — 验证各模式下的数据加载逻辑
- `useTaskDetail` — 验证保存答案、评估流程
- `useStats` — 验证刷新时 chapterId 传递

### 步骤 6.3：CI 增加测试步骤

**文件**：`.github/workflows/build-windows-release.yml`

```yaml
- name: Run frontend tests
  run: npm test

- name: Run Rust tests
  working-directory: src-tauri
  run: cargo test
```

### 步骤 6.4：种子数据外部化

将 `include_str!("../../data/seed.json")` 改为运行时从文件系统读取：

```rust
pub fn init_db(study_data_dir: &PathBuf) -> Result<Connection, String> {
    // ...
    if is_new {
        let seed_path = study_data_dir.join("seed.json");
        if seed_path.exists() {
            let json = std::fs::read_to_string(&seed_path).map_err(|e| e.to_string())?;
            import_seed_data_from_json(&mut conn, &json)?;
        }
    }
    // ...
}
```

好处：更新题库无需重新编译，只需替换 `seed.json` 文件。

### 步骤 6.5：添加 Tauri 权限声明

**文件**：`src-tauri/capabilities/default.json`

添加文件对话和写入权限以支持导入/导出：

```json
{
  "permissions": [
    "opener:default",
    "core:default",
    "core:path:default",
    "core:event:default",
    "dialog:default",
    "fs:default"
  ]
}
```

### 步骤 6.6：assess_requirement 后端校验 status 值

**文件**：`src-tauri/src/commands/practice.rs`

```rust
#[tauri::command]
pub fn assess_requirement(
    state: State<DbState>,
    task_id: String,
    req_index: i64,
    status: String,
) -> Result<(), AppError> {
    if status != "mastered" && status != "needs_work" {
        return Err(AppError::validation(
            &format!("status 必须为 mastered 或 needs_work，收到: {status}"),
        ));
    }
    // ...
}
```

---

## 修复优先级总览

| 阶段 | 内容 | 工作量 | 影响 |
|------|------|--------|------|
| 一 | 紧急 Bug 修复 | 0.5 天 | 修复数据丢失、命令报错等阻塞问题 |
| 二 | 桌面安装体验 | 0.5 天 | 解决用户反馈最强烈的体验问题 |
| 三 | 架构优化 | 3-4 天 | 为后续功能扩展打基础 |
| 四 | 中等 Bug 修复 | 1 天 | 修复评估/保存/刷新等交互问题 |
| 五 | 功能补全 | 3-5 天 | 补齐核心缺失功能 |
| 六 | 工程化改进 | 2-3 天 | 测试覆盖、CI、数据外部化 |

建议按顺序执行，每完成一个阶段做一次集成验证后提交。
