import { errorToResp } from "@/lib/utils/error_to_resp";
import { getServerUrl } from "@/lib/utils/serverUrl";
import { updateOrganisationSchema } from "@/lib/zod/validators/organisation";
import { APIError } from "@/types/api_error";
import { Organisation } from "@/types/organisation";
import { OrgaisationParam } from "@/types/params/oragnisation_param";
import axios, { AxiosError, AxiosResponse } from "axios";
import { ZodError } from "zod";

export async function GET(
  _req: Request,
  { params: { organisationId } }: OrgaisationParam,
) {
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
    const resp: AxiosResponse<Organisation> = await axios(
      `${serverUrl}/organisations/${organisationId}/`,
      { headers: process.env.KAVACH_ENABLED ? { "X-User": "1" } : undefined },
    );
    return new Response(JSON.stringify(resp.data), { status: 200 });
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
  { params: { organisationId } }: OrgaisationParam,
) {
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
    const organisation = updateOrganisationSchema.parse(body);
    console.log(organisation);
    const res: AxiosResponse<Organisation> = await axios.put(
      serverUrl + `/organisations/${organisationId}/`,
      organisation,
      { headers: process.env.KAVACH_ENABLED ? { "X-User": "1" } : undefined },
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
    await axios.delete(serverUrl + `/organisations/${organisationId}/`, {
      headers: process.env.KAVACH_ENABLED ? { "X-User": "1" } : undefined,
    });
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
