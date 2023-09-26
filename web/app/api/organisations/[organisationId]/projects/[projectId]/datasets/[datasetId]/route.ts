import { errorToResp } from "@/lib/utils/error_to_resp";
import { getServerUrl } from "@/lib/utils/serverUrl";
import { updateFileSchema } from "@/lib/zod/validators/files";
import { APIError } from "@/types/api_error";
import { File } from "@/types/file";
import { FileParam } from "@/types/params/file_param";
import axios, { AxiosError, AxiosResponse } from "axios";
import { ZodError } from "zod";

export async function GET(
  _req: Request,
  { params: { organisationId, projectId, datasetId: fileId } }: FileParam,
) {
  const errorResp: APIError = { message: "", status: 500 };
  if (!organisationId || !projectId || !fileId) {
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }

  const serverUrl = getServerUrl();
  if (!serverUrl) {
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }

  try {
    const res: AxiosResponse<File> = await axios.get(
      serverUrl +
      `/organisations/${organisationId}/projects/${projectId}/files/${fileId}/`,
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

export async function PATCH(
  req: Request,
  { params: { organisationId, projectId, datasetId: fileId } }: FileParam,
) {
  const errorResp: APIError = { message: "", status: 500 };
  if (!organisationId || !projectId || !fileId) {
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
    const file = updateFileSchema.parse(body);
    return new Response(JSON.stringify(file));
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
}

export const DELETE = async (
  _req: Request,
  { params: { organisationId, projectId, datasetId: fileId } }: FileParam,
) => {
  const errorResp: APIError = { message: "", status: 500 };
  if (!organisationId || !projectId || !fileId) {
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
      serverUrl +
      `/organisations/${organisationId}/projects/${projectId}/files/${fileId}/`,
      {
        headers: {
          "X-User": "1",
        },
      },
    );
    return new Response("Successfully deleted the organisation");
  } catch (err) {
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
