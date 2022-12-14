
## QUICKSTART
To quickly setup Ruspie, follow these steps:

Install Rust, if you don't already have it installed. You can do this by following the instructions on the Rust website: `https://www.rust-lang.org/tools/install`.

Clone the Ruspie repository from GitHub: git clone `https://github.com/factly/ruspie.git`

Navigate to the cloned repository and build the project using `cargo build`

Set the `FILE_PATH` environment variable to the path of the dataset files that you want to serve through the API. For example: `FILE_PATH=./data`

Start the Ruspie server by running `cargo run`

You can now send queries to the Ruspie server using the REST API, SQL, or GraphQL. For more information, see the documentation for the project.

Note: If you want to enable token-based authorization, you will need to set the `MASTER_KEY` environment variable before starting the server. This will enable authentication for all the endpoints, and you will need to pass a valid authorization token in the `AUTHORIZATION` header of your requests.