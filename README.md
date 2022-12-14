# RUSPIE
## INTRODUCTION
Ruspie is a query engine for datasets stored in CSV and Parquet formats. It allows you to query your data using SQL, REST API, and GraphQL. Ruspie is built on top of Apache Arrow and Datafusion, and it is written in Rust.

To use Ruspie, you can start the server by running the cargo run command, and specifying the path to your dataset files using the `FILE_PATH` environment variable. You can then send queries to the server using the REST API, SQL, or GraphQL. Ruspie supports a variety of query operators, such as filtering, sorting, and limiting the number of results, which can be specified in the query.



## QUICKSTART
To quickly setup Ruspie, follow these steps:

Install Rust, if you don't already have it installed. You can do this by following the instructions on the Rust website: https://www.rust-lang.org/tools/install.

Clone the Ruspie repository from GitHub: git clone https://github.com/factly/ruspie.git

Navigate to the cloned repository and build the project using cargo build

Set the `FILE_PATH` environment variable to the path of the dataset files that you want to serve through the API. For example: `FILE_PATH=./data`

Start the Ruspie server by running `cargo run`

You can now send queries to the Ruspie server using the REST API, SQL, or GraphQL. For more information, see the documentation for the project.

Note: If you want to enable token-based authorization, you will need to set the `MASTER_KEY` environment variable before starting the server. This will enable authentication for all the endpoints, and you will need to pass a valid authorization token in the `AUTHORIZATION` header of your requests.

## CONFIGURATION
You can configure Ruspie using environment variables. The following environment variables are available:

`FILE_PATH`: This specifies the path to the dataset files that you want to serve through the API. If not set, the default is the test directory in the root of the project.

`PORT`: This specifies the port that the Ruspie server will listen on. If not set, the default is 8080.

`MASTER_KEY`: This enables token-based authorization for all endpoints. If not set, authentication is disabled.

`LIMIT`: This specifies the default limit on the number of results returned by a query. If not set, the default is 100.

`MAX_LIMIT`: This specifies the maximum limit that can be specified in a query. If not set, the default is 1000.

`DEFAULT_EXT`: This specifies the default file extension that Ruspie will look for when serving files through the API. If not set, the default is csv.

To set an environment variable, you can use the export command before starting the Ruspie server. For example, to set the FILE_PATH variable to the data directory and the PORT variable to 8080, you can run the following commands:

```bash
export FILE_PATH=./data
export PORT=8080
cargo run
```
You can also set the environment variables in your shell configuration file (e.g. .bashrc or .zshrc) so that they are automatically set when you open a new terminal. For more information, see the documentation for your shell.

## ENDPOINTS
Ruspie exposes the following endpoints:

1. GET /api/tables/{table_name}: This endpoint allows you to query a dataset using the REST API. You can specify query operators such as filters, sorting, and limits in the URL query parameters.

2. POST /api/sql: This endpoint allows you to query a dataset using SQL. You can pass the SQL query in the request body as plain text.

3. POST /api/graphql: This endpoint allows you to query a dataset using GraphQL. You can pass the GraphQL query in the request body as plain text.

4. GET /api/schemas/{table_name}: This endpoint returns the schema of the specified dataset.

Note: If you have enabled token-based authorization, these endpoints will require a valid authorization token to be passed in the AUTHORIZATION header of the request.

Additionally, the following endpoints are available for managing API keys when token-based authorization is enabled:

1. GET /auth/keys: This endpoint allows you to list all the API keys that have been generated.

2. POST /auth/keys: This endpoint allows you to generate a new API key.

3. PATCH /auth/keys/{key_id}: This endpoint allows you to update the name and description of an existing API key.

4. DELETE /auth/keys/{key_id}: This endpoint allows you to delete an existing API key.

5. POST /auth/keys/invalidate/{key_id}: This endpoint allows you to invalidate an existing API key, preventing it from being used to access the API.

## QUERYING
### REST

To query a dataset using query parameters in Ruspie, you can send a `GET` request to the `/api/tables/{table_name}` endpoint, where `table_name` is the name of the dataset you want to query. You can then specify the query operators in the URL query parameters.

For example, the following request will return the first 10 rows of the customers dataset, sorted in ascending order by the last_name column:

```bash
curl "localhost:8080/api/tables/customers?sort=last_name&limit=10"
```
You can also use the filter parameter to specify a filter condition. For example, the following request will return all rows from the customers dataset where the first_name column is equal to John:

