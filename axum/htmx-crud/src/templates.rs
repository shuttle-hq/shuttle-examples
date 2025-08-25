use crate::models;
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate;

#[derive(Template)]
#[template(path = "stream.html")]
pub struct StreamTemplate;

#[derive(Template)]
#[template(path = "todos.html")]
pub struct Records {
    pub todos: Vec<models::Todo>,
}

#[derive(Template)]
#[template(path = "todo.html")]
pub struct TodoNewTemplate {
    pub todo: models::Todo,
}

impl IntoResponse for HelloTemplate {
    fn into_response(self) -> Response {
        let rendered = self.render().unwrap_or_default();
        Html(rendered).into_response()
    }
}

impl IntoResponse for StreamTemplate {
    fn into_response(self) -> Response {
        let rendered = self.render().unwrap_or_default();
        Html(rendered).into_response()
    }
}

impl IntoResponse for Records {
    fn into_response(self) -> Response {
        let rendered = self.render().unwrap_or_default();
        Html(rendered).into_response()
    }
}

impl IntoResponse for TodoNewTemplate {
    fn into_response(self) -> Response {
        let rendered = self.render().unwrap_or_default();
        Html(rendered).into_response()
    }
}
