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
