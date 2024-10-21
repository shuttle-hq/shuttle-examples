An example that showcases using Turso with Axum and Shuttle.

## Example usage

Set your `TURSO_DB_TOKEN` in Secrets.toml and Turso database in `src/main.rs` in the annotation.

Run the app, then try it out with the following `curl` command:

```sh
curl http://localhost:8000 -H 'content-type: application/json' --data '{"uid":"1","email":"foo@bar.xyz"}'
```
