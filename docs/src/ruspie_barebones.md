### Ruspie Barebones

Ruspie is the core query engine of the Ruspie ecosystem, designed for data querying using SQL and various query operators.

### Setup and Configuration

To set up and configure Ruspie, follow these steps:

1. **Environment Configuration**: Configure the necessary environment variables for Ruspie. These variables control various aspects of Ruspie's behavior, including the source of data and authentication.

2. **Run Docker Compose**: .Navigate to the `/docker-compose-files/` directory and run the `docker-compose.yml` file to start the integrated environment

3. **Dataset Management**: Upload datasets to Minio for use with Ruspie.

To Upload datasets to Minio goto http://127.0.0.9001 and use `MINIO_ACCESS_KEY`, `MINIO_SECRET_KEY` used the docker-compose.yml file as username and password. As per the config in docker-compose.yml, datasets uploaded to the ruspie bucker can only be queried.