```bash
curl "localhost:8080/api/tables/customers?filter[first_name]=John"
```
You can use the page and limit parameters to paginate the results. For example, the following request will return the second page of results, where each page contains 10 rows:

```bash
curl "localhost:8080/api/tables/customers?page=2&limit=10"
```
The `/api/tables/{table_name}` endpoint supports the following query parameters:


1. `columns`: specifies which columns of the dataset to include in the response.

2. `sort`: specifies the order in which the rows of the dataset should be sorted. This parameter can be used to sort the rows in ascending or descending order by one or more columns.

3. `limit`: specifies the maximum number of rows to include in the response.

4. `filter`: specifies a condition that rows must satisfy in order to be included in the response.
5. `page`: specifies which page of the dataset to include in the response, when pagination is used.
For example, to sort the rows of the dataset by the col1 and col2 columns in ascending and descending order, respectively, and return only the first 100 rows, the query might look like this:

```
/api/tables/{table_name}?sort=col1,-col2&limit=100
```
To filter the rows of the dataset to only include those where the col1 column has the value foo, the query might look like this:

```
/api/tables/{table_name}?filter[col1]=foo
```
And to retrieve the second page of the dataset, with 10 rows per page, the query might look like this:

```
/api/tables/{table_name}?page=2&limit=10
```
These query parameters can be combined in various ways to retrieve the desired subset of the dataset.






### SQL
To query a dataset using SQL in Ruspie, you can send a `POST` request to the `/api/sql` endpoint, and pass the SQL query in the request body as plain text.

For example, the following request will return the first 10 rows of the customers dataset, sorted in ascending order by the last_name column:

```bash
curl -X POST -d "SELECT * FROM customers ORDER BY last_name ASC LIMIT 10" localhost:8080/api/sql
```
You can also use the WHERE clause in your SQL query to specify a filter condition. For example, the following request will return all rows from the customers dataset where the first_name column is equal to John:

```bash
curl -X POST -d "SELECT * FROM customers WHERE first_name = 'John'" localhost:8080/api/sql
```

### GRAPHQL
To query a dataset using GraphQL in Ruspie, you can send a `POST` request to the `/api/graphql` endpoint, and pass the GraphQL query in the request body as plain text.

For example, the following request will return the first 10 rows of the customers dataset, sorted in ascending order by the last_name column:

```bash
curl -X POST -d "query { customers(sort: [{ field: "last_name", order: "asc" }], limit: 10) { id,first_name,last_name } }" localhost:8080/api/graphql
```
You can also use the filter parameter in your GraphQL query to specify a filter condition. For example, the following request will return all rows from the customers dataset where the first_name column is equal to John:

```bash
curl -X POST -d "query { customers(filter: { first_name: "John" }) { id,first_name,last_name } }" localhost:8080/api/graphql
```

### NOTE
To override the `DEFAULT_EXT` use `FILE-EXT` header while querying. For example, to query a dataset in Parquet format, the `FILE-EXT` header have to be set to parquet:

```bash
curl -H "FILE-EXT: parquet" localhost:8080/api/tables/{table_name}
```

## AUTHORIZATION
In Ruspie, authorization is enabled by setting the `MASTER_KEY` environment variable. Once this variable is set, users must provide a valid key in the `AUTHORIZATION` header of their request in order to access the Ruspie APIs.

Ruspie provides a set of endpoints for managing keys, which can be accessed by making a request to the `/auth/keys` endpoint. This endpoint supports the following methods:

1. `GET`: lists all keys that have been generated.
2. `POST`: generates a new key.
3. `PATCH` /{KEY_ID}: edits the name and description of a key.
4. `DELETE` /{KEY_ID}: deletes a key.
5. `POST` /invalidate/{key_id}: invalidates a key, rendering it no longer valid for authentication.

To access these endpoints, users must provide the AUTHORIZATION header with their request, in the form `Bearer {MASTER_KEY}`, where `{MASTER_KEY}` is the value of the `MASTER_KEY` environment variable.

Once a key has been generated, it can be used to authenticate requests to the Ruspie APIs by providing it in the `AUTHORIZATION` header, in the form `Bearer {key}`, where `{key}` is the generated key. For example, to query the `/api/tables/{table_name}` endpoint with a generated key, the request might look like this:

```bash
curl -H "AUTHORIZATION: Bearer {key}" localhost:8080/api/tables/{table_name}
````
Note that keys can be invalidated or deleted, in which case they will no longer be valid for authentication. Users should manage their keys carefully to ensure that only valid keys are used to access the Ruspie APIs.