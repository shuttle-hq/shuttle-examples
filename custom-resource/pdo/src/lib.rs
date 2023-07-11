use async_trait::async_trait;
use serde::Serialize;
use shuttle_service::Factory;
use shuttle_service::ResourceBuilder;
use shuttle_service::Type;

#[derive(Serialize)]
pub struct Builder {
    name: String,
}

#[derive(Clone)]
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
impl ResourceBuilder<Pdo> for Builder {
    const TYPE: Type = Type::Custom;

    type Config = Self;

    type Output = String;

    fn new() -> Self {
        Self {
            name: String::new(),
        }
    }

    fn config(&self) -> &Self::Config {
        self
    }

    async fn output(
        self,
        _factory: &mut dyn Factory,
    ) -> Result<Self::Output, shuttle_service::Error> {
        Ok(self.name)
    }

    async fn build(build_data: &Self::Output) -> Result<Pdo, shuttle_service::Error> {
        Ok(Pdo {
            name: build_data.clone(),
        })
    }
}
