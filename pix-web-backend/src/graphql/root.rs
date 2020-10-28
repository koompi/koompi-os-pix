use async_graphql::{EmptySubscription, Schema};
use super::{mutations::MutationRoot, queries::QueryRoot};
use crate::models::packages::Packages;
// use futures::{lock::Mutex, stream::StreamExt};
// use slab::Slab;
// use std::sync::Arc;

#[derive(Debug,Clone)]
pub struct DB {
    pub package: Packages,
}
// pub type Storage = Arc<Mutex<Slab<Packages>>>;
pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
