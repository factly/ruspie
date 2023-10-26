const kavachEnabled = process.env.NEXT_PUBLIC_KAVACH_ENABLED === "true";

export const kavachAxiosConfig = {
  withCredentials: kavachEnabled,
};
