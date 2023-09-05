import axios, { AxiosInstance } from "axios";

export function api(): AxiosInstance {
  const api = axios.create({
    baseURL: process.env.SERVER_URL,
  });
  return api;
}
