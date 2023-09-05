import { z } from "zod";

const title = z.string({ required_error: "title is a required field" });
const logo = z.string().optional();
const description = z.string().optional();
export const createOrganisationSchema = z.object({
  title,
  logo,
  description,
});

export const updateOrganisationSchema = z.object({
  title: title.optional(),
  logo,
  description,
});

export type CreateOrganisationSchema = z.TypeOf<
  typeof createOrganisationSchema
>;
export type UpdateOrganisationSchema = z.TypeOf<
  typeof updateOrganisationSchema
>;
