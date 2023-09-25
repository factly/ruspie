import { z } from "zod";

const name = z
  .string({ required_error: "field name is required" })
  .min(4, "name has to be atleast 4 chars long");
const s3_url = z.string({ required_error: "field s3_url is required" }).url();
const extenstion = z.enum(["csv", "parquet"]);

export const createFileSchema = z.object({
  name,
  s3_url: s3_url.optional(),
  extenstion: extenstion.optional(),
});

export const updateFileSchema = z
  .object({
    name: name.optional(),
    s3_url: s3_url.optional(),
    extenstion: extenstion.optional(),
  })
  .refine((data) => data.name || data.s3_url || data.extenstion);

export type CreateFileSchema = z.TypeOf<typeof createFileSchema>;
export type UpdateFileSchema = z.TypeOf<typeof updateFileSchema>;
