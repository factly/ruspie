import { restEndpoint, schemaURL } from '@/lib/constants/apiEndpoints';

export const fetchSchemaForTable = async (table: string, json = false) => {
	const headers = new Headers();
	headers.append('FILE-EXT', 'csv');
  const requestOptions : RequestInit= {
    method: 'GET',
		headers,
    redirect: 'follow',
  };

  const response = await fetch(`${schemaURL}/${table}`, requestOptions);

  if (!response.ok) {
    throw new Error(`Failed to fetch schema: ${response.status}`);
  }
  if (json) {
    return response.json();
  }
  return response.text();
};

export const fetchRowsForTable = async (table : string, limit = 20) => {
  const requestOptions :RequestInit = {
    method: 'GET',
    redirect: 'follow',
  };

  const response = await fetch(`${restEndpoint}/${table}?limit=${limit}`, requestOptions);

  if (!response.ok) {
    throw new Error(`Failed to table rows: ${response.status}`);
  }

  return response.text();
};
