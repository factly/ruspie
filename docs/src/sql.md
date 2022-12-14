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
