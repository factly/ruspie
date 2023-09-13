import { sqlEndpoint } from "@/lib/constants/apiEndpoints";

export const fetchSqlForQuery = async (query: string) => {
  const headers = new Headers();
  headers.append('FILE-EXT', 'csv');
  headers.append('Content-Type', 'text/plain');

  const requestOptions: RequestInit = {
    method: 'POST',
    headers: headers,
    body: query.replaceAll(';', ''),
    redirect: 'follow',
  };

  const response = await fetch(sqlEndpoint, requestOptions);

  if (!response.ok) {
    throw new Error(`Failed to fetch SQL: ${response.status}`);
  }

  return response.json();
};
