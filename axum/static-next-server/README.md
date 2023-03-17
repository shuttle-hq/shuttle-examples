# Serving a NextJS project with Axum

This example is a template for an [axum](https://github.com/tokio-rs/axum) server that serves a NextJS single-page application as static assets, deployed with shuttle. For more information on how to achieve this, check out this article by one of our contributors:  https://joshmo.hashnode.dev/deploying-a-nextjs-front-end-with-a-rust-api-in-one-go.

## Note

This project misses the `static` NextJS bundle because it's common to not commit it to GitHub. This is done through a `.gitignore`, but Shuttle uses a `.ignore` file which waves the `.gitignore` rule. That's because otherwise Shuttle wouldn't package the `static` directory to be able to deploy it, because of the `.gitignore` filtering.

As a result, building this project or trying to run it locally will fail because of the missing `static` folder.
