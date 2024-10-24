# Shuttle Examples

This is a collection of some example apps that show what you can deploy on Shuttle.

The examples in this repository, consists of *"Hello, world!"* examples of all officially supported web frameworks and how to combine them with Shuttle resources, but also fullstack templates and more.

## How to clone, run and deploy an example

To clone an example, use the `init` command of the [`cargo-shuttle`](https://docs.shuttle.dev/introduction/installation) CLI and specify the git URL and optional subfolder:

```bash
shuttle init --from https://github.com/shuttle-hq/shuttle-examples --subfolder axum/hello-world

### Other forms:

# GitHub prefix. Change to 'gl:' or 'bb:' for GitLab or BitBucket
shuttle init --from gh:username/repository
# Also GitHub
shuttle init --from username/repository

# From local folder
shuttle init --from ./path/to/folder
shuttle init --from ../../another/folder
shuttle init --from /home/user/some/folder

# Clone into 'my-folder', and use the project name 'my-project-name'
shuttle init --from username/repository --name my-project-name my-folder
```

Then, you can navigate into the folder where it was created, and use these commands to run the example locally, and to deploy it.

```bash
# Run locally
shuttle run

# Deploy to Shuttle
shuttle deploy
```
