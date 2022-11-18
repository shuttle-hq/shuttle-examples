# Hello World Application using Axum
This example shows how to build a hello world application using the [Axum](https://docs.rs/axum/latest/axum/) web framework.


## Structure
This example has only one file to launch the application.

### src/lib.rs
This file contains the hello world application of axum that uses the `#[shuttle_service::main]` attribute in the `async fn axum() ` function.


## Deploy

After logging into shuttle, use the following command to deploy this example:

```sh
$ cargo shuttle project new
$ cargo shuttle deploy --name=$PROJECT_NAME
```



