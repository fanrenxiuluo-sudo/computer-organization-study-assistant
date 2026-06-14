# 数据库设计

## 设计目标

数据库用于保存题库、练习记录、错题记录和章节掌握度。第一阶段先使用 JSON 种子数据，后续接入 SQLite。

## 核心实体

### chapters

章节表。

| 字段 | 含义 |
| --- | --- |
| id | 章节编号 |
| title | 章节名称 |
| order_index | 排序 |

### questions

题目表。

| 字段 | 含义 |
| --- | --- |
| id | 题目编号 |
| chapter_id | 所属章节 |
| type | 题型 |
| stem | 题干 |
| options | 选项 |
| answer | 标准答案 |
| analysis | 解析 |
| difficulty | 难度 |

### practice_records

练习记录表。

| 字段 | 含义 |
| --- | --- |
| id | 记录编号 |
| question_id | 题目编号 |
| user_answer | 用户答案 |
| is_correct | 是否正确 |
| created_at | 作答时间 |

### wrong_questions

错题表。

| 字段 | 含义 |
| --- | --- |
| id | 错题编号 |
| question_id | 题目编号 |
| wrong_count | 错误次数 |
| last_wrong_at | 最近错误时间 |
| mastered | 是否已掌握 |

