### TEXT-TO-SQL SERVICE (Deprecated)

**Note: The Text to SQL service that was previously deployed as a cloudflare worker has been deprecated. The functionality has been integrated into the Ruspie API as an optional endpoint. This section provides information about the deprecated service for reference.**

#### Introduction

The Text to SQL functionality allowed users to convert natural language text queries into valid SQL queries. It provided a convenient way to interact with your dataset using plain English queries, making it accessible to users who may not be familiar with SQL.

#### How it Worked

The Text to SQL service was a standalone component that accepted a text query and other parameters as input. It returned a valid SQL query that could be executed against the dataset stored in Ruspie. This service was deployed as a cloudflare worker.

#### Deprecated Cloudflare Worker Endpoint

The Text to SQL service was previously available as a cloudflare worker endpoint. Users could send a POST request to this endpoint, providing the necessary input parameters, and receive a valid SQL query as the response.

**Note: You can still refer the code to this service text-to-sql directory**

#### Transition to Ruspie API

As of the latest version, the Text to SQL functionality has been deprecated as a cloudflare worker and is no longer maintained as a separate component. The functionality of translating text queries into SQL queries is now available as an optional endpoint within the Ruspie API. Users can use the `/text_to_sql` endpoint to achieve the same results.

#### Updated Workflow

To enable the optional endpoint set the following envs:

1. TEXT_TO_SQL: true
2. OPENAI_API_KEY: openai api key(get one from openai)

#### Making a Request to `/text_to_sql` Endpoint

The `/text_to_sql` endpoint in the Ruspie API allows you to convert natural language text queries into valid SQL queries. To make a request to this endpoint, follow these steps:

#### 1. Set the HTTP Method

Send a POST request to the `/text_to_sql` endpoint. This is because you are sending data to the server to process.

#### 2. Set the Request Headers

Ensure that your request includes the necessary headers:

- **Content-Type**: Set this header to `application/json` to specify that the request body contains JSON data.

#### 3. Create the Request Body

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

#### 4. Send the Request

Once you have set the HTTP method, headers, and created the request body, you can send the POST request to the `/text_to_sql` endpoint.

#### 5. Receive the Response

The server will process your request and respond with a JSON object that contains the valid SQL query. The response may also include additional information, such as a status indicating the success of the operation.

Here's an example of a response:

```json
{
  "query": "SELECT customer_name FROM customer_purchases WHERE purchase_date >= '2023-09-01'"
}
```

The `sql_query` field in the response contains the valid SQL query that can be used to retrieve the requested data from the dataset.

That's it! You've successfully made a request to the `/text_to_sql` endpoint and received a valid SQL query in response, allowing you to perform dataset queries using natural language text.
