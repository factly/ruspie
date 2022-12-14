## ENDPOINTS
Ruspie exposes the following endpoints:

1. `GET /api/tables/{table_name}`: This endpoint allows you to query a dataset using the REST query params. You can specify query operators such as filters, sorting, and limits in the URL query parameters.

2. `POST /api/sql`: This endpoint allows you to query a dataset using SQL queries. You can pass the SQL query in the request body as plain text.

3. `POST /api/graphql`: This endpoint allows you to query a dataset using GraphQL queries. You can pass the GraphQL query in the request body as plain text.

4. `GET /api/schemas/{table_name}`: This endpoint returns the schema of the specified dataset.

Note: If you have enabled token-based authorization, these endpoints will require a valid authorization token to be passed in the `AUTHORIZATION` header of the request.

Additionally, the following endpoints are available for managing API keys when token-based authorization is enabled:

1. `GET /auth/keys`: This endpoint allows you to list all the API keys that have been generated.

2. `POST /auth/keys`: This endpoint allows you to generate a new API key.

3. `PATCH /auth/keys/{key_id}`: This endpoint allows you to update the name and description of an existing API key.

4. `DELETE /auth/keys/{key_id}`: This endpoint allows you to delete an existing API key.

5. `POST /auth/keys/invalidate/{key_id}`: This endpoint allows you to invalidate an existing API key, preventing it from being used to access the API.