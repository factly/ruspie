import { OrgaisationParam } from "./oragnisation_param";

export interface ProjectParam {
  params: {
    projectId: string;
  } & OrgaisationParam["params"];
}
