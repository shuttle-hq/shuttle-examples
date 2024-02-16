use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Debug)]
pub enum MutationKind {
    Create,
    Delete,
}

#[derive(Clone, Serialize, Debug)]
pub struct TodoUpdate {
    pub mutation_kind: MutationKind,
    pub id: i32,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct TodoNew {
    pub description: String,
}
