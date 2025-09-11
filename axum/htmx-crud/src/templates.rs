use crate::models;
use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct HelloTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "stream.html")]
pub struct StreamTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "todos.html")]
pub struct Records {
    pub todos: Vec<models::Todo>,
}

#[derive(Template, WebTemplate)]
#[template(path = "todo.html")]
pub struct TodoNewTemplate {
    pub todo: models::Todo,
}
