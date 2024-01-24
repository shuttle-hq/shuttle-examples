use async_trait::async_trait;
use serde::Serialize;
use shuttle_service::{resource::Type, Error, Factory, IntoResource, ResourceBuilder};

#[derive(Default, Serialize)]
pub struct Builder {
    name: String,
}

pub struct Pdo {
    pub name: String,
}

impl Builder {
    /// Name to give resource
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

#[async_trait]
impl ResourceBuilder for Builder {
    const TYPE: Type = Type::Custom;
    type Config = Self;
    type Output = String;

    fn config(&self) -> &Self::Config {
        self
    }

    async fn output(self, _factory: &mut dyn Factory) -> Result<Self::Output, Error> {
        // factory can be used to get resources from Shuttle
        Ok(self.name)
    }
}

#[async_trait]
impl IntoResource<Pdo> for String {
    async fn into_resource(self) -> Result<Pdo, Error> {
        Ok(Pdo { name: self })
    }
}
