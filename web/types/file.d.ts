export interface File {
  name: string;
  s3_url: string;
  extenstion: Extenstion;
  project_id: number;
}

export enum Extenstion {
  csv,
  parquet,
}
