
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