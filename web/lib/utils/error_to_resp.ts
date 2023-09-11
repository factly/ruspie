import { APIError } from "@/types/api_error";

export function errorToResp(error: APIError): [string, { status: number }] {
  return [JSON.stringify(error), { status: error.status }];
}
