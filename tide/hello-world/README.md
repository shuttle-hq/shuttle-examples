# First App with tide

This example shows how to launch a hello world app using the [tide](https://docs.rs/tide/latest/tide/) framework and the `#[shuttle_service::main]` attribute.

## Structure
The folder contains one main file `lib.rs` for creating and launching the application.

### src/lib.rs
This code uses the `#[shuttle_service::main]` attribute to define the `tide` framework and launch the application.

## Deploy
After logging into shuttle, use the following command to deploy this example:

```sh
$ cargo shuttle project new
$ cargo shuttle deploy
```

Now make a note of the `Host` for the deploy to use in the examples below. Or just use `hello-world-tide-app.shuttleapp.rs` as the host below.