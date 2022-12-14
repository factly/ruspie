## Authorization
In Ruspie, authorization is enabled by setting the `MASTER_KEY` environment variable. Once this variable is set, users must provide a valid key in the `AUTHORIZATION` header of their request in order to access the Ruspie APIs.

Ruspie provides a set of endpoints for managing keys, which can be accessed by making a request to the `/auth/keys` endpoint. This endpoint supports the following methods:

1. `GET`: lists all keys that have been generated.
2. `POST`: generates a new key.
3. `PATCH` /{key_id}: edits the name and description of a key.
4. `DELETE` /{key_id}: deletes a key.
5. `POST` /invalidate/{key_id}: invalidates a key, rendering it no longer valid for authentication.

To access these endpoints, users must provide the AUTHORIZATION header with their request, in the form `Bearer {MASTER_KEY}`, where `{MASTER_KEY}` is the value of the `MASTER_KEY` environment variable.

Once a key has been generated, it can be used to authenticate requests to the Ruspie APIs by providing it in the `AUTHORIZATION` header, in the form `Bearer {key}`, where `{key}` is the generated key. For example, to query the `/api/tables/{table_name}` endpoint with a generated key, the request might look like this:

```bash
curl -H "AUTHORIZATION: Bearer {key}" localhost:8080/api/tables/{table_name}
````
Note that keys can be invalidated or deleted, in which case they will no longer be valid for authentication. Users should manage their keys carefully to ensure that only valid keys are used to access the Ruspie APIs.