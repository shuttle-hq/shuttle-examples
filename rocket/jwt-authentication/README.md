# Rocket with JWT authentication

## Introduction

This example shows how to use [Rocket request guards](https://rocket.rs/v0.5-rc/guide/requests/#request-guards) for authentication with [JSON Web Tokens](https://jwt.io/) (JWT for short).
The idea is that all requests authenticate first at <https://authentication-rocket-app.shuttleapp.rs/login> to get a JWT.
Then the JWT is sent with all requests requiring authentication using the HTTP header `Authorization: Bearer <token>`.

This example uses the [`jsonwebtoken`](https://github.com/Keats/jsonwebtoken) which supports symmetric and asymmetric secret encoding, built-in validations, and most JWT algorithms.
However, this example only makes use of symmetric encoding and validation on the expiration claim.

## Features

- A route that can be called without needing any authentication.
- A route for posting a JSON object with a username and password to get a JWT.
- A route that can only be accessed with a valid JWT.

## Pre-requisites

- Rust
- [cargo-shuttle](https://www.shuttle.dev)

## How to use this template

Run the project with `shuttle run`.

First, we should be able to access the public endpoint without any authentication using:

```sh
curl http://localhost:8000/public
```

But trying to access the private endpoint will fail with a 403 forbidden:

```sh
curl https://localhost:8000/private
```

So let's get a JWT from the login route first:

```sh
curl --request POST --data '{"username": "username", "password": "password"}' https://localhost:8000/login
```

Accessing the private endpoint with the token will now succeed:

```sh
curl --header "Authorization: Bearer <token>" https://localhost:8000/private
```

The token is set to expire in 5 minutus, so wait a while and try to access the private endpoint again. Once the token has expired, a user will need to get a new token from login.
Since tokens usually have a longer than 5 minutes expiration time, we can create a `/refresh` endpoint that takes an active token and returns a new token with a refreshed expiration time.

Once you're ready to deploy, use `shuttle deploy`.

## Troubleshooting
- Shuttle connects by default to port 8000 - if you're currently already using something at port 8000, you can add
  the `--port <port-number>` to the `shuttle run` command to change this.
