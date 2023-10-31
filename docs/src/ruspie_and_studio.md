### Ruspie + Studio

Ruspie + Studio is a configuration of the Ruspie that combines the core Ruspie query engine with the Ruspie Studio web interface. This version offers a environment for managing datasets, querying data without any authentication and very lightweight compared to the previos configuration.

#### Setup and Configuration

To get started with Ruspie + Studio + Kavach, follow these steps:

1. **Run Docker Compose**: Navigate to the `/docker-compose-files/` directory and run the `docker-compose-studio.yml` file to start the integrated environment.

2. **Access Web Interface**: Access Ruspie Studio via `http://127.0.0.1:3000/home/organisations` for project and data management. Access the Ruspie API at `http://127.0.0.1:8800/server` for querying datasets.

**Note: If you need text to sql service in the above two setups, use the same envs as mentioned above to enable it on this setup as well**
