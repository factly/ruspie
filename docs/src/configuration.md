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