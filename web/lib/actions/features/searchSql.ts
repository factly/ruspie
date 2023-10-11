import { sqlEndpoint } from "@/lib/constants/apiEndpoints";
import axios from "axios";

export const fetchSqlForQuery = async (query: string, extension: string) => {
  const response = await axios.post(sqlEndpoint, query, {
    headers: {
      "FILE-EXT": extension,
      "Content-Type": "text/plain",
    },
  });

  console.log(response.status);

  if (response.status != 200) {
    throw new Error(`Failed to fetch SQL: ${response.status}`);
  }

  return response.data;
};
