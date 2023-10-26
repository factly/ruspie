export function getBasepathUrl(): string {
  if (process.env.NEXT_PUBLIC_KAVACH_ENABLED === "true") {
    return process.env.NEXT_PUBLIC_BASEPATH || "";
  }
  return "";
}
