<<<<<<< HEAD
# Hello World application using warp
This example develops a hello world application using the [warp](https://docs.rs/warp/latest/warp/) framework.
The example is to present an app that uses the warp web framework.

# Run it
Run it locally: `cargo shuttle run`
Curl it to test: `curl localhost:8000/hello`

=======
# Hello World app with warp
This example develops a hello world application using the [warp](https://docs.rs/warp/latest/warp/) framework.
The example is to present an app that uses the warp web framework.

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


