# Todo App

A full-stack todo application built with Rust and Axum, deployed on Shuttle.

## Features

- Create, read, update, and delete todos
- PostgreSQL database with SQLx
- Static frontend with HTML/CSS/JavaScript
- CORS enabled API

## API Endpoints

- `GET /api/todos` - Get all todos
- `POST /api/todos` - Create a new todo
- `GET /api/todos/{id}` - Get a specific todo
- `PUT /api/todos/{id}` - Update a todo
- `DELETE /api/todos/{id}` - Delete a todo

## Running

```bash
shuttle run
```

## Deployment

```bash
shuttle deploy
```
