export interface File {
  id: string;
  name: string;
  s3_url: string;
  extenstion: Extenstion;
  project_id: number;
  updatedAt: Date;
  createdAt: Date;
}

export enum Extenstion {
  csv,
  parquet,
}
