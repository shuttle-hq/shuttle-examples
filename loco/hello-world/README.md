# Shuttle Loco template :train:

Welcome to the [Loco](https://loco.rs) Hello World Shuttle Template, designed with a focus on controllers and views (response schema). This minimalistic template comes with several features to kickstart your project:

## REST API Service without a Database

* **Controllers:** Handle web requests parameters, body, validation, and render a response that is content-aware. We use Axum for the best performance, simplicity, and extensibility. Controllers also allow you to easily build middlewares, which can be used to add logic such as authentication, logging, or error handling before passing requests to the main controller actions.
* **Views:** Loco can integrate with templating engines to generate dynamic HTML content from templates.
* **Background Jobs:** Perform compute or I/O intensive jobs in the background with a Redis backed queue, or with threads. Implementing a worker is as simple as implementing a perform function for the Worker trait.
* **Scheduler:** Simplifies the traditional, often cumbersome crontab system, making it easier and more elegant to schedule tasks or shell scripts.
* **Mailers:** A mailer will deliver emails in the background using the existing loco background worker infrastructure. It will all be seamless for you.
* **Storage:** In Loco Storage, we facilitate working with files through multiple operations. Storage can be in-memory, on disk, or use cloud services such as AWS S3, GCP, and Azure.
* **Cache:** Loco provides an cache layer to improve application performance by storing frequently accessed data.

So see more Loco features, check out our [documentation website](https://loco.rs/docs/getting-started/tour/).

## Quick Start

To launch your app, simply use the following command:

```sh
shuttle run
```
