# Code Snippet Share API

A lightweight API for sharing code snippets, built with **Rust**, **Axum**, and **Shuttle**. This hands-on tutorial demonstrates how to build, deploy, and manage a snippet-sharing service in the cloud.

---

## Features

- Create, list, view, and delete code snippets.
- Public and private snippet support.
- Tracks view count for each snippet.
- Filter snippets by programming language.
- Lightweight in-memory storage (ideal for tutorials and demos).

---

## Tech Stack

- **Rust** - Fast, safe, and expressive.
- **Axum** - Web framework for building async HTTP APIs.
- **Shuttle** - Rust serverless hosting platform.

---

## Getting Started

### Prerequisites

- Rust (stable)
- Cargo
- Shuttle CLI: [Install Shuttle](https://docs.shuttle.dev/getting-started/installation)

### Run Locally

```bash
shuttle run
```

The API will start on `http://127.0.0.1:8000`

### Deploy to Shuttle

```bash
shuttle deploy
```

Your service will be live with a public URL provided by Shuttle.

### Project Structure

```bash
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    └── main.rs
```
