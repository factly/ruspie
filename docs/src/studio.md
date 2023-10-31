### Studio(Optional)

Simple web interface built with nextjs and golang enables to create organisations and projects to upload datasets and query using ruspie.

Note: To run the server make sure to copy the envs from config.env.example in /studio/server/ directory into config.env file

#### Environment Variables for server component

- **SERVER_PORT**: The port on which the Ruspie Studio Server will listen for incoming requests. The default is set to `8800`.

- **DATABASE_HOST**: The host name or IP address of the PostgreSQL database used by the server. In your setup, it's configured to `postgres`.

- **DATABASE_PORT**: The port on which the PostgreSQL database is running. It's set to the default `5432`.

- **DATABASE_USERNAME**: The username used to authenticate with the PostgreSQL database. In your configuration, it's set to `postgres`.

- **DATABASE_PASSWORD**: The password associated with the specified database username (`DATABASE_USERNAME`). In your setup, it's configured as `postgres`.

- **DATABASE_NAME**: The name of the PostgreSQL database where Ruspie Studio Server stores its data. It's set to `ruspie` in your configuration.

- **DATABASE_SSLMODE**: This variable specifies the SSL mode for the PostgreSQL database connection. In your setup, it's set to `disable`, which means SSL is not used.

- **LOG_LEVEL**: The log level for server logs. It's set to `debug` in your configuration, providing detailed log information.

- **LOG_OUTPUT**: The output destination for logs. In your setup, it's configured to `stdout`, which means logs are directed to the standard output.

- **KAVACH_ENABLED**: This variable enables or disables integration with Kavach, an internal authentication and authorization tool.

##### Configuration Note

To configure the Ruspie Studio Server, you should adjust these environment variables to match your specific setup. Ensure that the database connection details (`DATABASE_HOST`, `DATABASE_PORT`, `DATABASE_USERNAME`, `DATABASE_PASSWORD`, and `DATABASE_NAME`) are correctly set to connect to your PostgreSQL database.

Additionally, if you decide to enable Kavach integration, set `KAVACH_ENABLED` to `true` and configure the Kavach-related environment variables accordingly.

#### Environment Variables for frontend component

- **NEXT_PUBLIC_SERVER_URL**: This variable defines the URL for accessing the Ruspie Studio Server. This URL is used for making API requests to the server.

- **NEXT_PUBLIC_COMPANION_URL**: The Companion URL of companion service

- **NEXT_PUBLIC_KAVACH_ENABLED**: This variable enables or disables integration with Kavach, an internal authentication and authorization tool..

- **NEXT_PUBLIC_BASEPATH**: The Base Path is set to `/.factly/ruspie/web` in `KAVACH+STUDIO+RUSPIE` configurations. It defines the base URL path for accessing the Ruspie Studio web interface.

- **NEXT_PUBLIC_S3_URL**: This variable specifies the URL for accessing the S3 storage service.

- **NEXT_PUBLIC_TEXT_TO_SQL**: The `TEXT_TO_SQL` environment variable is referenced here. Depending on its value, Text to SQL functionality may be enabled or disabled in the frontend.

- **NEXT_PUBLIC_KRATOS_URL**: The Kratos URL .
