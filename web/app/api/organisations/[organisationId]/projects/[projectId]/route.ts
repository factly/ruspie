import { errorToResp } from "@/lib/utils/error_to_resp";
import { getServerUrl } from "@/lib/utils/serverUrl";
import {
  changeOrgProjectSchema,
  updateProjectSchema,
} from "@/lib/zod/validators/projects";
import { APIError } from "@/types/api_error";
import { Project } from "@/types/organisation";
import { ProjectParam } from "@/types/params/project_param";
import axios, { AxiosError, AxiosResponse } from "axios";
import { ZodError } from "zod";

export async function GET(
  _req: Request,
  { params: { projectId, organisationId } }: ProjectParam,
) {
  const errorResp: APIError = { message: "", status: 500 };
  if (!organisationId || !projectId) {
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }

  const serverUrl = getServerUrl();
  if (!serverUrl) {
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }

  try {
    const res: AxiosResponse<Project> = await axios.get(
      serverUrl + `/organisations/${organisationId}/projects/${projectId}/`,
      {
        headers: {
          "X-User": 1,
        },
      },
    );
    return new Response(JSON.stringify(res.data));
  } catch (err) {
    if (err instanceof AxiosError) {
      const resp = err.response!;
      if (resp.status === 404 || resp.status === 401) {
        errorResp.message = JSON.stringify(resp.data.errors[0].message);
        errorResp.status = resp.status;
        return new Response(...errorToResp(errorResp));
      }
    }
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }
}

export async function PUT(
  req: Request,
  { params: { organisationId, projectId } }: ProjectParam,
) {
  const errorResp: APIError = { message: "", status: 500 };
  if (!organisationId || !projectId) {
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
    const project = updateProjectSchema.parse(body);
    console.log(project);
    const res: AxiosResponse<Project> = await axios.put(
      serverUrl + `/organisations/${organisationId}/projects/${projectId}/`,
      project,
      {
        headers: {
          "X-User": "1",
        },
      },
    );
    return new Response(JSON.stringify(res.data));
  } catch (err) {
    if (err instanceof ZodError) {
      errorResp.message = err.message;
      errorResp.status = 400;
      return new Response(...errorToResp(errorResp));
    }
    if (err instanceof AxiosError) {
      const response = err.response;
      if (!response) {
        errorResp.message = "Unhandled Error occured";
        errorResp.status = 500;
        return new Response(...errorToResp(errorResp));
      }
      if (
        response.status === 400 ||
        response.status === 404 ||
        response.status === 401 ||
        response.status === 409
      ) {
        errorResp.message = JSON.stringify(response.data);
        errorResp.status = response.status;
        return new Response(...errorToResp(errorResp));
      }
    }
    errorResp.message = "Internal server error";
    errorResp.status = 500;
    return new Response(...errorToResp(errorResp));
  }
}

export const DELETE = async (
  _req: Request,
  { params: { organisationId, projectId } }: ProjectParam,
) => {
  const errorResp: APIError = { message: "", status: 500 };
  if (!organisationId || !projectId) {
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }

  const serverUrl = getServerUrl();
  if (!serverUrl) {
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }

  try {
    await axios.delete(
      serverUrl + `/organisations/${organisationId}/projects/${projectId}/`,
      {
        headers: {
          "X-User": "1",
        },
      },
    );
    return new Response("Successfully deleted the project");
  } catch (err) {
    console.log(err);
    if (err instanceof AxiosError) {
      const response = err.response;
      console.log(response?.data);
      if (!response) {
        errorResp.message = "Unhandled Error occured";
        errorResp.status = 500;
        return new Response(...errorToResp(errorResp));
      }
      if (
        response.status === 400 ||
        response.status === 404 ||
        response.status === 401
      ) {
        errorResp.message = JSON.stringify(response.data);
        errorResp.status = response.status;
        return new Response(...errorToResp(errorResp));
      }
    }
    errorResp.message = "Internal server error";
    errorResp.status = 500;
    return new Response(...errorToResp(errorResp));
  }
};

export const PATCH = async (
  req: Request,
  { params: { organisationId, projectId } }: ProjectParam,
) => {
  const errorResp: APIError = { message: "", status: 500 };
  if (!organisationId || !projectId) {
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
    const changeOrgReq = changeOrgProjectSchema.parse(body);
    const res = await axios.patch(
      serverUrl +
        `/organisations/${organisationId}/projects/${projectId}/change_org`,
      {
        new_org_id: changeOrgReq.new_org_id.toString(),
      },
      {
        headers: {
          "X-User": 1,
        },
      },
    );
    return new Response(JSON.stringify(res.data));
  } catch (err) {
    console.log(err);
    if (err instanceof AxiosError) {
      const response = err.response;
      console.log(response?.data);
      if (!response) {
        errorResp.message = "Unhandled Error occured";
        errorResp.status = 500;
        return new Response(...errorToResp(errorResp));
      }
      if (
        response.status === 400 ||
        response.status === 404 ||
        response.status === 401
      ) {
        errorResp.message = JSON.stringify(response.data);
        errorResp.status = response.status;
        return new Response(...errorToResp(errorResp));
      }
    }
    errorResp.message = "Internal server error";
    errorResp.status = 500;
    return new Response(...errorToResp(errorResp));
  }
};
