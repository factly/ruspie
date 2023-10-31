### Ruspie + Studio + Kavach

Ruspie + Studio + Kavach is a configuration of the Ruspie that combines the core Ruspie query engine with the Ruspie Studio web interface and the Kavach authentication and authorization tool. This version offers a environment for managing datasets, querying data, and controlling access.

#### Setup and Configuration

To get started with Ruspie + Studio + Kavach, follow these steps:

1. **Run Docker Compose**: Navigate to the `/docker-compose-files/` directory and run the `docker-compose-studio-kavach.yml` file to start the integrated environment.

2. **Kavach Integration**: Ensure that Kavach is configured with the required user roles and permissions for secure access control.

3. **Access Web Interface**: Access Ruspie Studio via `http://127.0.0.1:4455/.factly/ruspie/web/home/organisations` for project and data management. Access the Ruspie API at `http://127.0.0.1:4455/.factly/ruspie/server` for querying datasets.
