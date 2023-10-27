# RUSPIE

## INTRODUCTION

Ruspie is a query engine for datasets stored in CSV and Parquet formats. It allows you to query your data using SQL, REST API, and GraphQL. Ruspie is built on top of Apache Arrow and Datafusion, and it is written in Rust.

To use Ruspie, you can start the server by running the `cargo run` command, and specifying the path to your dataset files using the `FILE_PATH` environment variable. You can then send queries(SQL, GraphQL, REST Query params) to the server using the REST API. Ruspie supports a variety of query operators, such as filtering, sorting, and limiting the number of results, which can be specified in the query.

## QUICKSTART

To quickly setup Ruspie, follow these steps:

Install Rust, if you don't already have it installed. You can do this by following the instructions on the Rust website: https://www.rust-lang.org/tools/install.

Clone the Ruspie repository from GitHub: git clone https://github.com/factly/ruspie.git

Navigate to the cloned repository and build the project using `cargo build`

Set the `FILE_PATH` environment variable to the path of the dataset files that you want to serve through the API. For example: `FILE_PATH=./data`

Start the Ruspie server by running `cargo run`

You can now send queries to the Ruspie server using the REST API, SQL, or GraphQL. For more information, see the documentation for the project.

Note: If you want to enable token-based authorization, you will need to set the `MASTER_KEY` environment variable before starting the server. This will enable authentication for all the endpoints, and you will need to pass a valid authorization token in the `AUTHORIZATION` header of your requests.

## CONFIGURATION

You can configure Ruspie using environment variables. The following environment variables are available:

`SOURCE`: This specifies the source of the files to be fetched from. It can either be `S3` or `FILESYSTEM`, by default it is set to `FILESYSTEM`.

`S3_PATH`: If the `SOURCE` env variable is set to `S3`, you have to spicify this env variable. `S3_PATH` can also be passed from request headers which overrides the value set in env. If the env variable is not set it is defaulted to `ruspie/`.Ex:

```bash
# if your a have s3 url bucket/path/to/file.csv
export SOURCE=S3
export S3_PATH=bucket/path/to
cargo run
# In this request it will serve from s3://bucket/path/to/blogs.csv
curl -H "FILE-EXT: csv"  http://localhost:8080/api/tables/blogs
# In this request it will override env value as serve from s3://newbucket/newpath/to/blogs.csv
curl -H "S3_PATH: newbucket/newpath/to" http://localhost:8080/api/tables/blogs
```

`FILE_PATH`: If `SOURCE` is set to `FILESYSTEM` this specifies the path to the dataset files that you want to serve through the API. If not set, the default is the test directory in the root of the project.

`PORT`: This specifies the port that the Ruspie server will listen on. If not set, the default is 8080.

`MASTER_KEY`: This enables token-based authorization for all endpoints. If not set, authentication is disabled.

`LIMIT`: This specifies the default limit on the number of results returned by a query. If not set, the default is 100.

`MAX_LIMIT`: This specifies the maximum limit that can be specified in a query. If not set, the default is 1000.

`DEFAULT_EXT`: This specifies the default file extension that Ruspie will look for when serving files through the API. If not set, the default is csv.

To set an environment variable, you can use the export command before starting the Ruspie server. For example, to set the FILE_PATH variable to the data directory and the PORT variable to 8080, you can run the following commands:

```bash
export SOURCE=FILESYSTEM
export FILE_PATH=./data
export PORT=8080
cargo run
```

You can also set the environment variables in your shell configuration file (e.g. .bashrc or .zshrc) so that they are automatically set when you open a new terminal. For more information, see the documentation for your shell.

## ENDPOINTS

Ruspie exposes the following endpoints:

1. GET /api/tables/{table_name}: This endpoint allows you to query a dataset using the REST query params. You can specify query operators such as filters, sorting, and limits in the URL query parameters.

2. POST /api/sql: This endpoint allows you to query a dataset using SQL queries. You can pass the SQL query in the request body as plain text.

3. POST /api/graphql: This endpoint allows you to query a dataset using GraphQL queries. You can pass the GraphQL query in the request body as plain text.

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

### PREFETCHING WITH ROBINPIE

Prefetching is a technique used in Ruspie to optimize schema infereing process. Robinpie is a component that prefetches dataset file schemas from ruspie and then stores them in a specified source. This is only when the `SOURCE` env is set to `S3`.

#### Environment Variables

- `PRE_FETCH_ENABLE`: set to true to enable prefetching (Default is false).
- `RUSPIE_PREFETCH_INTERVAL`: specifies how often Ruspie should fetch schemas from Source(Default is 60s).
- `ROBINPIE_PREFETCH_INTERVAL`: specifies how often Robinpie should fetch schemas from the Ruspie(Default is 30s).
- `PRE_FETCH_SOURCE`: specifies the source for fetching schemas (S3, Mongo, or the filesystem) (Default is Mongo).
- `MONGO_URI`: specifies the URI for MongoDB when using it as the source.

#### Working of Robinpie

