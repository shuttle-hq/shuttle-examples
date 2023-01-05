# Hello-World application using Actix
This example shows you how to create a hello world application with shuttle and [Actix Web](https://docs.rs/actix-web/latest/actix_web/index.html) using the [App](https://docs.rs/actix-web/latest/actix_web/struct.App.html) struct. However, shuttle needs to move the users configuration across threads to start the server on our backend, and the `App` struct is `!Send` and `!Sync`.
That means that for shuttle to support Actix Web, we need to use the [ServiceConfig](https://docs.rs/actix-web/latest/actix_web/web/struct.ServiceConfig.html) struct. You should be able to configure your application like you normally would, but some steps may be a bit different. If you do you find something that you would expect to be possible not working, please reach out and let us know.

## Run it
We can start our application using the shuttle CLI.
Run it locally: `cargo shuttle run`
Curl it to test it: `curl localhost: 8000/hello`



