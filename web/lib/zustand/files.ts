import { File } from "@/types/file";
import { create } from "zustand";

interface FileStore {
  files: File[];
  setFiles: (files: File[]) => void;
}

export const useFileStore = create<FileStore>()((set) => ({
  files: [],
  setFiles: (files) => set({ files }),
}));
