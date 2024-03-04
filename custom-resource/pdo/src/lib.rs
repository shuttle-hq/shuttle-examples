use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use shuttle_service::{Error, IntoResource, ResourceFactory, ResourceInputBuilder};

#[derive(Default)]
pub struct Builder {
    field: String,
}

impl Builder {
    // Make a setter method for each field in the builder
    pub fn field(mut self, field: &str) -> Self {
        self.field = field.to_string();
        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct InputType {
    name: String,
}

#[async_trait]
impl ResourceInputBuilder for Builder {
    // Read the trait docs for explanations of these types
    type Input = InputType;
    type Output = InputType;

    async fn build(self, _factory: &ResourceFactory) -> Result<Self::Input, Error> {
        // factory can be used to get metadata from Shuttle
        Ok(InputType { name: self.field })
    }
}

pub struct Pdo {
    pub name: String,
}

#[async_trait]
impl IntoResource<Pdo> for InputType {
    async fn into_resource(self) -> Result<Pdo, Error> {
        // connection or setup logic can be handled here
        Ok(Pdo { name: self.name })
    }
}
