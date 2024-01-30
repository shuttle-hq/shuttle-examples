# Using Clerk auth with a Rust Actix web backend

Clone this template with `cargo shuttle init --from ...`

Sign up at clerk.com, go to the dashboard and create an application with the sign-in options you prefer.

Get the `CLERK_SECRET_KEY` secret and put it in `backend/Secrets.toml`.
Make sure you don't commit this file.

Get the `VITE_CLERK_PUBLISHABLE_KEY` secret and put it in `frontend/.env`.
The key placed there is public, so it does not have to be gitignored.

> The frontend was initialized from the React+TS template in `npm create vite@latest`.
> The clerk components were then added by following the guide at <https://clerk.com/docs/quickstarts/react>.

cd into the frontend and run `npm install` and `npm run build`. This builds the frontend assets and places them in `frontend/dist`.

cd back to the root and run `cargo shuttle run`.
The backend serves the web page from the dist folder, and an auth-protected API that fetches the list of all users that have signed up so far.

You can then do `cargo shuttle project start` and `cargo shuttle deploy`, but consider switching to a production key from clerk.
The development key can still be used in `Secrets.dev.toml`.

TODO: A way to auto refresh the vite build and rust build at the same time?
