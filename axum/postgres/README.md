# Shuttle shared Postgres DB with Axum

This template shows how to connect a Postgres database and use it for a simple TODO list app.

## Example usage

```bash
curl -X POST -H 'content-type: application/json' localhost:8000/todos --data '{"note":"My todo"}'
# {"id":1,"note":"My todo"}

curl localhost:8000/todos/1
# {"id":1,"note":"My todo"}
```
