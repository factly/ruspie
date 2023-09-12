import { OrgaisationParams } from "./oragnisation_param";

export interface ProjectParam {
  params: {
    projectId: string;
  } & OrgaisationParams["params"];
}
