# Hello World app with poem

This example develops a Todo application using the [poem framework](https://docs.rs/poem/latest/poem/).
The idea is to present a web app developed with the poem framework.

## Structure
This example has only one file to launch the project the rest of the project includes a [MongoDB](https://github.com/shuttle-hq/examples/tree/main/poem/mongodb) and [Postgres](https://github.com/shuttle-hq/examples/tree/main/poem/postgres).

### src/lib.rs
The main part of this example is to launch the hello world application using the `#[shuttle_service::main]` and your hello world app. 

## Deploy 

After logging into shuttle, use the following command to deploy this example:

```sh
$ cargo shuttle project new
$ cargo shuttle deploy
```

## Seeing it in action
We should be able to type in the following command to run the app.
```sh
$ cargo run
```


