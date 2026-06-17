import { useState, useCallback } from "react";
import type { PracticeMode } from "../types";

const STORAGE_KEY_MODE = "jizubeikao_practiceMode";

function loadMode(): PracticeMode {
  try {
    const stored = localStorage.getItem(STORAGE_KEY_MODE);
    if (stored === "sequential" || stored === "weak" || stored === "random" || stored === "knowledge-point") {
      return stored;
    }
  } catch {}
  return "sequential";
}

export function usePracticeMode() {
  const [mode, setMode] = useState<PracticeMode>(loadMode);
  const [selectedKnowledgePointId, setSelectedKnowledgePointId] = useState<string | null>(null);

  const switchMode = useCallback((newMode: PracticeMode) => {
    setMode(newMode);
    setSelectedKnowledgePointId(null);
    try { localStorage.setItem(STORAGE_KEY_MODE, newMode); } catch {}
  }, []);

  const selectKnowledgePoint = useCallback((kpId: string) => {
    setMode("knowledge-point");
    setSelectedKnowledgePointId(kpId);
    try { localStorage.setItem(STORAGE_KEY_MODE, "knowledge-point"); } catch {}
  }, []);

  return {
    mode,
    selectedKnowledgePointId,
    switchMode,
    selectKnowledgePoint,
    setSelectedKnowledgePointId,
  };
}