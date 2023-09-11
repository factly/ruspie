import { errorToResp } from "@/lib/utils/error_to_resp";
import { getServerUrl } from "@/lib/utils/serverUrl";
import {
  CreateOrganisationSchema,
  createOrganisationSchema,
} from "@/lib/zod/validators/organisation";
import { APIError } from "@/types/api_error";
import { Organisation } from "@/types/organisation";
import axios, { AxiosError, AxiosResponse } from "axios";
import { NextRequest } from "next/server";
import { ZodError } from "zod";

export async function POST(req: NextRequest) {
  const body = await req.json();
  const serverUrl = getServerUrl();
  const errorResp: APIError = { message: "", status: 500 };
  if (!serverUrl) {
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }
  try {
    const organisation: CreateOrganisationSchema =
      createOrganisationSchema.parse(body);
    const res: AxiosResponse<Organisation> = await axios.post(
      serverUrl + "/organisations",
      organisation,
      { headers: { "X-User": "1" } },
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
        // Error type return from server:
        // [{erorrs: [{message: string, status: number, source: string]}]
        errorResp.message = JSON.stringify(resp.data.errors[0].message);
        errorResp.status = resp.status;
        return new Response(...errorToResp(errorResp));
      }
    }
    errorResp.message = "Internal Server Error";
    errorResp.status = 500;
    return new Response(...errorToResp(errorResp));
  }
}

export async function GET() {
  const serverUrl = getServerUrl();
  const errorResp: APIError = { message: "", status: 500 };
  if (!serverUrl) {
    errorResp.message = "Internal Server Error";
    return new Response(...errorToResp(errorResp));
  }

  try {
    const res: AxiosResponse<{ code: number; organisations: Organisation[] }> =
      await axios.get(serverUrl + "/organisations", {
        headers: { "X-User": "1" },
      });
    return new Response(JSON.stringify(res.data));
  } catch (err) {
    if (err instanceof AxiosError) {
      if (err.status === 400) {
        return new Response(err.message, { status: 400 });
      }
    }
    return new Response("Internal Server Error", { status: 500 });
  }
}
