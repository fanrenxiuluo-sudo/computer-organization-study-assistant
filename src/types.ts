export type Chapter = {
  id: string;
  title: string;
  orderIndex: number;
};

export type AssessmentTask = {
  id: string;
  chapterId: string;
  difficulty: "foundation" | "applied" | "advanced";
  outcome: string;
  scenario: string;
  requirements: string[];
  reference: string;
};

export type SeedData = {
  chapters: Chapter[];
  tasks: AssessmentTask[];
};
