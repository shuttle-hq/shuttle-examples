# Todo App with tide and postgres
The following example is to present a way to use [tide](https://docs.rs/tide/latest/tide/) framework and develop a relational database for a TODO app using [Postgres](https://www.postgresql.org/docs/).

## Structure 
The example contains one folder `lib.rs` where the database and the CRUD operations are defined. 

### src/lib.rs
Three main functions are included in this file for the development and operations of on the database.
1. `retrieve` this function is used to retrieves the TODO note from the database.
2. `add` this function uses adds a TODO list in the database.
3. `tide` this function implements the tide framework and executes the schema of the database allowing for its creation using the `#[shuttle_service::main]` attribute.

