# Using Clerk auth with a Rust backend

Clone this template with `cargo shuttle init --from ...`

Sign up at clerk.com, go to the dashboard and create an application with the sign-in options you prefer.

Get the `CLERK_SECRET_KEY` secret and put it in `backend/Secrets.toml`.

The frontend was made with `npm create vite@latest frontend -- --template react-ts` and by following the guide at <https://clerk.com/docs/quickstarts/react>.

Get the `VITE_CLERK_PUBLISHABLE_KEY` secret and put it in `frontend/.env`.

cd into frontend and `npm i; npm run build`. This builds the frontend assets.

cd back to root and `cargo shuttle run`.

TODO:

- Shuttle project start and deploy
- A way to auto refresh the vite build?
