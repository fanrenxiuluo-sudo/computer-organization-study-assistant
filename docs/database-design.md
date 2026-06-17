# 数据库设计

## 设计目标

数据库用于保存题库、OBE 综合任务、练习记录、自评状态和课程目标达成度。使用 SQLite 本地存储，首次启动时自动从 JSON 种子数据导入。

## 架构说明

- 存储引擎：SQLite（WAL 模式）
- ORM：无，直接使用 rusqlite 执行 SQL
- 数据库文件：位于 Tauri app_local_data_dir 下的 `study-data/study.db`
- 种子数据：编译时通过 `include_str!` 嵌入 `data/seed.json`

## 核心实体

### schema_version

数据库版本管理表。

| 字段 | 类型 | 含义 |
| --- | --- | --- |
| version | INTEGER PK | 当前 schema 版本号 |

### course_outcomes

课程目标表，对应 OBE 教学大纲中的课程目标。

| 字段 | 类型 | 含义 |
| --- | --- | --- |
| id | TEXT PK | 课程目标编号（如 co1） |
| code | TEXT NOT NULL | 目标代码（如"课程目标1"） |
| description | TEXT NOT NULL | 目标描述 |
| order_index | INTEGER NOT NULL | 排序序号 |

### chapters

章节表。

| 字段 | 类型 | 含义 |
| --- | --- | --- |
| id | TEXT PK | 章节编号 |
| title | TEXT NOT NULL | 章节名称 |
| description | TEXT NOT NULL DEFAULT '' | 章节描述 |
| order_index | INTEGER NOT NULL | 排序序号 |
| course_outcome_id | TEXT NOT NULL FK | 关联的课程目标 |

### knowledge_points

知识点表，每个章节包含多个知识点。

| 字段 | 类型 | 含义 |
| --- | --- | --- |
| id | TEXT PK | 知识点编号 |
| chapter_id | TEXT NOT NULL FK | 所属章节 |
| name | TEXT NOT NULL | 知识点名称 |
| order_index | INTEGER NOT NULL | 排序序号 |

### tasks

综合任务表（核心题目表）。

| 字段 | 类型 | 含义 |
| --- | --- | --- |
| id | TEXT PK | 任务编号 |
| chapter_id | TEXT NOT NULL FK | 所属章节 |
| course_outcome_id | TEXT NOT NULL FK | 关联课程目标 |
| difficulty | TEXT NOT NULL | 难度：foundation/applied/advanced |
| scenario | TEXT NOT NULL | 题目场景描述 |
| reference | TEXT NOT NULL | 参考答案 |
| source | TEXT | 来源标注 |
| created_at | TEXT NOT NULL | 创建时间 |

索引：chapter_id, difficulty, course_outcome_id

### task_requirements

任务的作答要求（每道题有多个子要求）。

| 字段 | 类型 | 含义 |
| --- | --- | --- |
| id | INTEGER PK AUTO | 自增主键 |
| task_id | TEXT NOT NULL FK | 所属任务 |
| req_index | INTEGER NOT NULL | 要求序号 |
| content | TEXT NOT NULL | 要求内容 |

UNIQUE 约束：(task_id, req_index)

### task_knowledge_points

任务与知识点的多对多关联表。

| 字段 | 类型 | 含义 |
| --- | --- | --- |
| task_id | TEXT NOT NULL FK | 任务编号 |
| knowledge_point_id | TEXT NOT NULL FK | 知识点编号 |

复合主键：(task_id, knowledge_point_id)

### task_sources

题目来源追踪表。

| 字段 | 类型 | 含义 |
| --- | --- | --- |
| id | INTEGER PK AUTO | 自增主键 |
| task_id | TEXT NOT NULL FK | 所属任务 |
| university | TEXT NOT NULL | 来源学校 |
| year | TEXT | 年份 |
| exam_type | TEXT NOT NULL | 考试类型：final/postgraduate/obe/adapted |
| original_text | TEXT | 原题文本 |
| note | TEXT | 改编说明 |

### answer_records

作答记录表。

| 字段 | 类型 | 含义 |
| --- | --- | --- |
| id | INTEGER PK AUTO | 自增主键 |
| task_id | TEXT NOT NULL FK | 任务编号 |
| chapter_id | TEXT NOT NULL FK | 章节编号 |
| answer_text | TEXT NOT NULL DEFAULT '' | 作答内容 |
| created_at | TEXT NOT NULL | 作答时间 |

### requirement_assessments

逐条自评记录表。

| 字段 | 类型 | 含义 |
| --- | --- | --- |
| id | INTEGER PK AUTO | 自增主键 |
| task_id | TEXT NOT NULL FK | 任务编号 |
| req_index | INTEGER NOT NULL | 要求序号 |
| status | TEXT NOT NULL | 掌握状态：mastered/needs_work |
| created_at | TEXT NOT NULL | 评估时间 |

### task_assessments

整体自评记录表。

| 字段 | 类型 | 含义 |
| --- | --- | --- |
| id | INTEGER PK AUTO | 自增主键 |
| task_id | TEXT NOT NULL FK | 任务编号 |
| chapter_id | TEXT NOT NULL FK | 章节编号 |
| assessment | TEXT NOT NULL | 整体评价：mastered/needs_work |
| created_at | TEXT NOT NULL | 评估时间 |

## 种子数据统计

| 实体 | 数量 |
| --- | --- |
| 课程目标 | 6 |
| 章节 | 6 |
| 知识点 | 43 |
| 综合任务 | 24 |
