# Shuttle Examples

This is a collection of some example apps that show what you can do with shuttle.

The examples in this repository, listed below, consists of *"Hello, world!"* examples of all officially supported web frameworks and how to combine them with Shuttle resources, but also fullstack templates and more.

There are also unofficial examples that are created and maintained by community members.
Contributions to this list are welcome.
Check out the list at the [bottom of this page](#community-examples).

## How to clone, run and deploy an example

To clone an example, use the init command of the [`cargo-shuttle`](https://docs.shuttle.rs/introduction/installation) binary and specify the git URL and optional subfolder:

```bash
cargo shuttle init --from https://github.com/shuttle-hq/shuttle-examples --subfolder axum/hello-world
```

You can also add `--name my-project-name` and the target path as arguments. Otherwise, they will be prompted for.

The `--from` argument uses [cargo-generate](https://cargo-generate.github.io/cargo-generate/) internally, which means you can also use these forms:

```bash
# GitHub prefix. Change to 'gl:' or 'bb:' for GitLab or BitBucket
cargo shuttle init --from gh:username/repository
# Also GitHub
cargo shuttle init --from username/repository

# From local folder
cargo shuttle init --from ../path/to/folder
cargo shuttle init --from /home/user/some/folder
```

Then, you can navigate into the folder where it was created, and use these commands to run the example locally, and to deploy it.

```bash
# Run locally
cargo shuttle run

# Start the Shuttle environment, make sure the project has a unique name
cargo shuttle project start
# Deploy to Shuttle
cargo shuttle deploy
```

## Official Examples

This is a list of all examples maintained in this repostiory.

**Framework** | **Name & Link** | **Description** | **Command**
--------------|-----------------|-------------|----------------
Actix Web | [hello-world](./actix-web/hello-world/) | Hello World | `cargo shuttle init --template actix-web`
Actix Web | [postgres](./actix-web/postgres/) | TODO app with a Postgres DB | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder actix-web/postgres`
Actix Web | [websocket-actorless](./actix-web/websocket-actorless/) | Websocket app that checks the status of Shuttle's API | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder actix-web/websocket-actorless`
Axum | [hello-world](./axum/hello-world/) | Hello World | `cargo shuttle init --template axum`
Axum | [static-files](./axum/static-files/) | Hello World page that serves static HTML and JS files | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder axum/static-files`
Axum | [static-next-server](./axum/static-next-server/) | SPA server for serving a apps from frameworks such as Next.js | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder axum/static-next-server`
Axum | [websocket](./axum/websocket/) | Websocket app that checks the status of Shuttle's API | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder axum/websocket`
Axum | [with-state](./axum/with-state/) | Hello World with example of how to utilize State in Axum | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder axum/with-state`
Axum & Next.js | [saas](./fullstack-templates/saas/) | Competent opinionated fullstack web app with pre-made routes and assets | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder fullstack-templates/saas`
Poem | [hello-world](./poem/hello-world/) | Hello World | `cargo shuttle init --template poem`
Poem | [mongodb](./poem/mongodb/) | TODO app with MongoDB | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder poem/mongodb`
Poem | [postgres](./poem/postgres/) | TODO app with a Postgres DB | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder poem/postgres`
Poise | [hello-world](./poise/hello-world/) | Hello World Discord bot | `cargo shuttle init --template poise`
Rocket | [authentication](./rocket/authentication/) | Use JWT to authenticate API endpoints | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/authentication`
Rocket | [dyn-templates](./rocket/dyn-templates/) | Handlebars dynamic templates | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/dyn-templates`
Rocket | [hello-world](./rocket/hello-world/) | Hello World | `cargo shuttle init --template rocket`
Rocket | [persist](./rocket/persist/) | Store weather data with Shuttle Persist | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/persist`
Rocket | [postgres](./rocket/postgres/) | TODO app with a Postgres DB | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/postgres`
Rocket | [secrets](./rocket/secrets/) | Extract secrets from your Secrets.toml file | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/secrets`
Rocket | [url-shortener](./rocket/url-shortener/) | URL shortener with Postgres storage | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/url-shortener`
Rocket | [workspace](./rocket/workspace/) | A cargo workspace where one crate is a Shuttle service | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/workspace`
Salvo | [hello-world](./salvo/hello-world/) | Hello World | `cargo shuttle init --template salvo`
Serenity | [hello-world](./serenity/hello-world/) | Hello World Discord bot | `cargo shuttle init --template serenity`
Thruster | [hello-world](./thruster/hello-world/) | Hello World | `cargo shuttle init --template thruster`
Thruster | [postgres](./thruster/postgres/) | TODO app with a Postgres DB | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder thruster/postgres`
Tide | [hello-world](./tide/hello-world/) | Hello World | `cargo shuttle init --template tide`
Tide | [postgres](./tide/postgres/) | TODO app with a Postgres DB | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder tide/postgres`
Tower | [hello-world](./tower/hello-world/) | Hello World | `cargo shuttle init --template tower`
Warp | [hello-world](./warp/hello-world/) | Hello World | `cargo shuttle init --template warp`
*Custom Service* | [none](./custom/none/) | No framework - Custom empty service - A barebones minimal Shuttle app that does nothing | `cargo shuttle init --template none`
*Custom Service* | [request-scheduler](./custom/request-scheduler/) | A custom *Request Scheduler* service | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder request-scheduler`

## Community Examples

This is a collection of unofficial examples that community members have submitted.
Quality and maintenance is not guaranteed.
You can submit a Pull Request to add your example to the list.

<!--
############ TO ADD TO THE LIST ############

1. Your example should be in a public repository.
2. If it has a license, it should allow users to copy and modify the code.
3. Keep the list sorted by Framework name, then Example name.

### You can copy this line and fill in the placeholders:

FRAMEWORK | [NAME](LINK_TO_REPO) | DESCRIPTION | `cargo shuttle init --from USER/REPO --subfolder PATH`

- If the example is in the root of the repo, you can remove the --subfolder argument.
- Add `gl:` or `bb:` prefix for examples hosted on GitLab or BitBucket (see examples at the top).

############################################
-->

**Framework** | **Name & Link** | **Description** | **Command**
--------------|-----------------|-------------|----------------
Axum | [Test](https://github.com/shuttle-hq/shuttle-examples) | A test example A test example A test example | `cargo shuttle init --from <user>/<repo> --subfolder <path>`
Axum/Any | [Fullstack Rust](https://github.com/TylerBloom/shuttle-fullstack-rust-example) | A basic project template for fullstack Rust projects | `cargo shuttle init --from TylerBloom/shuttle-fullstack-rust-example`
