
const ruspieApiUrl = process.env.RUSPIE_API_URL || 'http://localhost:8080';

export const schemaURL = ruspieApiUrl + '/api/schema';
export const restEndpoint = ruspieApiUrl + '/api/tables';
export const graphqlEndpoint = ruspieApiUrl + '/api/graphql';
export const sqlEndpoint = ruspieApiUrl + '/api/sql';
