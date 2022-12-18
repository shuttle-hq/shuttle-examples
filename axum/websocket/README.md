# Web Sockets with AXUM 
This example shows how to use axum to develop a web socket [axum](https://docs.rs/axum/0.1.1/axum/ws/index.html).
The idea is that the `lib.rs` file handles the client and user and http requests and opens the web sockets.

<<<<<<< HEAD
# Run it
Run it locally: `cargo shuttle run`
Curl it to test it: `curl localhost:8000/hello`

=======
## Structure
This example has only 1 file register to handle the http communication between server and client.

### src/lib.rs
Two main functions are in this file:
1.  `async fn websocket_handler` this function creates the websocket and sends and receives data from them.
2. `async fn main()` the main function that spawns a thread to check the status of the API.

## Deploy 

## Deploy
After logging into shuttle, use the following command to deploy this example:

```sh
$ cargo shuttle project new
$ cargo shuttle deploy
```

## Seeing it in action

```sh
$ cargo run
```
>>>>>>> origin/update#1



