import { restEndpoint, schemaURL } from "@/lib/constants/apiEndpoints";
import { File } from "@/types/file";
import { getNameFromUrl } from "./getNameFromUrl";

export const fetchSchemaForTable = async (dataset: File, json = false) => {
  const name = getNameFromUrl(dataset.s3_url);

  const response = await fetch(`${schemaURL}/${name}`);

  console.log(response);
  if (!response.ok) {
    throw new Error(`Failed to fetch schema: ${response.status}`);
  }
  if (json) {
    return response.json();
  }
  return response.text();
};

export const fetchRowsForTable = async (table: string, limit = 20) => {
  const requestOptions: RequestInit = {
    method: "GET",
    redirect: "follow",
  };

  const response = await fetch(
    `${restEndpoint}/${table}?limit=${limit}`,
    requestOptions,
  );

  if (!response.ok) {
    throw new Error(`Failed to table rows: ${response.status}`);
  }

  return response.text();
};
