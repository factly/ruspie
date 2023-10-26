import { Organisation } from "@/types/organisation";
import { create } from "zustand";

interface OrganisationsStore {
  organisations: Organisation[];
  setOrganisations: (organisations: Organisation[]) => void;
}

export const useOrganisationsStore = create<OrganisationsStore>()((set) => ({
  organisations: [],
  setOrganisations: (organisations) => set({ organisations }),
}));
