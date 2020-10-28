use async_graphql::Object;
use serde_derive::{Serialize, Deserialize};
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    pub description: String,
    pub maintainer: String,
    pub pgp_key: String,
    pub build_date: String,
}

impl Application {
    pub fn new() -> Self {
        Self::default()
    }
}

#[Object]
impl Application {
    async fn name(&self) -> &str {
        &self.name
    }
    async fn description(&self) -> &str {
        &self.description
    }
    async fn maintainer(&self) -> &str {
        &self.maintainer
    }
    async fn pgp_key(&self) -> &str {
        &self.pgp_key
    }
    async fn build_date(&self) -> &str {
        &self.build_date
    }
}