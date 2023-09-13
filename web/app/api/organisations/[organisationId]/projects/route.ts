import { errorToResp } from "@/lib/utils/error_to_resp";
import { getServerUrl } from "@/lib/utils/serverUrl";
import { createProjectSchema } from "@/lib/zod/validators/projects";
import { APIError } from "@/types/api_error";
import { Project } from "@/types/organisation";
import { OrgaisationParam } from "@/types/params/oragnisation_param";
import axios, { AxiosError, AxiosResponse } from "axios";
import { ZodError } from "zod";

export const GET = async (
  _req: Request,
  { params: { organisationId } }: OrgaisationParam,
) => {
  const errorResp: APIError = { message: "", status: 500 };
  if (!organisationId) {
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }

  const serverUrl = getServerUrl();
  if (!serverUrl) {
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }

  try {
    const res: AxiosResponse<{ projects: Project[] }> = await axios.get(
      serverUrl + `/organisations/${organisationId}/projects/`,
    );
    return new Response(JSON.stringify(res.data));
  } catch (err) {
    if (err instanceof AxiosError) {
      const response = err.response;
      if (!response) {
        errorResp.message = "Internal Server Error";
        errorResp.status = 500;
        return new Response(...errorToResp(errorResp));
      }
      if (response.status === 400 || response.status === 401) {
        errorResp.message = JSON.stringify(response.data);
        errorResp.status = response.status;
        return new Response(...errorToResp(errorResp));
      }
    }
    errorResp.message = "Internal Server Error";
    errorResp.status = 500;
    return new Response(...errorToResp(errorResp));
  }
};

export const POST = async (
  req: Request,
  { params: { organisationId } }: OrgaisationParam,
) => {
  const errorResp: APIError = { message: "", status: 500 };
  if (!organisationId) {
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }

  const serverUrl = getServerUrl();
  if (!serverUrl) {
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }
  try {
    const body = await req.json();
    const project = createProjectSchema.parse(body);
    const res: AxiosResponse<Project> = await axios.post(
      serverUrl + `/organisatoins/${organisationId}/projects`,
      project,
      {
        headers: {
          "X-User": 1,
        },
      },
    );
    return new Response(JSON.stringify(res.data), { status: 201 });
  } catch (err) {
    if (err instanceof ZodError) {
      errorResp.message = err.message;
      errorResp.status = 400;
      return new Response(...errorToResp(errorResp));
    }
    if (err instanceof AxiosError) {
      const resp = err.response!;
      if (resp.status === 400 || resp.status === 401 || resp.status === 409) {
        errorResp.message = JSON.stringify(resp.data.errors[0].message);
        errorResp.status = resp.status;
        return new Response(...errorToResp(errorResp));
      }
    }
    errorResp.message = "Internal Server Error";
    errorResp.status = 500;
    return new Response(...errorToResp(errorResp));
  }
};
