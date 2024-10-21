# Url Shortener

A URL shortener that you can use from your terminal - built with shuttle, rocket and postgres/sqlx.

## How to use it

You can use this URL shortener directly from your terminal. Just copy and paste this command to your terminal and replace `<URL>` with the URL that you want to shorten

```bash
curl -X POST -d '<URL>' https://s.shuttleapp.rs
```

like this

```bash
curl -X POST -d 'https://docs.rs/shuttle-service/latest/shuttle_service/' https://s.shuttleapp.rs
```

you will get the shortened URL back (something like this `https://s.shuttleapp.rs/RvpVU_`)

## Project structure

The project consists of the following files

- `migrations` folder is for DB migration files created by [sqlx-cli](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli)
- `src/main.rs` is where all the magic happens - it creates a shuttle service with two endpoints: one for creating new short URLs and one for handling shortened URLs.
