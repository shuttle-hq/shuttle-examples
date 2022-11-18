# Postgres with POEM
The following example is to present a way to use [poem](https://docs.rs/poem/latest/poem/) framework and devlop a relational database for a TODO app using [Postgres](https://www.postgresql.org/docs/).

This app uses the `shuttle service` attribute and `sqlx` to build a database and define its CRUD operations.

## Structure
This example has one file for development of the database and operations associated with it.

### src/lib.rs
Three main functions are included in this file for the development and operations of on the database.
1. `retrieve` this function is used to retrieves the TODOs from the database
2. `add` this function adds a TODO list in the database
3. `main` the main function executes the schema of the database allowing for its creation




