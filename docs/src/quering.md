# Querying
Ruspie Supports SQL, GraphQL, and querying with query params.

### NOTE
To override the `DEFAULT_EXT` use `FILE-EXT` header while querying. For example, to query a dataset in Parquet format, the `FILE-EXT` header have to be set to parquet:

```bash
curl -H "FILE-EXT: parquet" localhost:8080/api/tables/{table_name}
curl -H "FILE-EXT: parquet" localhost:8080/api/sql
```