use async_graphql::{EmptySubscription, Schema};
use super::{mutations::MutationRoot, queries::QueryRoot};
use crate::models::packages::Packages;

use serde_derive::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub token: String
}
#[derive(Debug,Clone)]
pub struct DB {
    pub package: Packages,
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
