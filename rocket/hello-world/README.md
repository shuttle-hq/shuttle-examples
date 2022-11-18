# First App with Rocket

This example shows how to launch a hello world app using the [rocket](https://rocket.rs/v0.4/guide/introduction/) framework and the `#[shuttle_service::main]` attribute.

## Structure
The folder contains one main file `lib.rs` for creating and launching the application.

### src/lib.rs
This code uses the `macro_use` attribute to define the `rocket` framework. Then uses the `#[shuttle_service::main]` to launch the hello world application.
## Deploy

## Deploy
After logging into shuttle, use the following command to deploy this example:

```sh
$ cargo shuttle project new
$ cargo shuttle deploy
```

Now make a note of the `Host` for the deploy to use in the examples below. Or just use `hello-world-rocket-app.shuttleapp.rs` as the host below.

