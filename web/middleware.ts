import { NextResponse, type NextRequest } from "next/server";
export function middleware(req: NextRequest) {
  return NextResponse.redirect(new URL("/home", req.url));
}

// See "Matching Paths" below to learn more
export const config = {
  matcher: ["/"],
};
