import { graphqlEndpoint } from '@/lib/constants/apiEndpoints';

export const fetchGraphqlQuery = async (query :BodyInit) => {
  var headers = new Headers();
  headers.append('Content-Type', 'text/plain');
  const requestOptions: RequestInit = {
    method: 'POST',
    headers: headers,
    body: query,
    redirect: 'follow',
  };
  const response = await fetch(graphqlEndpoint, requestOptions);
  if (!response.ok) {
    throw new Error(`Failed to fetch SQL: ${response.status}`);
  }
  return response.json();
};
