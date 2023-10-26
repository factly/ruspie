import { Project } from "@/types/organisation";
import { create } from "zustand";

interface ProjectsStore {
  projects: Project[];
  setProjects: (projects: Project[]) => void;
}

export const useProjectsStore = create<ProjectsStore>()((set) => ({
  projects: [],
  setProjects: (projects) => set({ projects }),
}));
