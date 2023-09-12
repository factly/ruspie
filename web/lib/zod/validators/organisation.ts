import { z } from "zod";

const title = z
  .string({ required_error: "title is a required field" })
  .min(4, "title must contain atleast 4 chars");
const logo = z.string().optional();
const description = z.string().optional();
export const createOrganisationSchema = z.object({
  title,
  logo,
  description,
});

export const updateOrganisationSchema = z
  .object({
    title: title.optional(),
    logo,
    description,
  })
  .refine((data) => !data.title || !data.logo || !data.description);

export type CreateOrganisationSchema = z.TypeOf<
  typeof createOrganisationSchema
>;

export type UpdateOrganisationSchema = z.TypeOf<
  typeof updateOrganisationSchema
>;
