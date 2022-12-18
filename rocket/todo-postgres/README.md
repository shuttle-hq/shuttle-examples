<<<<<<< HEAD
# Hello World Application using Post-Gres
The following example is to present a way to use [rocket](https://rocket.rs/v0.4/guide/introduction/) framework and devlop a relational database for a TODO app using [Postgres](https://www.postgresql.org/docs/).

# Run it 
Run locally: `cargo shuttle run`
Curl it to test: `curl localhost: 8000/hello`
=======
# Todo App with Rocket and postgres
The following example is to present a way to use [rocket](https://rocket.rs/v0.4/guide/introduction/) framework and devlop a relational database for a TODO app using [Postgres](https://www.postgresql.org/docs/).

## Structure 
The example contains one folder `lib.rs` where the database and the CRUD operations are defined. 

### src/lib.rs
Three main functions are included in this file for the development and operations of on the database.
1. `retrieve` this function uses the `get` attribute and is used to retrieves the TODO note from the database
2. `add` this function uses the `post` attribute and adds a TODO list in the database
3. `rocket` this function implements the rocket framework and executes the schema of the database allowing for its creation using the `#[shuttle_service::main]` attribute 
>>>>>>> origin/update#1

