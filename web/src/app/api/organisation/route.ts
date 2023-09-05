import { api } from "@/lib/utils/axios";
import {
  CreateOrganisationSchema,
  createOrganisationSchema,
} from "@/lib/zod/validators/organisation";
import { Organisation } from "@/types/organisation";
import { AxiosError, AxiosResponse } from "axios";
import { NextRequest } from "next/server";
import { ZodError } from "zod";

export async function POST(req: NextRequest) {
  const body = await req.json();
  const axios = api();
  try {
    const organisation: CreateOrganisationSchema =
      createOrganisationSchema.parse(body);
    const res: AxiosResponse<Organisation> = await axios.post(
      "/organisations",
      organisation,
    );
    return new Response(JSON.stringify(res.data), { status: 201 });
  } catch (err) {
    console.log(err);
    if (err instanceof ZodError) {
      return new Response(err.message, { status: 400 });
    }
    if (err instanceof AxiosError) {
      if (err.status === 400) {
        return new Response(err.message, { status: 400 });
      }
      return new Response("Internal Server Error", { status: 500 });
    }
    return new Response("Internal Server Error", { status: 500 });
  }
}

export async function GET() {
  const axios = api();

  try {
    const res: AxiosResponse<Organisation[]> = await axios.get(
      "/organisations",
    );
    return new Response(JSON.stringify(res.data));
  } catch (err) {
    if (err instanceof AxiosError) {
      if (err.status === 400) {
        return new Response(err.message, { status: 400 });
      }
      return new Response("Internal Server Error", { status: 500 });
    }
    return new Response("Internal Server Error", { status: 500 });
  }
}

export async function PUT(req: NextRequest) {
  const body = await req.json();
}
