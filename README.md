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
# GitHub
cargo shuttle init --from gh:username/repository --subfolder path/to/example
# GitLab
cargo shuttle init --from gl:username/repository --subfolder path/to/example
# BitBucket
cargo shuttle init --from bb:username/repository --subfolder path/to/example
# Also GitHub
cargo shuttle init --from username/repository --subfolder path/to/example

# When the example is stored in the root (clone the entire repo)
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

Actix Web

- [hello-world](./actix-web/hello-world/) - Hello World

Axum

- [hello-world](./axum/hello-world/) - Hello World

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
Axum | [Test](https://somewhe.re) | `cargo shuttle init --from <user>/<repo> --subfolder <path>` | A test example
