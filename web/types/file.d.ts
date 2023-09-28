export interface File {
  id: string;
  name: string;
  s3_url: string;
  extenstion: "csv" | "parquet";
  project_id: number;
  updatedAt: Date;
  createdAt: Date;
}
