<<<<<<< HEAD
# Hello World application using Tower
This example develops a hello world application using the [tower](https://docs.rs/tower/latest/tower//) framework.
The example is to present a network between client and server using tower

# Run it
Run it locally: `cargo shuttle run`
Curl it to test it: `curl localhost:8000/hello`

=======
# Hello World app with tower
This example develops a hello world application using the [tower](https://docs.rs/tower/latest/tower//) framework.
The example is to present a network between client and server using tower

## Structure
This example has only one file to launch the project the rest of the project.

### src/lib.rs
The main part of this example is to launch the hello world application using the `#[shuttle_service::main]` attribute.

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
>>>>>>> origin/update#1


