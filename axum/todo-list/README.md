# Todo list app on Shuttle with Axum and Postgres

This template combines a Postgres database with a file server to make a simple Todo list app.

## How to use

Use `shuttle run` to try it locally, and access the UI on <http://127.0.0.1:8000>.

Then you can use `shuttle deploy` to deploy it.

## Example command line usage (local run)

```bash
curl -X POST -H 'content-type: application/json' localhost:8000/api/todos --data '{"content":"My todo"}'
# {"id":1,"content":"My todo"}

curl localhost:8000/api/todos
# [{"id":1,"content":"My todo"}]

curl -X PUT -H 'content-type: application/json' localhost:8000/api/todos/1 --data '{"content":"Updated text"}'
# {"id":1,"content":"Updated text"}

curl -X DELETE localhost:8000/api/todos/1
# {"id":1,"content":"Updated text"}

curl localhost:8000/api/todos
# []
```
