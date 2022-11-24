# ruspie
Move to `ruspie` directory and create a directory called `test` and save csv and parquet files to that directory

Then run the following command

```
cargo run
```

# rest api

```
/test
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