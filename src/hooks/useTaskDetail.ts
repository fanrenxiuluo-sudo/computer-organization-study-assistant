import { useState, useEffect, useCallback, useRef } from "react";
import { api } from "../services/api";
import type { TaskDetail, RequirementStatus, Assessment } from "../types";

export function useTaskDetail(taskId: string | null) {
  const [detail, setDetail] = useState<TaskDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [answers, setAnswers] = useState<Record<string, string>>({});
  const [revealed, setRevealed] = useState(false);
  const [reqStatuses, setReqStatuses] = useState<Record<string, RequirementStatus>>({});
  const [assessing, setAssessing] = useState(false);

  // Ref mirror of answers — avoids stale closures in saveCurrentAnswer
  const answersRef = useRef<Record<string, string>>({});

  useEffect(() => {
    if (!taskId) {
      setDetail(null);
      setLoading(false);
      return;
    }
    let cancelled = false;
    setLoading(true);
    api.getTaskDetail({ taskId })
      .then((d) => {
        if (!cancelled) {
          setDetail(d);
          setAnswers((prev) => {
            const next = { ...prev, [taskId]: d.latestAnswer ?? "" };
            answersRef.current = next;
            return next;
          });
          setRevealed(false);
          const statusMap: Record<string, RequirementStatus> = {};
          for (const rs of d.requirementStatuses) {
            statusMap[`${taskId}-${rs.reqIndex}`] = rs;
          }
          setReqStatuses(statusMap);
        }
      })
      .catch((error) => {
        console.error("加载任务详情失败", error);
        if (!cancelled) {
          setDetail(null);
        }
      })
      .finally(() => {
        if (!cancelled) {
          setLoading(false);
        }
      });
    return () => { cancelled = true; };
  }, [taskId]);

  const updateAnswer = useCallback(
    (value: string) => {
      if (!taskId) return;
      setAnswers((prev) => {
        const next = { ...prev, [taskId]: value };
        answersRef.current = next;
        return next;
      });
    },
    [taskId]
  );

  const saveCurrentAnswer = useCallback(async () => {
    if (!taskId || !detail) return;
    const answer = answersRef.current[taskId] ?? "";
    if (answer.trim().length > 0) {
      await api.saveAnswer({
        taskId,
        chapterId: detail.task.chapterId,
        answerText: answer,
      });
    }
  }, [taskId, detail]);

  const revealReference = useCallback(() => {
    void saveCurrentAnswer();
    setRevealed(true);
  }, [saveCurrentAnswer]);

  const assessReq = useCallback(
    async (reqIndex: number, status: Assessment) => {
      if (!taskId || assessing) return;
      setAssessing(true);
      try {
        await api.assessRequirement({ taskId, reqIndex, status });
        setReqStatuses((prev) => ({
          ...prev,
          [`${taskId}-${reqIndex}`]: {
            reqIndex,
            status,
          },
        }));
      } finally {
        setAssessing(false);
      }
    },
    [taskId, assessing]
  );

  const assessWholeTask = useCallback(
    async (assessment: Assessment) => {
      if (!taskId || !detail || assessing) return;
      setAssessing(true);
      try {
        await saveCurrentAnswer();
        await api.assessTask({
          taskId,
          chapterId: detail.task.chapterId,
          assessment,
        });
      } finally {
        setAssessing(false);
      }
    },
    [taskId, detail, saveCurrentAnswer, assessing]
  );

  const currentAnswer = taskId ? (answers[taskId] ?? "") : "";

  return {
    detail,
    loading,
    currentAnswer,
    revealed,
    reqStatuses,
    assessing,
    updateAnswer,
    saveCurrentAnswer,
    revealReference,
    assessReq,
    assessWholeTask,
  };
}
