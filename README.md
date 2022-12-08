# ruspie
Ruspie is a service that allows users to create full-fledged APIs (REST, GraphQL and SQL) for slow moving datasets without writing any code. It is built on Rust and offers a range of features for building and managing APIs. Ruspie is lightweight, performant, and highly scalable, making it an ideal solution for creating APIs for datasets that change infrequently. It is easy to use and allows users to quickly and easily set up APIs for their data, providing a convenient and powerful tool for accessing and sharing data.

# QUICKSTART
Move to `ruspie` directory and then run the following command

```
cargo run
```

Set Environment Variable `FILE_PATH`, by default rupie looks into `test` directory if `FILE_PATH` is not passed

# CONFIGURABLE ENV VARIABLES
`FILE_PATH`: configure path to look for files to query
`PORT`: configure port to run the server
`MASTER_KEY`: configure this enables authorization on all endpoints
`LIMIT`: configure default limit on number of responses when query is doesn't pass limit
`MAX_LIMIT`: configure to cap limit parameter
`DEFAULT_EXT`: configure to default file extention



# rest api

```
/
    file.csv
    another_file.parquet
```

To query the a file called `file.csv` through restapi request

`curl "localhost:8080/api/tables/file"`

Note: add a limit query param along with the request while querying large files

example: `curl "localhost:8080/api/tables/file?limit=100"`

The default file extention is `csv`
you can change the file extention by passing `FILE-EXT` header with the request

example:
    `curl -H "FILE-EXT: parquet" "localhost:8080/api/tables/another_file"`