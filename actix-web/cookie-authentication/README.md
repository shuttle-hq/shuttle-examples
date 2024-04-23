# Cookie authentication with actix-session

This template uses the [actix_identity](https://docs.rs/actix-identity) and [actix_session](https://docs.rs/actix-session) crates to manage user sessions.

Running the project and visiting <http://localhost:8000> will show the currently logged in user.
Visiting <http://localhost:8000/login> will log you in as `user1` (no authentication logic is in place).
Visiting <http://localhost:8000/logout> will log you out again.
