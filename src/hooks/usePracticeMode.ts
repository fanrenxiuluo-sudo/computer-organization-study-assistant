import { useState } from "react";
import type { PracticeMode } from "../types";

export function usePracticeMode() {
  const [mode, setMode] = useState<PracticeMode>("sequential");
  const [selectedKnowledgePointId, setSelectedKnowledgePointId] = useState<string | null>(null);

  const switchMode = (newMode: PracticeMode) => {
    setMode(newMode);
    setSelectedKnowledgePointId(null);
  };

  const selectKnowledgePoint = (kpId: string) => {
    setMode("knowledge-point");
    setSelectedKnowledgePointId(kpId);
  };

  return {
    mode,
    selectedKnowledgePointId,
    switchMode,
    selectKnowledgePoint,
    setSelectedKnowledgePointId,
  };
}