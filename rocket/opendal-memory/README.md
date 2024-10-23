# OpenDAL Memory Storage

This example shows that how to connect to an in-memory storage using OpenDAL.

## Project structure

The project consists of the following files

- `Shuttle.toml` contains the name of the app
- `src/main.rs` is where all the magic happens - it creates a Shuttle service with two endpoints: one for adding new data and one for retrieving it back.

## How to use this template

Run the project, then try out the following `curl` command:

```bash
curl -X POST -H 'content-type: application/json' localhost:8000/todo --data '{"note":"My todo"}'
# {"id":1,"note":"My todo"}
```
