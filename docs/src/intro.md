# RUSPIE
## INTRODUCTION
Ruspie is a query engine for datasets stored in CSV and Parquet formats. It allows you to query your data using SQL, REST API, and GraphQL. Ruspie is built on top of Apache Arrow and Datafusion, and it is written in Rust.

To use Ruspie, you can start the server by running the `cargo run` command, and specifying the path to your dataset files using the `FILE_PATH` environment variable. You can then send queries(SQL, GraphQL, REST Query params) to the server using the REST API. Ruspie supports a variety of query operators, such as filtering, sorting, and limiting the number of results, which can be specified in the query.