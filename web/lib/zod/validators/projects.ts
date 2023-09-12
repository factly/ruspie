import { z } from "zod";

const title = z
  .string({ required_error: "title is a required field" })
  .min(4, "title must contain atleast 4 chars");
const description = z.string().optional();
export const createProjectSchema = z.object({
  title,
  description,
});

export const updateProjectSchema = z.object({
  title: title.optional(),
  description: description.optional(),
});

export type CreateProjectSchema = z.TypeOf<typeof createProjectSchema>;
export type UpdateProjectSchema = z.TypeOf<typeof updateProjectSchema>;
