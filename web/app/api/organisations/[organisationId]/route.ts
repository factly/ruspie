import { errorToResp } from "@/lib/utils/error_to_resp";
import { getServerUrl } from "@/lib/utils/serverUrl";
import { APIError } from "@/types/api_error";
import { Organisation } from "@/types/organisation";
import axios, { AxiosError, AxiosResponse } from "axios";

export async function GET(
  _req: Request,
  { params }: { params: { organisationId: string } },
) {
  const { organisationId } = params;
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
      {
        headers: {
          "X-User": "1",
        },
      },
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
