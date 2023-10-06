import { textToSqlEndpoint } from "@/lib/constants/apiEndpoints";

type TextToSqlParams = {
  table: string;
  schema: string | null;
  query: string | null;
  rows: string | null;
};

export const convertTextToSql = async ({
  table,
  schema,
  query,
  rows,
}: TextToSqlParams) => {
  const headers = new Headers();
  headers.append("Content-Type", "application/json");

  const body = JSON.stringify({
    query: query,
    schema: schema,
    tablename: table,
    rows: JSON.stringify(rows),
  });
  console.log(body);

  const requestOptions: RequestInit = {
    method: "POST",
    headers: headers,
    body: body,
    redirect: "follow",
  };

  const response = await fetch(textToSqlEndpoint, requestOptions);

  if (!response.ok) {
    throw new Error(`Failed to convert text to SQL: ${response.status}`);
  }

  return response.json();
};
