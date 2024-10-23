# Actix Web with Shuttle Shared DB (Postgres)

This template shows how to connect a Postgres database and use it for a simple TODO list app.

## How to use this template:

Run the project then run the following `curl` commands below:

```bash
curl -X POST -H 'content-type: application/json' localhost:8000/todos --data '{"note":"My todo"}'
# {"id":1,"note":"My todo"}

curl localhost:8000/todos/1
# {"id":1,"note":"My todo"}
```
