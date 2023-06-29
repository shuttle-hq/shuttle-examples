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

**Framework** | **Name & Link** | **Command** | **Description**
--------------|-----------------|-------------|----------------
Actix Web | [hello-world](./actix-web/hello-world/) | `cargo shuttle init --template actix-web` | Hello World
Actix Web | [postgres](./actix-web/postgres/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder actix-web/postgres` | TODO app with a Postgres DB
Actix Web | [websocket-actorless](./actix-web/websocket-actorless/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder actix-web/websocket-actorless` | Websocket app that checks the status of Shuttle's API
|||
Axum | [hello-world](./axum/hello-world/) | `cargo shuttle init --template axum` | Hello World
Axum | [static-files](./axum/static-files/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder axum/static-files` | Hello World page that serves static HTML and JS files
Axum | [static-next-server](./axum/static-next-server/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder axum/static-next-server` | SPA server for serving a apps from frameworks such as Next.js
Axum | [websocket](./axum/websocket/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder axum/websocket` | Websocket app that checks the status of Shuttle's API
Axum | [with-state](./axum/with-state/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder axum/with-state` | Hello World with example of how to utilize State in Axum
|||
Axum & Next.js | [saas](./fullstack-templates/saas/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder fullstack-templates/saas` | Competent opinionated fullstack web app with pre-made routes and assets
|||
Poem | [hello-world](./poem/hello-world/) | `cargo shuttle init --template poem` | Hello World
Poem | [mongodb](./poem/mongodb/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder poem/mongodb` | TODO app with MongoDB
Poem | [postgres](./poem/postgres/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder poem/postgres` | TODO app with a Postgres DB
|||
Poise | [hello-world](./poise/hello-world/) | `cargo shuttle init --template poise` | Hello World Discord bot
|||
Rocket | [authentication](./rocket/authentication/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/authentication` | Use JWT to authenticate API endpoints
Rocket | [dyn_template_hbs](./rocket/dyn_template_hbs/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/dyn_template_hbs` | Handlebars dynamic templates
Rocket | [hello-world](./rocket/hello-world/) | `cargo shuttle init --template rocket` | Hello World
Rocket | [persist](./rocket/persist/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/persist` | Store weather data with Shuttle Persist
Rocket | [postgres](./rocket/postgres/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/postgres` | TODO app with a Postgres DB
Rocket | [secrets](./rocket/secrets/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/secrets` | Extract secrets from your Secrets.toml file
Rocket | [url-shortener](./rocket/url-shortener/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/url-shortener` | URL shortener with Postgres storage
Rocket | [workspace](./rocket/workspace/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder rocket/workspace` | A cargo workspace where one crate is a Shuttle service
|||
Salvo | [hello-world](./salvo/hello-world/) | `cargo shuttle init --template salvo` | Hello World
|||
Serenity | [hello-world](./serenity/hello-world/) | `cargo shuttle init --template serenity` | Hello World Discord bot
|||
Thruster | [hello-world](./thruster/hello-world/) | `cargo shuttle init --template thruster` | Hello World
Thruster | [postgres](./thruster/postgres/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder thruster/postgres` | TODO app with a Postgres DB
|||
Tide | [hello-world](./tide/hello-world/) | `cargo shuttle init --template tide` | Hello World
Tide | [postgres](./tide/postgres/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder tide/postgres` | TODO app with a Postgres DB
|||
Tower | [hello-world](./tower/hello-world/) | `cargo shuttle init --template tower` | Hello World
|||
Warp | [hello-world](./warp/hello-world/) | `cargo shuttle init --template warp` | Hello World
|||
*Custom Service* | [none](./custom/none/) | `cargo shuttle init --template none` | No framework - Custom empty service - A barebones minimal Shuttle app that does nothing
*Custom Service* | [request-scheduler](./custom/request-scheduler/) | `cargo shuttle init --from shuttle-hq/shuttle-examples --subfolder request-scheduler` | A custom *Request Scheduler* service

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

<FRAMEWORK> | [<NAME>](<LINK_TO_REPO>) | `cargo shuttle init --from <USER>/<REPO> --subfolder <PATH>` | <DESCRIPTION>

- If the example is in the root of the repo, you can remove the --subfolder argument.
- Add `gl:` or `bb:` prefix for examples hosted on GitLab or BitBucket (see examples at the top).

############################################
-->

**Framework** | **Name & Link** | **Command** | **Description**
--------------|-----------------|-------------|----------------
Axum | [Test](https://github.com/user/repo) | `cargo shuttle init --from <user>/<repo> --subfolder <path>` | A test example A test example A test example
