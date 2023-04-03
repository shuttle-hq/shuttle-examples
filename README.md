# Examples

Some example apps to show what you can do with shuttle.

## How to deploy the examples

To deploy the examples, check out the repository locally

```bash
git clone https://github.com/shuttle-hq/examples.git
```

navigate to an example root folder

```bash
cd axum/hello-world
```

Pick a project name that is something unique - in shuttle,
projects are globally unique. Then run

```bash
cargo shuttle project start --name=$PROJECT_NAME
cargo shuttle deploy --name=$PROJECT_NAME
```
