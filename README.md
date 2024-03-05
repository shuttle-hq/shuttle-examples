# Shuttle Examples

This is a collection of some example apps that show what you can deploy on Shuttle.

The examples in this repository, consists of *"Hello, world!"* examples of all officially supported web frameworks and how to combine them with Shuttle resources, but also fullstack templates and more.

## How to clone, run and deploy an example

To clone an example, use the `init` command of the [`cargo-shuttle`](https://docs.shuttle.rs/introduction/installation) binary and specify the git URL and optional subfolder:

```bash
cargo shuttle init --from https://github.com/shuttle-hq/shuttle-examples --subfolder axum/hello-world

### Other forms:

# GitHub prefix. Change to 'gl:' or 'bb:' for GitLab or BitBucket
cargo shuttle init --from gh:username/repository
# Also GitHub
cargo shuttle init --from username/repository

# From local folder
cargo shuttle init --from ./path/to/folder
cargo shuttle init --from ../../another/folder
cargo shuttle init --from /home/user/some/folder

# Clone into 'my-folder', and use the project name 'my-project-name'
cargo shuttle init --from username/repository --name my-project-name my-folder
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
