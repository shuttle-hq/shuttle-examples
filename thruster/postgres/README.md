# Todo App with thruster and postgres

The following example is to present a way to use [thruster](https://docs.rs/thruster/latest/thruster/) framework and devlop a relational database for a TODO app using [Postgres](https://www.postgresql.org/docs/).

## Structure 
The example contains one folder `lib.rs` where the database and the CRUD operations are defined. 

### src/lib.rs
Three main functions are included in this file for the development and operations of on the database.
1. `retrieve` this function uses the `middle_ware` attribute and is used to retrieves the TODO note from the database
2. `add` this function uses `middle_ware` attribute and adds a TODO list in the database
3. `thruster` this function implements the thruster framework and executes the schema of the database allowing for its creation using the `#[shuttle_service::main]` attribute 
