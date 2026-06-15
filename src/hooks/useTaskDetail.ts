import { useState, useEffect, useCallback } from "react";
import { api } from "../services/api";
import type { TaskDetail, RequirementStatus, Assessment } from "../types";

export function useTaskDetail(taskId: string | null) {
  const [detail, setDetail] = useState<TaskDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [answers, setAnswers] = useState<Record<string, string>>({});
  const [revealed, setRevealed] = useState(false);
  const [reqStatuses, setReqStatuses] = useState<Record<string, RequirementStatus>>({});

  useEffect(() => {
    if (!taskId) {
      setDetail(null);
      setLoading(false);
      return;
    }
    let cancelled = false;
    setLoading(true);
    api.getTaskDetail({ taskId }).then((d) => {
      if (!cancelled) {
        setDetail(d);
        setAnswers((prev) => ({
          ...prev,
          [taskId]: d.latestAnswer ?? "",
        }));
        setRevealed(false);
        const statusMap: Record<string, RequirementStatus> = {};
        for (const rs of d.requirementStatuses) {
          statusMap[`${taskId}-${rs.reqIndex}`] = rs;
        }
        setReqStatuses(statusMap);
        setLoading(false);
      }
    });
    return () => { cancelled = true; };
  }, [taskId]);

  const updateAnswer = useCallback(
    (value: string) => {
      if (!taskId) return;
      setAnswers((prev) => ({ ...prev, [taskId]: value }));
    },
    [taskId]
  );

  const saveCurrentAnswer = useCallback(async () => {
    if (!taskId || !detail) return;
    const answer = answers[taskId] ?? "";
    if (answer.trim().length > 0) {
      await api.saveAnswer({
        taskId,
        chapterId: detail.task.chapterId,
        answerText: answer,
      });
    }
  }, [taskId, detail, answers]);

  const revealReference = useCallback(() => {
    setRevealed(true);
  }, []);

  const assessReq = useCallback(
    async (reqIndex: number, status: Assessment) => {
      if (!taskId) return;
      await api.assessRequirement({ taskId, reqIndex, status });
      setReqStatuses((prev) => ({
        ...prev,
        [`${taskId}-${reqIndex}`]: {
          reqIndex,
          status,
        },
      }));
    },
    [taskId]
  );

  const assessWholeTask = useCallback(
    async (assessment: Assessment) => {
      if (!taskId || !detail) return;
      await api.assessTask({
        taskId,
        chapterId: detail.task.chapterId,
        assessment,
      });
    },
    [taskId, detail]
  );

  const currentAnswer = taskId ? (answers[taskId] ?? "") : "";

  return {
    detail,
    loading,
    currentAnswer,
    revealed,
    reqStatuses,
    updateAnswer,
    saveCurrentAnswer,
    revealReference,
    assessReq,
    assessWholeTask,
  };
}