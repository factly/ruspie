import { restEndpoint, schemaURL } from "@/lib/constants/apiEndpoints";
import { File } from "@/types/file";
import { getNameFromUrl } from "./getNameFromUrl";
import axios from "axios";

export const fetchSchemaForTable = async (dataset: File, json = false) => {
  const name = getNameFromUrl(dataset.s3_url);

  const response = await fetch(`${schemaURL}/${name}`);

  if (!response.ok) {
    throw new Error(`Failed to fetch schema: ${response.status}`);
  }
  if (json) {
    return response.json();
  }
  return response.text();
};

export const fetchRowsForTable = async (dataset: File, limit = 10) => {
  const name = getNameFromUrl(dataset.s3_url);

  const response = await axios.get(`${restEndpoint}/${name}?limit=${limit}`, {
    headers: {
      "FILE-EXT": dataset.extension,
    },
  });

  if (response.status === 400) {
    throw new Error(`Failed to table rows: ${response.status}`);
  }

  return response.data;
};
