<<<<<<< HEAD
# Hello World Application with Salvo
This example shows how to launch a hello world app using the [salvo](https://docs.rs/salvo/latest/salvo/all.html) framework and the `#[shuttle_service::main]` attribute.

# Run it 

Run it locally: `cargo shuttle run`
Curl it to test: `curl localhost:8000/hello`



=======
# First App with salvo

This example shows how to launch a hello world app using the [salvo](https://docs.rs/salvo/latest/salvo/all.html) framework and the `#[shuttle_service::main]` attribute.

## Structure
The folder contains one main file `lib.rs` for creating and launching the application.

### src/lib.rs
This code uses the `shuttle_service` attribute to define the `salvo` framework. Then uses the `#[handler]` attribute to launch the hello world application.

## Deploy
After logging into shuttle, use the following command to deploy this example:

```sh
$ cargo shuttle project new
$ cargo shuttle deploy
```

Now make a note of the `Host` for the deploy to use in the examples below. Or just use `hello-world-salvo-app.shuttleapp.rs` as the host below.
>>>>>>> origin/update#1

