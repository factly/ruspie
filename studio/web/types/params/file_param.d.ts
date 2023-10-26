import { OrgaisationParam } from "./oragnisation_param";
import { ProjectParam } from "./project_param";

export interface FileParam {
  params: {
    datasetId: string;
  } & OrgaisationParam["params"] &
  ProjectParam["params"];
}