Robinpie fetches all dataset file schemas and stores them in the specified source, creating `schemas.json/schemas.parquet` when set to `S3` it uses same `S3_PATH` . On restart, Ruspie loads schemas from the source instead of inferring them. Robinpie also periodically fetches schemas from Ruspie and updates the source for new files.

## AUTHORIZATION

In Ruspie, authorization is enabled by setting the `MASTER_KEY` environment variable. Once this variable is set, users must provide a valid key in the `AUTHORIZATION` header of their request in order to access the Ruspie APIs.

Ruspie provides a set of endpoints for managing keys, which can be accessed by making a request to the `/auth/keys` endpoint. This endpoint supports the following methods:

1. `GET`: lists all keys that have been generated.
2. `POST`: generates a new key.
3. `PATCH` /{key_id}: edits the name and description of a key.
4. `DELETE` /{key_id}: deletes a key.
5. `POST` /invalidate/{key_id}: invalidates a key, rendering it no longer valid for authentication.

To access these endpoints, users must provide the AUTHORIZATION header with their request, in the form `Bearer {MASTER_KEY}`, where `{MASTER_KEY}` is the value of the `MASTER_KEY` environment variable.

Once a key has been generated, it can be used to authenticate requests to the Ruspie APIs by providing it in the `AUTHORIZATION` header, in the form `Bearer {key}`, where `{key}` is the generated key. For example, to query the `/api/tables/{table_name}` endpoint with a generated key, the request might look like this:

```bash
curl -H "AUTHORIZATION: Bearer {key}" localhost:8080/api/tables/{table_name}
```

Note that keys can be invalidated or deleted, in which case they will no longer be valid for authentication. Users should manage their keys carefully to ensure that only valid keys are used to access the Ruspie APIs.

# Text to SQL Service (Deprecated)

**Note: The Text to SQL service that was previously deployed as a cloudflare worker has been deprecated. The functionality has been integrated into the Ruspie API as an optional endpoint. This section provides information about the deprecated service for reference.**

## Introduction

The Text to SQL functionality allowed users to convert natural language text queries into valid SQL queries. It provided a convenient way to interact with your dataset using plain English queries, making it accessible to users who may not be familiar with SQL.

## How it Worked

The Text to SQL service was a standalone component that accepted a text query and other parameters as input. It returned a valid SQL query that could be executed against the dataset stored in Ruspie. This service was deployed as a cloudflare worker.

## Deprecated Cloudflare Worker Endpoint

The Text to SQL service was previously available as a cloudflare worker endpoint. Users could send a POST request to this endpoint, providing the necessary input parameters, and receive a valid SQL query as the response.

**Note: You can still refer the code to this service text-to-sql directory**

## Transition to Ruspie API

As of the latest version, the Text to SQL functionality has been deprecated as a cloudflare worker and is no longer maintained as a separate component. The functionality of translating text queries into SQL queries is now available as an optional endpoint within the Ruspie API. Users can use the `/text_to_sql` endpoint to achieve the same results.

## Updated Workflow

To enable the optional endpoint set the following envs:

1. TEXT_TO_SQL: true
2. OPENAI_API_KEY: openai api key(get one from openai)

### Making a Request to `/text_to_sql` Endpoint

The `/text_to_sql` endpoint in the Ruspie API allows you to convert natural language text queries into valid SQL queries. To make a request to this endpoint, follow these steps:

### 1. Set the HTTP Method

Send a POST request to the `/text_to_sql` endpoint. This is because you are sending data to the server to process.

### 2. Set the Request Headers

Ensure that your request includes the necessary headers:

- **Content-Type**: Set this header to `application/json` to specify that the request body contains JSON data.

### 3. Create the Request Body

The request body should be in JSON format and include the following parameters:

- **query**: A string parameter that represents the natural language text query you want to convert into SQL. Provide a clear and concise description of the data you want to retrieve from the dataset.

- **tablename**: A string parameter specifying the name of the dataset table on which you want to perform the query.

- **schema** (Optional): If the schema of the dataset is not already known or needs to be explicitly defined, you can include it as a string parameter. The schema should list the columns available in the dataset.

- **rows** (Optional): Intial rows of datasets.

Here is an example of the request body in JSON format:

```json
{
  "query": "Retrieve the names of customers who made a purchase in the last month",
  "tablename": "customer_purchases",
  "schema": "customer_name, purchase_date, ...",
  "rows": "10"
}
```

### 4. Send the Request

Once you have set the HTTP method, headers, and created the request body, you can send the POST request to the `/text_to_sql` endpoint.

### 5. Receive the Response

The server will process your request and respond with a JSON object that contains the valid SQL query. The response may also include additional information, such as a status indicating the success of the operation.

Here's an example of a response:

```json
{
  "sql_query": "SELECT customer_name FROM customer_purchases WHERE purchase_date >= '2023-09-01'",
  "status": "success"
}
```

The `sql_query` field in the response contains the valid SQL query that can be used to retrieve the requested data from the dataset.

That's it! You've successfully made a request to the `/text_to_sql` endpoint and received a valid SQL query in response, allowing you to perform dataset queries using natural language text.
